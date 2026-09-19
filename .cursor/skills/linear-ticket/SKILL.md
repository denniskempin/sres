---
name: linear-ticket
description: "File a Linear issue from an implementation plan. Use when the user asks to create a Linear ticket, issue, or brief for a plan or Gaps item."
---

# File a Linear implementation ticket

Workspace process. Do not put team names or upload steps in `AGENTS.md`.

## When

The user asks to create a Linear ticket / issue for a plan, Gaps item, or implementation brief.

## Procedure

1. Discover Linear tools (`GetDynamicTools`). If the namespace is `needsAuth`, call `mcp_auth` then rediscover.
2. Team is `SRES` (one team). `list_issues` with `query` set to the feature name first. Do not file a duplicate.
3. `save_issue`:
   - `team`: `SRES`
   - `title`: imperative, hardware name + short scope
   - `description`: checklist from the plan todos, then the full markdown plan. Literal newlines. No YAML frontmatter.
4. Attach the plan file when one exists:
   1. `stat -c '%s'` for the exact byte size
   2. `prepare_attachment_upload` (`issue`, `filename`, `contentType`, `size`)
   3. `curl -X PUT --data-binary @file` with **every** `uploadRequest.headers` key verbatim (casing included). Signed URLs expire in 60s.
   4. `create_attachment_from_upload` with the returned `assetUrl`
   Prepare → PUT → finalize **one file** before starting another.
5. Reply with the identifier and URL only.

## Hard rules

- Search before create.
- Do not set assignee, project, or priority unless the user names them.
- Do not comment on or edit an existing ticket unless asked.
