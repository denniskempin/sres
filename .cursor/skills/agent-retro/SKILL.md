---
name: agent-retro
description: "Mine a completed conversation, transcript, or prior agent run for friction, then propose generalized AGENTS.md or skill edits. Use only when the user asks to retro, debrief, or review a conversation for agent-doc gaps. Do not use to run Understand/Plan/Execute/Submit (workflow), to draft implementation plans, post on Linear/GitHub issues (linear), implement hardware or tests, read AGENTS.md as reference during other work, or edit a skill the user already named. Do not use to write AGENTS.md after an architecture change (write-agents-docs), to review a code diff (code-review), to store user-preference memory (continual-learning), or for periodic crate/CI/skill housekeeping (health-audit)."
---

# Agent retro

Review a conversation for friction a future agent would hit again. Propose **generalized** `AGENTS.md` and skill changes. Suggest only until the user asks to apply. Do the work in this conversation; do not launch a subagent (subagents cannot see the chat).

Policy, architecture, and test taxonomy live in root `AGENTS.md`. How to author `AGENTS.md` / `//!` lives in [../write-agents-docs/SKILL.md](../write-agents-docs/SKILL.md). Do not restate either.

## When

Use when any of these is true:

- The user asks to retro, debrief, or review a conversation for agent-doc gaps
- The user names a transcript, prior agent run, or PR thread as the source to mine

Do not use for:

- Gated Understand / Implementation Plan / Execution / Submit (`workflow`)
- Drafting or posting implementation plans, issue comments, or Linear/GitHub tickets
- Implementing a feature or bugfix, even if `AGENTS.md` or a skill might be updated later
- Reading `AGENTS.md` or skills as reference during other work
- Editing a skill or `AGENTS.md` the user already named (`write-agents-docs` for `AGENTS.md` / `//!`; otherwise edit the named skill)
- Authoring `AGENTS.md` after a known architecture or test-strategy change (`write-agents-docs`)
- Reviewing a code diff (`code-review`)
- Looking up Linear usage (`linear`)
- Mining chats for user-preference memory (`continual-learning` / `agents-memory-updater`)
- Periodic crate, CI, or skill/AGENTS.md housekeeping (`health-audit`)

## Procedure

1. **Source.** Default: this conversation. If the user names a transcript path, cloud-agent run, or PR thread, gather that first. Do not invent turns.
2. **Inventory.** Read, in order:
   - Root `AGENTS.md`
   - The nearest directory `AGENTS.md` for every area the conversation touched
   - Every `.cursor/skills/*/SKILL.md` frontmatter (`name` + `description`). Open a body only when the description might already cover the friction
3. **Extract friction.** List only events that cost retries, wrong-layer edits, failed commands, user corrections, or long rediscovery. Ignore smooth work.
4. **Classify** each item with the table below. Drop one-offs.
5. **Generalize.** Run the gate on every survivor. Fail → move to `## Rejected`.
6. **Dedup.** If the fact or procedure already exists, do not copy it. You may propose a pointer (root Reference, skill `When`, leaf one-liner that names the parent pattern). If the agent simply failed to read existing docs, reject.
7. **Report** in the template. Stop. Apply only when the user says to apply.

Cap: at most 5 `AGENTS.md` edits and 2 skill create/improve items. Prefer deleting a stale line over adding a new one.

## Where a change belongs

| Kind | Home |
|------|------|
| Durable fact that `ls` / one `rg` cannot reproduce | Nearest directory `AGENTS.md` (`Behaviors & Gotchas`, `Gaps`, `Tests`) |
| Cross-cutting policy, commands, system variants, test taxonomy | Root `AGENTS.md` |
| Hardware register semantics | Pointer to `docs/index.md`, never a new extract |
| Multi-step procedure the agent should not invent | Skill (improve existing first) |
| User preference or one-off task detail | Nowhere in this skill |

A skill is the wrong home for a fact. `AGENTS.md` is the wrong home for an 8-step workflow. A missing command (`bass foo.asm`, a nextest filter) is one `AGENTS.md` line, not a skill. If you propose a skill, `AGENTS.md` gets only a Reference pointer, not the same steps.

## Generalization gate

Every proposal must pass all of these:

- **Class, not instance.** Strip ROM titles, game names, issue/PR numbers, single opcodes, and the user's wording of this task. Name the hardware class, test type, or layer.
- **Second task.** The report's `Would help when` names a *different* subsystem, test type, or directory where the same guidance applies. If you cannot name one, reject.
- **Inferable-content test.** If `ls` or one `rg` reproduces it, it does not go in `AGENTS.md` (write-agents-docs).
- **One home.** Pointers elsewhere, not a restatement.
- **Actionable.** Names the file, section, and the exact bullet/procedure. Not "consider documenting DMA better."

| Conversation friction | Reject (too specific) | Accept (class of work) |
|-----------------------|-----------------------|------------------------|
| HDMA work for one game, agent put it in `ppu/` | "When adding HDMA for that ROM, edit `ppu/`" | Unimplemented `$4200`-range MMIO belongs in `main_bus/`; components never own it. `Gaps` already lists HDMA — add a procedure skill only if the *implementation sequence* is reused |
| Trace mismatch on one CPU test | "Use `SyncSystem` for that test ROM" | Trace-comparison tests use `SyncSystem` (root Testing Strategy). If that row exists, fix discoverability, do not duplicate |
| Agent guessed a nextest filter | "The command for this file is …" | Directory `## Tests` must include the exact filtered command that runs today |
| Agent updated one PPU snapshot by hand | Skill named after that snapshot | Skill only if golden-image update is a reused multi-step procedure (regenerate, inspect `.actual.png`, commit). Name the test type, not the asset |
| Agent tried `bass -o` then ca65 for one DMA ROM | Skill `assemble-dma-wram` plus the same `bass` flags in root `AGENTS.md` | One `## Tests` / Environment Gotchas line: `bass foo.asm` from the source dir, no `-o`. A skill only if assemble → commit binary → wire driver is a reused procedure; then `AGENTS.md` points at the skill |

## Skill proposals

Prefer **improving** an existing skill (description triggers, `When` near-misses, a missing step, a gotcha). Create a new skill only when all of these hold:

- The procedure is multi-step and will be reused across tasks
- No existing skill covers it
- The work is not a fact that belongs in `AGENTS.md`

New skills live at `.cursor/skills/<name>/SKILL.md` with `name` equal to the folder (kebab-case, `[a-z0-9-]`). Description: what + when + near-miss `Do not use`. Body: `When` → numbered procedure → gotchas → pointers to `AGENTS.md` / other skills. Policy stays in `AGENTS.md`; the skill does not copy it.

After adding a skill agents should discover from the root, add one Reference bullet in root `AGENTS.md`. Add a `Do not use` near-miss on sibling skills that share keywords.

## Report template

Return exactly this structure:

```
## Verdict
enough | gaps-found

## Friction
- What happened (one line). Cost (retry / wrong layer / user correction).

## Proposals
### AGENTS.md
- `path` → `## Section` — proposed line. Would help when: <different task>. Why it is not inferable.

### Skills
- improve `skill-name` | create `skill-name` — what changes. Would help when: <different task>.

## Rejected
- Item — too-specific | already-documented | inferable | one-off | agent-skipped-existing-docs.

## Apply
not-requested | ready (user asked to apply)
```

`enough` only when `## Proposals` is empty. `Would help when` is required on every proposal.

Do not paste conversation excerpts, secrets, or credentials into `AGENTS.md` or skills.

## Apply

If `Apply` is `not-requested`, do not edit files.

If the user asks to apply:

1. Apply only items listed under `## Proposals`. Leave `## Rejected` untouched.
2. `AGENTS.md` / `//!`: read [../write-agents-docs/SKILL.md](../write-agents-docs/SKILL.md) and follow it exactly (template order, budgets, net-negative bias, self-check). This skill does not restate that procedure.
3. Skills: edit or create as specified. Keep `name` matching the folder. Do not duplicate `AGENTS.md` policy.
4. Root Reference and sibling `Do not use` lines: update when adding a discoverable skill.
5. Re-read the write-agents-docs self-check for every touched `AGENTS.md`.

## Hard rules

- Do not load this skill unless the user asked to retro, debrief, or review a conversation. "Might later need `AGENTS.md`" is not a trigger.
- Suggest first; never apply on the retro turn unless the user already said to apply.
- Never write `## Learned User Preferences` or `## Learned Workspace Facts` (that is continual-learning).
- Never add a skill whose body is a single fact.
- Never name a ROM, game, or issue in a proposed `AGENTS.md` line or skill description unless that identifier is already a fixture the test harness requires.
- If the only lesson is "read the existing `AGENTS.md`," the verdict is `enough`.

## Self-check

Answer each before sending the report. Any "no" means go back.

- Existing root, leaf `AGENTS.md`, and skill descriptions inventoried?
- Every proposal has `Would help when` naming a different task?
- No ROM, game, or issue in proposed lines unless it is a test fixture?
- Nothing already documented, inferable by one `rg`, or a one-off?
- No skill whose only payload is a single command? No `AGENTS.md` line that restates a proposed skill's steps?
- Caps respected (5 `AGENTS.md`, 2 skills)?
- `Apply` is `not-requested` unless the user asked to apply?
