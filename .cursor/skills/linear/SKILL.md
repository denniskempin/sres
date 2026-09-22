---
name: linear
description: "How this project uses Linear: SRES team, SRES-N identifiers, comments, status, and MCP tools. Use when the user mentions Linear, SRES-N, or posting on an issue. Do not use to run Understand, Implementation Plan, Execution, or Submit (workflow); workflow calls this skill. Do not use to review a diff (code-review), author AGENTS.md (write-agents-docs), retro a conversation (agent-retro), or run housekeeping (health-audit)."
---

# Linear

Project facts for talking to Linear. Git, tests, and unimplemented-hardware policy live in root `AGENTS.md`. Do not restate those.

## When

Use when the user mentions Linear, `SRES-N`, posting a comment on an issue, or changing issue status.

Do not use to run Understand, Implementation Plan, Execution, or Submit (`workflow`); that skill calls this one. Do not use to review a PR (`code-review`), write `AGENTS.md` (`write-agents-docs`), retro a conversation (`agent-retro`), or run housekeeping (`health-audit`).

## Project

| Item | Value |
|------|-------|
| Workspace | [linear.app/sres](https://linear.app/sres) |
| Team | **SRES** |
| Issue id | **`SRES-N`** (example: `SRES-5`) |
| Old prefix | `SRE-N` still resolves; normalize to `SRES-N` and write that |

`get_issue` returns `id` as `SRES-N` and `gitBranchName` like `denniskempin/sres-5-…`. Prefer that branch name when creating a work branch.

Statuses: `Backlog`, `Todo`, `In Progress`, `In Review`, `Done`, `Canceled`, `Duplicate`.

## Tools

1. Discover schemas with `GetDynamicTools` namespace `Linear` before the first call. If the namespace is `needsAuth`, authenticate.
2. `gh` is read-only. GitHub PRs use `ManagePullRequest`. Linear attachments can 404; confirm the PR with `gh` / REST.

| Tool | Use |
|------|-----|
| `get_issue` | Fetch by `SRES-N`. Pass `includeRelations` when scope vs related issues matters. |
| `list_comments` | Issue discussion. Oldest-first when reconstructing history. |
| `list_issues` | Search the SRES team. |
| `save_comment` | New top-level comment. Markdown, literal newlines. |
| `save_issue` | Status, `links`, `relatedTo`. Do **not** overwrite the description unless the user asked. |

## Comments and status

- Post a **new top-level** comment. Do not reply to the "This thread is for an agent session" stub.
- Do not edit the issue description to store a plan or progress; comment instead.
- `relatedTo` and `links` are append-only. Related issues own their own scope unless the user says otherwise.
- Set status only when the user asked, or when it is an obvious match (starting work → `In Progress`; PR opened → `In Review`; user said the work is done and the PR is merged → `Done`). Do not set `Canceled` or `Duplicate` unless asked. Do not set `Done` before the PR is merged.

## Gotchas

- Identifier is `SRES-N`. `SRE-N` is an alias only; always record and write `SRES-N`.
- `gh` GraphQL `pr view` can 500 when REST works.
- Do not paste secrets into Linear.

## Pointers

- Root `AGENTS.md` — git from `origin/main`, delete head after merge, `gh` read-only
- [../workflow/SKILL.md](../workflow/SKILL.md) — gated Understand / Implementation Plan / Execution / Submit
- [../code-review/SKILL.md](../code-review/SKILL.md) — PR/diff review
- Linear: `get_issue`, `list_comments`, `save_comment`, `save_issue`
