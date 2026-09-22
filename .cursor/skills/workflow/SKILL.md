---
name: workflow
description: "Gated four-phase SRES work: Understand, Implementation Plan, Execution, Submit. Use when the user names a phase or this skill, or wants a Linear/GitHub issue done in gated phases. Run exactly one phase per invocation, then stop. Do not use to look up Linear usage (linear); this skill calls that one. Do not use to author AGENTS.md (write-agents-docs), mine a conversation (agent-retro), review a standalone diff (code-review), or run crate/CI housekeeping (health-audit)."
---

# Workflow

Four phases with a hard gate: Understand, Implementation Plan, Execution, Submit. Policy, commands, and test taxonomy live in root `AGENTS.md`. Linear project facts live in [../linear/SKILL.md](../linear/SKILL.md). Do not copy either.

## When

Use when any of these is true:

- The user names this skill or a phase (Understand, Implementation Plan, Execution, Submit)
- The user wants a Linear or GitHub issue implemented in gated phases
- The user asks to merge or submit after a draft PR from this workflow

Do not use for:

- Authoring `AGENTS.md` / `//!` (`write-agents-docs`)
- Conversation retro (`agent-retro`)
- A standalone diff review (`code-review`; Execution calls that skill as a step)
- Looking up Linear usage or posting a comment with no phase (`linear`; this skill calls that one when an issue is associated)
- Crate, CI, or skill housekeeping (`health-audit`)
- A one-shot fix when the user did not name this skill or a phase

## Hard gate

Run **exactly one** phase, then stop. Name the next phase; do not start it.

- User-named phase wins. If they named none, the phase is Understand.
- "Continue" / "next" means only the next phase, still one phase.
- If they name two phases or say to run the whole workflow, do the first (or Understand) and push back.
- Same-phase waits only: Understand after questions (resume Understand when the user answers); Submit after CI subscribe (resume Submit on the notification). A resume never starts a different phase.

## Linear

Follow [../linear/SKILL.md](../linear/SKILL.md) when an issue is associated (`SRES-N` or a Linear URL). Fetch it first. Do not restate team, identifiers, or MCP tools.

- Understand and Implementation Plan: post the artifact as a new top-level comment; always also put it in chat.
- No issue: chat only. Do not create an issue.

## Understand

Ask until the task is clear. Do not guess product, scope, or acceptance. After questions, end the turn.

When answers make it clear, write:

```
## Understanding
Goal:
Non-goals:
Constraints:
Acceptance:
```

Review with a fresh subagent (prompt below). If it lists gaps, ask the user those questions, update the artifact, review again with a **new** subagent. Repeat until it reports clear, or 3 rounds. After 3, stop and list remaining gaps.

Post to Linear if associated. **Stop.** Next phase: Implementation Plan. Do not write a plan.

## Implementation Plan

Require a reviewed Understanding in this conversation or on the Linear issue. Else refuse and name Understand.

Write the plan only. No code, no implementation branch, no tests run as work, no PR.

```
## Implementation Plan
Goal: (pointer to Understanding)
Steps:
1. file-level action
Tests:
- exact command (from root Testing Strategy / directory `## Tests`)
Risks:
Stop if:
```

Review with a fresh subagent. Fix the plan for every issue it finds. Re-review with a new subagent until clean, or 3 rounds then stop and list remaining issues.

Post to Linear if associated. **Stop.** Next phase: Execution. Do not execute.

## Execution

Require a reviewed Implementation Plan. Else refuse and name Implementation Plan.

Follow the plan in order.

If a step is wrong:

- **Trivial:** local, obvious, same step, plan intent unchanged. Apply it and note the deviation.
- **Otherwise: stop.** Report the broken step. Do not invent a new plan. Next phase is still Implementation Plan (revise), not a guess in this phase.

After the plan's code is in:

1. Run `cargo nextest run --workspace --locked` (root Commands Test). Fail → no PR. fmt, clippy, and wasm are CI on Submit, not this gate.
2. If the change altered architecture or test strategy, [../write-agents-docs/SKILL.md](../write-agents-docs/SKILL.md) in this same phase, before review.
3. Run [../code-review/SKILL.md](../code-review/SKILL.md). Fix blockers; re-review once as that skill says. Blockers remaining → no PR.
4. Push and open a **draft** PR (`ManagePullRequest` `create_pr`, `draft: true`).
5. If a Linear issue is associated, set `In Review` per [../linear/SKILL.md](../linear/SKILL.md).

**Stop.** Next phase: Submit. Do not mark ready. Do not merge.

## Submit

This phase is merge authorization. Require the draft (or open) PR from Execution. Else refuse.

1. Mark ready: `ManagePullRequest` `update_pr` with `draft: false`.
2. Subscribe to CI on the PR head branch (`subscribe_github_ci`). State that you are waiting for checks, then end the turn. On failure notification: stop, do not merge. On success: continue Submit.
3. Merge. `ManagePullRequest` has no merge action. Submit is the exception to `gh` read-only in [../linear/SKILL.md](../linear/SKILL.md): `gh pr merge <n>` with explicit `--squash`, `--merge`, or `--rebase` matching the repo. No `--auto`. If merge is refused, stop and report.
4. Delete the merged head: `git push origin --delete <branch>`. Never delete `main` (root `AGENTS.md` Cursor Cloud).
5. If a Linear issue is associated and this PR finishes it, set `Done` per [../linear/SKILL.md](../linear/SKILL.md).

**Stop.**

## Review subagent

One Task, `subagent_type` `generalPurpose`, `model` `inherit`, `run_in_background` false. Fresh every round; never resume. Readonly: no file edits, commits, or PRs.

Fill brackets. The prompt is the whole brief.

**Understand**

```
You review an Understanding for gaps. Do not implement.

Read root AGENTS.md. Issue text:
<issue or user request>

Understanding:
<artifact>

Return exactly:
## Verdict
clear | gaps

## Not clear
- question the user must answer
(empty if clear)
```

**Implementation Plan**

```
You review an Implementation Plan. Do not implement.

Read root AGENTS.md and the nearest directory AGENTS.md for every path the plan names.

Understanding:
<artifact>

Plan:
<artifact>

Reject plans that put work in the wrong layer, skip tests the Testing Strategy requires, or execute rather than plan.

Return exactly:
## Verdict
clean | issues

## Issues
- what to fix in the plan
(empty if clean)
```

## Hard rules

- Never chain phases in one turn (except Understand resuming after questions, and Submit resuming after CI).
- Never execute during Understand or Implementation Plan.
- Never mark ready or merge during Execution.
- Guessing scope during Understand is a bug; ask.
- Subagent found a gap → ask the user; do not fill it yourself unless it is a documented repo fact.

## Pointers

- Root `AGENTS.md`: Commands, Testing Strategy, Cursor Cloud (branch delete)
- [../linear/SKILL.md](../linear/SKILL.md) — identifiers, comments, status, MCP
- [../code-review/SKILL.md](../code-review/SKILL.md) — Execution step
- [../write-agents-docs/SKILL.md](../write-agents-docs/SKILL.md) — Execution, only if architecture/test strategy changed
