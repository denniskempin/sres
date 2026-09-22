---
name: linear-issue
description: "Work a SRES Linear issue (SRE-# / SRES-#): plan and post, implement the posted plan, review its PR, or submit/merge/close. Use when the user names SRE-N or SRES-N. Pick the step from the ask (default: plan). Do not use to review a diff with no Linear id (code-review), author AGENTS.md (write-agents-docs), retro a conversation (agent-retro), or run housekeeping (health-audit)."
---

# Linear issues

Orchestrate one SRES Linear issue. Policy, commands, tests, and git rules live in root `AGENTS.md`. Diff review is [../code-review/SKILL.md](../code-review/SKILL.md). `AGENTS.md` / `//!` edits are [../write-agents-docs/SKILL.md](../write-agents-docs/SKILL.md). Do not restate those.

The Linear team is **SRES**. Issue identifiers are **`SRE-N`**. Treat `SRES-N` as `SRE-N`.

## When

Use when the user names `SRE-N` or `SRES-N`. This skill is the parent workflow. Call `code-review` for Review and as a pre-merge gate; do not skip it.

Do not use when there is no Linear id (`code-review`), the task is `AGENTS.md` authorship (`write-agents-docs`), a conversation retro (`agent-retro`), or housekeeping (`health-audit`).

## Pick the step

Do **only** the steps the ask names, in this order. Default if the id is the only signal: **Plan**.

| Ask | Step |
|-----|------|
| plan, implementation plan, still exist, validate, investigate | Plan |
| implement, fix, build, do the plan | Implement |
| review the PR, code review (with an issue id) | Review |
| submit, merge, land, ship, close the issue | Submit |

## Shared setup

1. Discover Linear tool schemas (`GetDynamicTools` namespace `Linear`) before the first call. If the namespace is `needsAuth`, authenticate.
2. `get_issue` with `includeRelations` on `SRE-N`. `list_comments` (oldest first). Stop if the issue is `Canceled`.
3. Read the issue, comments, attachments, related issues, and `gitBranchName`. Related issues own their own scope; do not take that work unless the posted plan says so.
4. `git fetch origin main`. Stay on an existing PR branch for this issue if this run already has one; otherwise branch from `origin/main` (root Cursor Cloud instructions). Never commit on `main`.

Linear writes: `save_comment` (new top-level comment; do not reply to the agent-session stub). `save_issue` for status (`In Progress`, `In Review`, `Done`) and `links`. Do not overwrite the issue description. `gh` is read-only; create/update PRs with `ManagePullRequest`.

## 1. Plan

Create a plan and post it on the issue. Do not implement.

1. Set status `In Progress` unless it is already `Done` (then validate only; do not reopen unless the user asked to implement).
2. Check **current `origin/main`**, not a stale attachment branch. Confirm whether the stated bug or gap still exists (code plus a command or focused test). Stale GitHub attachments can 404; ignore them unless `gh` / REST shows the PR still exists.
3. Do not cherry-pick an old `cursor/SRE-N-*` branch without re-validating against `main`.
4. Post one top-level comment with this shape (markdown, literal newlines):

```
## Implementation plan (validated on `main` @ <sha>)

### Does the issue still exist?
<crash/gap/fixed — evidence>

### Recommended scope
<what this issue does; what related issues own>

### Phases
<numbered code changes, files, unimplemented-enum impact>

### Tests
<exact nextest/clippy commands from the nearest AGENTS.md>

### Docs
<AGENTS.md / `//!` if architecture or Gaps change; use write-agents-docs>

### Suggested PR split
<one PR unless the plan needs a hard cut>
```

5. If the issue is fully gone on `main`, say so, set `Done`, and stop. If it is a partial fix, plan only the remainder. Do not invent work the issue does not ask for.

## 2. Implement

Implement the plan **found on the Linear issue**.

1. Take the newest comment whose heading is `## Implementation plan`. If none exists, run Plan first, then continue.
2. If `main` has moved past the plan's sha, re-validate the still-exists section. If the plan is wrong, post an updated plan, then implement that update.
3. Branch from `origin/main` if needed. Prefer Linear `gitBranchName`, else `cursor/sre-N-short-slug`.
4. Implement only the posted scope. Unimplemented hardware stays `on_unimplemented` until the plan removes that variant.
5. Tests and `AGENTS.md` / `//!`: follow root `AGENTS.md` and `write-agents-docs`. Run the commands in the plan.
6. Before opening a PR, run [../code-review/SKILL.md](../code-review/SKILL.md) (you authored the diff). Fix blockers; re-review once.
7. Push. `ManagePullRequest` `create_pr` **draft**. `save_issue` `links` the PR URL, status `In Review`, comment with branch + PR.

Do not merge. Do not delete the branch.

## 3. Review

Review the PR created for the issue. Do not merge, do not implement unless the ask also includes Implement.

1. Resolve the PR: Linear `links` / attachments, comments, `gitBranchName`, then `gh pr list` / REST. A 404 attachment is not a PR.
2. Follow [../code-review/SKILL.md](../code-review/SKILL.md) as **user asked for a review**: present the verdict, every blocker, and nits. Do not auto-fix.
3. Post the verdict and blockers on the Linear issue. Leave status `In Review` on approve; `In Progress` on request-changes.
4. Do not comment on the GitHub PR unless the user asked.

## 4. Submit

Submit is explicit permission to merge this issue's PR, delete its head branch, and close the Linear issue. Never `main`.

1. Refuse unless a GitHub PR exists and Review in this conversation (or a posted Linear review) is `approve` with no open blockers. If Review never ran, run step 3 first and stop on request-changes.
2. `ManagePullRequest` `get_ci_status`. Stop if not green.
3. Mark the PR ready (`draft: false`) if it is still a draft.
4. Merge using a merge action on `ManagePullRequest` if this run has one. Do not use `gh` for writes. If no merge path exists, comment that CI is green and the PR is ready, leave the issue open, and **do not** delete the branch.
5. After merge is confirmed (`mergedAt` set): `git push origin --delete <head>` (root Cursor Cloud instructions).
6. `save_issue` status `Done`. Comment the merged PR URL. Do not use `set_pr_status` closed as a substitute for merge.

## Gotchas

- Identifier is `SRE-N`, not `SRES-N`. The MCP `get_issue` id is `SRE-N`.
- Agent-session threads are stubs. Plans and reviews are new top-level comments.
- `gh` GraphQL `pr view` can 500 when REST works.
- Draft PRs stay draft through Implement. Ready is Submit.
- Closing Linear before merge hides unfinished work. Deleting the branch before merge drops the PR.

## Pointers

- Root `AGENTS.md` — Commands, testing, unimplemented hardware, Cursor Cloud git
- [../code-review/SKILL.md](../code-review/SKILL.md) — Review step and pre-merge gate
- [../write-agents-docs/SKILL.md](../write-agents-docs/SKILL.md) — `AGENTS.md` / `//!` in Implement
- Linear: `get_issue`, `list_comments`, `save_comment`, `save_issue`

## Self-check

Answer each before finishing. Any "no" means go back.

- Linear id normalized to `SRE-N` and the issue fetched?
- Only requested steps ran?
- Plan posted as a top-level comment with the heading `## Implementation plan`?
- Implement followed that comment, not a new invented plan?
- `code-review` used for Review and before opening a PR?
- Submit merged (or stopped cleanly), deleted the head only after merge, set `Done` only after merge?
- `gh` used read-only? Issue description left intact?
