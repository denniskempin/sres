---
name: code-review
description: "Run an independent SRES code-owner review via a readonly subagent. Use when the user asks to review a PR, diff, plan, or changes; mentions review_guide or code review; or wants a pre-merge review. Do not review inline — launch the subagent and consume its report."
---

# SRES code review

The main agent does not review. It launches one readonly subagent, waits for the report, then acts on it. Checks live in [references/review-guide.md](references/review-guide.md); architecture policy lives in root `AGENTS.md`. Do not restate either here.

## When

Use this skill when any of these is true:

- The user asks to review, code-review, or apply the review guide
- The user names a PR, diff, uncommitted changes, or an implementation **plan** to review
- A plan exists and no code has been written yet (plan mode, or a `.plan.md` / Linear brief)
- You are about to open or update a PR and want an independent owner review

Do not use this skill for Bugbot or security review (those have their own subagents). Do not use it to author `AGENTS.md` (that is `write-agents-docs`). Do not use it to write the plan (that is `implement-hardware` when the work is a Gaps item).

## Procedure

1. Resolve the subject.
   - **Diff (default):** branch changes against the PR base (or `main` if there is no PR). Use **uncommitted changes** only when the user says so or there is no committed work.
   - **Plan:** no diff. Subject is the plan file path (or the plan text in the conversation). Record that path.
   Record the repo path, base (diff only), and any extra instructions from the user.
2. Launch **exactly one** Task subagent, foreground (`run_in_background: false`). Do not resume; always a fresh subagent. Do not review the subject yourself while it runs.
   - `description`: `SRES code review` (diff) or `SRES plan review` (plan)
   - `subagent_type`: `code-reviewer` if that type is in your available subagent types; otherwise `generalPurpose`
   - `model`: `inherit` unless the user named a model
   - Prompt: the matching template below, filled in. Subagents do not see this conversation; the prompt is the whole brief.
3. Read the report. Reject and re-launch once if the report is missing `## Verdict`, mixes blockers with nits, or lacks file:line (or `plan §N`) citations on findings. Do not re-launch to argue with a finding that matches the guide.
4. Consume the report (next section). Do not paste the review guide into the user-facing reply.

## Subagent prompt (diff)

Fill the bracketed fields. Omit `Custom Instructions` unless the user gave review instructions.

```
Read `.cursor/skills/code-review/references/review-guide.md` first and apply every check.
Read the root `AGENTS.md` and the nearest directory `AGENTS.md` for every touched path.

Full Repository Path: <absolute repo path>
Diff: <branch changes | uncommitted changes>
Base: <base branch, usually main>
Custom Instructions: <user instructions, or omit this line>

You are a readonly SRES code-owner reviewer. Do not edit files, do not commit, do not create PRs, do not run formatters or tests unless a check requires inspecting a command's existence.

Review only the stated diff. Cite `path:line`. Map every "Reject if" / "Reject:" row in the guide to a blocker. Everything else is a nit. If a directory `AGENTS.md` is stale after an architectural change, that is a blocker.

Return exactly this structure and nothing after it:

## Verdict
approve | request-changes

## Blockers
- `path:line` — [guide section] finding. Why it fails the check. What to change.

## Nits
- same shape

## Checklist
Copy section 11 of the review guide. Mark each item pass / fail / n/a with a one-line reason.

## Scope
Commit range or uncommitted files you actually reviewed.
```

## Subagent prompt (plan)

Fill the bracketed fields. Apply the review-guide checks that are meaningful without a diff (architecture, layer boundaries, system variants, bus/clock ordering, error handling, typed addresses, test taxonomy, documentation). Skip checklist items that require a code diff (`n/a`).

```
Read `.cursor/skills/code-review/references/review-guide.md` first and apply every check that is applicable without a code diff.
Read the root `AGENTS.md` and the nearest directory `AGENTS.md` for every path the plan would touch.

Full Repository Path: <absolute repo path>
Subject: the implementation plan at <absolute plan path or "the plan in the parent conversation">
Custom Instructions: <user instructions, or omit this line>

You are a readonly SRES code-owner reviewer. Do not edit files, do not commit, do not create PRs, do not run tests or formatters.

Review the plan for technical correctness and fit with this codebase BEFORE implementation. Verify every claim against the actual source and against `docs/index.md` (keyword search; read only the listed files). Specifically check:

1. Hardware: register layout, sequencing, trigger positions, and cycle model against the cited docs. Flag anything wrong or unsupported.
2. Codebase fit: Clock chunking and wrap (see `sres_emulator/src/components/AGENTS.md`), `advance_master_clock` ordering (`main_bus` AGENTS.md), batched device write timestamps vs `draw_scanline`, layer boundaries, typed addresses / `Wrap`, error-handling policy, and whether proposed unit tests are constructible (`MainBusImpl::new`, `Cartridge::with_program`).
3. Test strategy: correct taxonomy and System variant (root). Idle cycle cost must not drift `V:`/`H:` in `krom_*` traces. Goldens that snapshot unimplemented hardware must be regenerated and visually verified, not blindly committed.
4. Missing items the guide or AGENTS.md would require (`//!` headers, leaf `AGENTS.md` Gaps, `Encode/Decode` on new `Clock` fields).

Return exactly this structure and nothing after it:

## Verdict
approve | request-changes

## Blockers
- `path:line` or `plan §N` — [guide section or doc file] finding. Why it is wrong or risky. What to change in the plan.

## Nits
- same shape

## Verified claims
- Bullet list of plan claims you checked and confirmed correct, with file:line evidence.

## Scope
Plan path and files you actually read.
```

`approve` only when there are zero blockers. Nits never change the verdict.

## Consume the report

| Context | Action |
|---------|--------|
| User asked for a review | Present verdict, every blocker, and nits. Do not auto-fix. |
| You authored the **diff** (pre-merge gate) | Fix blockers. Leave nits unless they are one-line and in files you already touch. After blocker fixes, re-launch this skill once. |
| You authored the **plan** | Fold every blocker into the plan before implementing. Leave nits unless they change the design. Re-launch once after the plan edit. |
| Finding contradicts root `AGENTS.md` | Trust `AGENTS.md`. Drop that finding and note the mismatch. |
| Finding matches the guide | Do not argue. Fix or report it. |

Blockers you fix stay in the conversation as evidence; do not hide them from the user.

## Hard rules

- One subagent, not a swarm. The checklist is a single coherent review.
- The subagent is readonly even on the `generalPurpose` fallback. Never give it edit instructions.
- Do not apply the review-guide checks in the parent context. The point is independent verification.
- Do not load [references/review-guide.md](references/review-guide.md) in the parent unless you must adjudicate a parent-vs-subagent contradiction with `AGENTS.md`.
