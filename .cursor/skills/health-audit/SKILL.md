---
name: health-audit
description: "Periodic SRES housekeeping: review agent/skill files, crate and toolchain freshness, and CI alignment/speed. Use when the user asks for a health audit, housekeeping, crate updates, CI hygiene, or skill/AGENTS.md alignment across the repo. Do not use to author AGENTS.md after an architecture change (write-agents-docs), to retro a conversation (agent-retro), or to review a feature PR (code-review)."
---

# Health audit

Periodic housekeeping across agent/skill files, crates/toolchain, and CI. Policy, commands, and test taxonomy live in root `AGENTS.md`. `AGENTS.md` / `//!` authorship is [../write-agents-docs/SKILL.md](../write-agents-docs/SKILL.md). Conversation mining is [../agent-retro/SKILL.md](../agent-retro/SKILL.md). Feature diffs are [../code-review/SKILL.md](../code-review/SKILL.md). Do not restate those.

## When

Use when any of these is true:

- The user asks for a health audit, housekeeping, or repo hygiene
- The user asks to review agent or skill files for staleness
- The user asks to check crate or toolchain updates
- The user asks to keep CI aligned and fast

Do not use for:

- Writing `AGENTS.md` after a known architecture or test-strategy change (`write-agents-docs`)
- Conversation retro (`agent-retro`)
- Reviewing a feature PR or diff (`code-review`)
- Implementing hardware or tests

## Procedure

Do the work in this conversation. Fan out only through `write-agents-docs` when that skill's step 0 yields more than 3 stale directories.

### 0. Scope

Default: whole repo on `origin/main`. Record the date, the `rust-toolchain.toml` pin, and the audit window (user-named, else since the last health-audit commit, else the past week). Report-only if the user said review or report only. Otherwise apply after the report.

### 1. Inventory (no edits)

Run from the repo root. Do not edit until step 6.

**Skills and agents**

```bash
ls .cursor/skills/*/SKILL.md .cursor/agents/*.md .github/agents/*.md
for f in .cursor/skills/*/SKILL.md; do
  dir=$(basename "$(dirname "$f")")
  name=$(sed -n 's/^name: //p' "$f" | head -1)
  desc=$(sed -n 's/^description: //p' "$f" | head -1)
  echo "$dir name=$name desc_chars=${#desc}"
done
```

**Docs freshness:** run step 0 of `write-agents-docs` (missing directory `AGENTS.md`, missing `//!`, `AGENTS.md` older than its code, chain `wc -c`).

**Command surfaces:** extract every clippy, nextest, fmt, wasm, trunk, and llvm-cov invocation from `AGENTS.md` (Commands), `check-all.sh`, `fix-all.sh`, `.github/workflows/postsubmit.yml`, and `.cursor/skills/code-review/references/review-guide.md` §10. Fill a table: surface × command. Do not treat today's flags as the spec; treat the files as the spec.

**Toolchain and crates**

```bash
rg '^channel' rust-toolchain.toml
rg '^rust-version' sres_egui/Cargo.toml sres_emulator/Cargo.toml
rustup check || true
cargo update -n
(cd sres_emulator/fuzz && cargo update -n)
```

For each direct dep in `sres_egui/Cargo.toml` and `sres_emulator/Cargo.toml`, `cargo search <name> --limit 1` when `cargo update -n` cannot bump a major/minor. Isolated fuzz lock is a second workspace (`sres_emulator/fuzz/AGENTS.md`).

**CI shape:** `on:`, `concurrency`, each job's `if:`, `uses:`, and `run:` in `.github/workflows/*.yml`. Confirm `.github/rulesets/require-postsubmit.json` `context` equals the aggregator job id in `postsubmit.yml`.

### 2. Agent and skill files

Flag each miss:

| Check | Fail if |
|-------|---------|
| `name` | Does not equal the folder (`[a-z0-9-]`) |
| Description | Missing what, when, or a near-miss `Do not use`; over 1024 characters |
| Discoverability | A skill agents should find from the root has no Reference bullet in root `AGENTS.md` |
| Near-miss | A sibling that shares keywords lacks a `Do not use` pointer |
| Policy | A skill restates root `AGENTS.md` architecture, commands, variants, or error-handling |
| Agents | `.cursor/agents/` or `.github/agents/` file names a tool, MCP, or workflow this repo does not use |
| Links | A markdown link from a skill, review-guide, or `AGENTS.md` Reference does not resolve |
| Budgets | A file exceeds the cap in `write-agents-docs` Budgets |

Stale `AGENTS.md` / `//!` facts go through `write-agents-docs` (template, budgets, self-check). Do not invent a second authorship procedure. A reused multi-step procedure with no skill is a create/improve item here only when it is not conversation-sourced (`agent-retro`).

### 3. Crates and toolchain

Pin a dated nightly in `rust-toolchain.toml` (`nightly-YYYY-MM-DD`), not floating `nightly`. Bump the pin when it is more than a few weeks behind or a dep MSRV requires it. Set crate `rust-version` to the heaviest direct-dep MSRV (recently egui).

Apply, in order:

1. Bump the toolchain pin and `rust-version` if needed.
2. Raise direct deps that `cargo search` shows behind (minor/major). `cargo update` for lockfile patches.
3. Replace unmaintained crates; drop crates that lag the UI stack.
4. Update `sres_emulator/fuzz/Cargo.lock` even when the bins do not compile.
5. Fix compile, API, and new-nightly clippy breakage.
6. If `egui` / `eframe` / `egui_kittest` moved, run `cargo test -p sres_egui` and commit regenerated widget snapshots after inspecting them.
7. Refresh `AGENTS.md` / `//!` for any call-chain or API change (`write-agents-docs`).

Verify after crate or toolchain edits:

```bash
cargo clippy --workspace --all-targets
cargo fmt --check
cargo nextest run --workspace
cargo check --target wasm32-unknown-unknown -p sres_egui
```

Skip `trunk build` unless frontend, WASM deps, or the deploy job changed.

### 4. CI alignment and speed

Compare the step-1 table. A documented exception in root `AGENTS.md` Commands or review-guide §10 is not a finding. Undocumented drift is a finding: document it in `AGENTS.md` Commands (one line) or align the scripts. Do not "fix" a documented exception without a reason.

Required shape:

- `health`, `test`, and `coverage` on pull requests targeting `main` and on push to `main`
- `deploy` stays `main`-only; `pages: write` stays on that job
- Aggregator job id `required` stays stable; the ruleset `context` matches it
- `concurrency.group` is per PR number or ref so a PR run cannot cancel Pages
- `health` compiles WASM (`cargo check --target wasm32-unknown-unknown -p sres_egui`). Native clippy never sees `#[cfg(target_arch = "wasm32")]`
- Rust jobs use `Swatinem/rust-cache` and `cargo nextest`, not `cargo test`
- Isolated workflows (`.github/workflows/copilot-setup-steps.yml`) stay path-filtered

Speed: do not add a workspace-wide job that rebuilds what `test` or `coverage` already builds. Do not run `trunk` on every PR when `health` already has the wasm32 check. Do not add apt packages a crate can replace with a static feature. Bump `uses:` majors when the pinned major is stale; do not churn patch tags.

`fix-all.sh` must remain a local fixer. If its clippy invocation is narrower than CI and that is not documented, document or align.

### 5. Report

Return exactly this structure before applying:

```
## Verdict
clean | work-found

## Agent & skills
- finding. Action.

## Crates & toolchain
- pin / rust-version. Available. Action.

## CI
- surface mismatch or speed issue. Action.

## Apply
report-only | applying
```

`clean` only when every section is empty. `report-only` when the user asked to review or report only.

### 6. Apply

If `Apply` is `report-only`, stop.

Otherwise apply only items listed in the report. One commit per track: (1) agent/skill/docs, (2) toolchain/crates, (3) CI. Split a crate major that needs API or snapshot work into its own PR. Never mix emulator feature work into a health-audit commit.

`AGENTS.md` / `//!`: follow `write-agents-docs`. Skills: keep `name` matching the folder; do not copy `AGENTS.md` policy. Root Reference and sibling `Do not use` lines: update when adding a discoverable skill.

After CI or crate edits, run the verify set in step 3. After skill/docs-only edits, the step-2 table and the `write-agents-docs` self-check are enough.

## Gotchas

- Native `clippy` / `check` does not compile `#[cfg(target_arch = "wasm32")]` bodies. A green health job that lacks the wasm32 line will fail Pages deploy.
- `sres_emulator/fuzz` is not a workspace member. Root `cargo update` does not refresh `sres_emulator/fuzz/Cargo.lock`.
- Fuzz bins are stale and do not compile (root Error Handling). Lock freshness is in scope; rewriting the bins is not, unless the user asks.
- `cargo-outdated` is not in the toolchain. Use `cargo update -n`, `cargo search`, and `rustup check`.
- egui major/minor bumps change `egui_kittest` glyph metrics. Blind snapshot commits fail the review-guide golden check.
- A skill is the wrong home for a single command. `AGENTS.md` is the wrong home for this procedure.

## Pointers

- Root `AGENTS.md` — Commands, Environment Gotchas, Reference
- [../write-agents-docs/SKILL.md](../write-agents-docs/SKILL.md) — `AGENTS.md` / `//!` edits
- [../agent-retro/SKILL.md](../agent-retro/SKILL.md) — conversation-sourced doc proposals
- [../code-review/references/review-guide.md](../code-review/references/review-guide.md) §10 — Tooling & CI
- `.github/workflows/postsubmit.yml`, `.github/rulesets/require-postsubmit.json`, `check-all.sh`, `fix-all.sh`, `rust-toolchain.toml`

## Self-check

Answer each before finishing. Any "no" means go back.

- Inventory ran with no edits?
- Command-surface table built from files, not memory?
- Documented CI exceptions left alone?
- WASM check considered whenever frontend or `#[cfg(wasm32)]` code changed?
- Isolated fuzz lock considered?
- `AGENTS.md` edits went through `write-agents-docs`?
- Commits split by track?
- Report filled, including `Apply`?
