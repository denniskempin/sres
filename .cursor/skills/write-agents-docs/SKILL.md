---
name: write-agents-docs
description: "Write or refresh AGENTS.md files and //! file headers in the SRES repo. Use after an architectural or test-strategy change, when a directory lacks an AGENTS.md, or when applying agreed AGENTS.md edits. Do not use to run gated Understand/Plan/Execute/Submit (workflow), to draft implementation plans or post on issues, to look up Linear usage (linear), to mine a conversation for what to document (that is agent-retro), or for periodic crate/CI/skill housekeeping (that is health-audit)."
---

# Writing AGENTS.md and `//!` headers

Scope: `AGENTS.md` files and the leading `//!` block of `.rs` files. Not in scope: `///` item docs, code changes, gated feature phases (`.cursor/skills/workflow/`), implementation plans or issue comments, conversation retrospectives (`.cursor/skills/agent-retro/`), `.cursor/skills/` authorship, Linear project facts (`.cursor/skills/linear/`), periodic crate/CI/skill housekeeping (`.cursor/skills/health-audit/`). Policy lives in the root `AGENTS.md`; the code-review skill enforces it. Do not restate it.
>>>>>>> 4dd6e72 (Add a gated four-phase workflow skill.)

## How these files are consumed

Agents load every `AGENTS.md` from the repo root down to the touched file, concatenated root to leaf. The root is loaded on every request in the repo. The deeper file wins on conflict. Some tools truncate the chain silently at 32 KiB. The cost unit is the chain, not the file.

Highest value per token: facts that cannot be inferred from the code in one step (intentional patterns that look wrong, timing quirks, what is unimplemented, the exact filtered test command). Lowest value: anything `ls` or a single `rg` reproduces (struct bodies, per-register handler tables, function inventories).

## Which fact goes where

| Level | Holds | Voice |
|-------|-------|-------|
| Root `AGENTS.md` | Commands, layer diagram, system variants, test taxonomy, error-handling policy, environment gotchas. The only place cross-cutting patterns are explained. | Imperative |
| Directory `AGENTS.md` | What the directory is, what each file owns, non-obvious behaviors, unimplemented hardware, how to run its tests. Names a root pattern and links up; never re-explains it. | Declarative ("`main_bus` runs DMA before the clock tick") |
| `//!` header | 1 to 4 lines: what the file owns and the one thing to know before editing it. | Declarative |

Rules:
- One home per fact. Everywhere else is a pointer. Anti-pattern: repeating the `$2100`/`$4200` register routing table in root `AGENTS.md`, a directory `AGENTS.md`, and the code-review skill's review guide.
- Inferable-content test: if `ls` or one `rg` reproduces it, it does not go in `AGENTS.md`. Anti-pattern: the 30-row register-to-handler table in `sres_emulator/src/components/ppu/AGENTS.md`. Register semantics live in `docs/index.md`.
- A rule that clippy, a test, or `sres_emulator/src/components/mod.rs` already enforces gets one line pointing at the enforcement.
- A leaf never contradicts the root silently. When it must differ, name the exception and the reason: "Trace tests here use `SyncSystem`, not the default `System`, because batched updates shift register-boundary timing."

## Budgets

| Target | Limit |
|--------|-------|
| Directory `AGENTS.md` | 40 to 80 lines; hard cap 100 |
| Root `AGENTS.md` | 150 lines |
| Chain (root + every ancestor + this file) | 24 KiB by `wc -c` |
| `//!` header | 1 to 4 lines |

## Directory `AGENTS.md` template

Fixed order. Omit a section that has nothing to say; never leave it empty.

```markdown
# `path/from/repo/root`
One sentence: what this directory implements.

## Files
| File | Owns |
|------|------|
| `mod.rs` | `TypeName` facade; register decode; `step()` loop |
(every `.rs` in the directory, one row each, main type(s) named, no struct bodies)

## Behaviors & Gotchas
1. Looks-wrong-but-intentional facts first. Each must be verifiable at a file:line.
2. Timing quirks, ordering constraints, latch behavior.

## Hardware Map            (optional)
| Range | Owner |
(address ranges to module; never per-register handler lists; point to `docs/index.md`)

## Integration
- Who calls this, what it calls. 3 to 6 bullets.

## Gaps
- Unimplemented hardware. Must agree with the root "Error Handling & Unimplemented Hardware" section.

## Tests
- Test type, exact filtered command, asset location.
```

Filtered command examples that run today:
- `cargo nextest run -p sres_emulator --test ppu_tests`
- `cargo nextest run -p sres_emulator --lib -E 'test(spc700::)'`

Directories with children add a `## Subdirectories` table (directory, one-line purpose).

## `//!` header template

- Line 1: what the file implements; hardware name if applicable.
- Lines 2 to 4 (optional): the one invariant or entry point that matters (`step()`, `draw_scanline`, `catch_up_to_master_clock`).
- Banned: "Dummy", "TODO", "placeholder", restating the filename, listing every function.

Examples from this repo:

| File | Verdict |
|------|---------|
| `sres_emulator/src/components/cpu/opcode_table.rs` | Good: says what it combines, why each opcode gets a unique function, and that macros do it. |
| `sres_emulator/src/apu/mod.rs` ("Dummy implementation of the audio processing unit.") | Stale: the module is the real APU integration layer. |
| `sres_emulator/src/main_bus/dma.rs` ("Implementation of DMA functionality in the main bus.") | Too thin: restates the filename; should name `DmaController`, the `$420B` trigger, and that HDMA sequencing lives in `hdma.rs`. |

## Hard rules

- Never paste struct, enum, or trait bodies. Name the type and the file. (`sres_emulator/src/apu/AGENTS.md` embeds `pub struct Apu {...}`; it goes stale on any field edit.)
- Every file, type, register, constant, and command named must exist. Verify with `rg` or by running it before writing it.
- Every "not implemented" claim must match the code and the root error-handling section.
- Backticks for all identifiers. `$XXXX` for SNES addresses. Hardware register names (`INIDISP`, `MDMAEN`), not prose descriptions.
- Tables for 3+ items of the same shape; bullets otherwise. Nested bullets at most one level.
- No "simple", "just", "easy", or marketing adjectives. Present tense, active voice.
- Keep existing section headers when editing; the code-review skill's review guide links to them.
- Edit, never regenerate. Open the existing file and change lines. Do not write a fresh file from the template over one that exists.
- Net-negative bias: for every line added to an existing `AGENTS.md`, look for a line to delete (duplicate of parent, inferable, stale). Report before and after line counts.

## Procedure

0. Find what needs work, mechanically. Run from the repo root:
   - Directories with `.rs` files but no `AGENTS.md`:
     `for d in $(find sres_emulator/src sres_emulator/tests sres_egui/src -type d); do ls "$d"/*.rs >/dev/null 2>&1 && [ ! -f "$d/AGENTS.md" ] && echo "$d"; done`
   - `.rs` files without a `//!` header:
     `find sres_emulator/src sres_emulator/tests sres_egui/src -name '*.rs' | while read f; do head -c 3 "$f" | grep -q '^//!' || echo "$f"; done`
   - `AGENTS.md` older than the code it documents (compare the two dates):
     `d=sres_emulator/src/apu; git log -1 --format=%cs -- $d/AGENTS.md; git log -1 --format=%cs -- $d ":!$d/AGENTS.md"`
   - Chain size for a leaf (add each ancestor `AGENTS.md` that exists):
     `wc -c AGENTS.md sres_emulator/src/AGENTS.md sres_emulator/src/apu/AGENTS.md`
1. `ls` the directory. List every `.rs` file.
2. For each file, read the first 40 lines and every `pub` item: `rg -n '^\s*pub' <file>`.
3. Read the parent `AGENTS.md` chain up to the root. Write down which facts are already explained upward. Delete them from the leaf if present. If the change alters a fact the root already states (`## Entry Point Call Chain`, `## Testing Strategy`, `## Error Handling & Unimplemented Hardware`), edit that root line in the same change. Do not leave the leaf as the only correct statement.
4. Edit in template order. For each fact you claim, note the file:line it came from in a scratch file outside the repo. Do not commit the scratch file.
5. Run the self-check.
6. If the change touched a file's role, update its `//!` block in the same commit.

## Fan-out with subagents

Use subagents when step 0 yields more than 3 directories or more than 10 files. Below that, do the work inline. Directories are the unit of work: one subagent per directory, headers for that directory's `.rs` files included in the same unit.

Order matters. Children dedupe against the final text of their parents, so process top-down in waves: all directories at depth N in one parallel batch, then depth N+1. Never run a parent and its child in the same batch. Sibling directories (`components/cpu`, `components/ppu`, `components/spc700`) go in one batch.

Subagents do not see this conversation. Each prompt must be self-contained and include:
- "Read `.cursor/skills/write-agents-docs/SKILL.md` first and follow it exactly."
- The one target directory, its ancestor `AGENTS.md` paths in order, and the current chain size in bytes.
- Whether the target `AGENTS.md` exists (edit) or not (create from the template).
- The `.rs` files in that directory that lack a `//!` header.
- "Edit files only. Do not commit, do not touch files outside `<dir>`, do not write scratch files inside the repo."
- The required report: files changed, before and after line counts, the answered self-check, and any fact you could not verify at a file:line (say so; do not guess).

Coordinator duties after each wave:
1. Read every report. Reject and re-run any unit whose self-check has a "no" or whose report lacks line counts.
2. Cross-file pass the subagent cannot do alone: `wc -c` every chain that changed; `rg` each new fact in the wave against sibling and parent `AGENTS.md` files for duplicates; check every `## Gaps` entry against the root error-handling section. If a leaf change alters a root-stated call-chain, test-taxonomy, or error-policy fact, the coordinator edits that root line (subagents stay inside `<dir>`).
3. Commit one directory per commit before starting the next wave.

## Self-check

Answer each before finishing. Any "no" means go back.

- Every `.rs` in the directory appears in `## Files`?
- No struct, enum, or trait bodies?
- No fact that also appears in a parent `AGENTS.md`?
- Nothing that `ls` or one `rg` reproduces?
- Every identifier, path, and register verified to exist?
- File under its cap and chain under 24 KiB?
- `## Gaps` consistent with the root error-handling section?
- Root `## Entry Point Call Chain` / `## Testing Strategy` / `## Error Handling & Unimplemented Hardware` still true?
- `## Tests` has a command that runs?
- Leaf voice declarative, root voice imperative?
- Every touched `//!` header 1 to 4 lines with no banned words?
- Before and after line counts reported?
