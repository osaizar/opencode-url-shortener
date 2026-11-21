# OpenCode Development Kit

A reusable, stack-agnostic [OpenCode](https://opencode.ai/) configuration kit: base
configuration, one custom subagent, slash commands, local and third-party skills, an MCP
server, and a requirements-first workflow with reusable document templates.

## What's inside

- **`opencode.jsonc`** — project config with the Context7 MCP server and a light override
  of the built-in `plan` agent so it can author documents but not code.
- **`AGENTS.md`** — persistent rules: workflow gates, artifact naming, definition of done.
- **`.opencode/agents/spec-reviewer.md`** — custom subagent for the pre-implementation
  gate: reviews requirement and plan quality, coverage and testability.
- **`.opencode/agents/verifier.md`** — custom subagent for the post-implementation gate:
  a read-only-to-code subagent that reviews the diff, runs checks and validates
  acceptance criteria.
- **`.opencode/commands/`** — `/spec`, `/plan`, `/spec-review`, `/review`, `/implement`.
- **`.opencode/skills/`** — local skills for EARS+BDD authoring, planning, and Context7 docs.
- **`.agents/skills/` + `skills-lock.json`** — skills installed from
  [skills.sh](https://skills.sh/), committed for reproducibility.
- **`docs/`** — templates and two complete worked examples (R001/R002).

OpenCode reuses its built-in `plan`, `build`, `explore` and `general` agents for everything
else, so the kit adds only two agents: `spec-reviewer` and `verifier`

## Quick start

1. Copy this repo (or `opencode.jsonc`, `AGENTS.md`, `.opencode/`, `.agents/` and `docs/`)
   into your project.
2. Install the third-party skills (Node.js >= 22.20 required):

   ```bash
   npx skills add mattpocock/skills \
     --skill grill-me --skill implement-spec --skill code-review \
     -a opencode -y

   npx skills add vercel-labs/skills --skill find-skills -a opencode -y

   # grill-me is a pointer to the grilling skill; install that too.
   npx skills add mattpocock/skills --skill grilling -a opencode -y
   ```

3. Start OpenCode, run `/mcps`, select **context7** and sign in with OAuth. Confirm with
   `opencode mcp list` that it reports `connected`.
4. Begin with `/spec "short feature description"`.

Config and command changes reload automatically. Use `/reload` to force a full reload.

## Workflow

```
/spec  →  /plan  →  /spec-review  →  /implement  →  /review  →  done
```

1. **`/spec "<feature>"`** — the `plan` agent uses `grill-me` to clarify the feature with
   you, then `requirements-earns-bdd` to write `docs/requirements/R###-<slug>.md`.
   Requirements are EARS statements with stable `REQ-###` IDs; acceptance criteria are
   Gherkin `Given/When/Then` scenarios linked to those IDs.
2. **Refine** — ask `general` or `explore` to challenge completeness and feasibility. Do
   not proceed until you approve the spec.
3. **`/plan R###`** — writes `docs/plans/P###-<slug>.md`: approach, alternatives, file-level
   changes, test strategy, risks, task checklist and `REQ-###` traceability. An ADR is
   recorded in `docs/decisions/` only for consequential decisions.
4. **`/spec-review R### P###`** — the `spec-reviewer` subagent independently checks the
   documents for gaps, contradictions, coverage holes and untestable criteria. It writes
   `docs/reviews/<requirement-id>-spec-review.md`. Iterate until clean.
5. **`/implement P###`** — the `build` agent writes code and tests, uses Context7 for
   library APIs, and finishes by launching the `verifier` subagent.
6. **Final `/review`** on the diff. Done when every Gherkin scenario passes and the
   verification report has no blocking findings.

## Skills

Local skills (`.opencode/skills/`) win on ID collisions with installed skills:

| Skill | Purpose |
|---|---|
| `requirements-earns-bdd` | Author EARS statements + Gherkin acceptance criteria |
| `implementation-planning` | Produce traceable implementation plans |
| `context7-docs` | Auto-triggered library doc lookups via Context7 |
| `grill-me` | Pointer skill; delegates to `grilling` |
| `grilling` | Clarify/stress-test requirements before authoring the spec |
| `implement-spec` | Implement from the approved spec using a task graph |
| `code-review` | Review checklist used by the verifier |
| `find-skills` | Discover more skills later |

Check for updates with `npx skills check`, list with `npx skills list`, and update project
skills with `npx skills update -p -y`. Installed skills are vendored — review each
`SKILL.md` before committing. Set `DISABLE_TELEMETRY=1` to opt out of skills.sh telemetry.
Some skills use `context: fork` or hooks, which OpenCode does not support; the skills
installed here use basic skills only. `grill-me` is a pointer to `grilling`, and
`code-review` launches two sub-agents for its Standards/Spec axes — `opencode.jsonc`
therefore enables `experimental.subagent_depth: 2` so the `verifier` sub-agent can spawn
them. Re-check that experimental setting when upgrading OpenCode. `implement-spec` is
adapted in `/implement`: the plan's task checklist stands in for its ticket graph, and its
issue-tracker/PR/worktree steps are skipped.

## MCP: Context7

Configured in `opencode.jsonc` as a remote server at `https://mcp.context7.com/mcp` with
OAuth (no API key in config). It exposes `context7_resolve-library-id` and
`context7_query-docs`. A commented API-key variant is included for headless/CI use.
Only Context7 is active; GitHub and Playwright are documented as disabled opt-ins to keep
context usage low.

Resolve library IDs with Context7 before finalising any plan that depends on a third-party
API, and record them in the plan.

## Verification

There are two independent gates, served by two subagents:

**Before implementation — `/spec-review` (the `spec-reviewer` subagent):**

- checks EARS quality, `REQ-###` coverage by Gherkin scenarios, testability and scope;
- checks the plan's traceability from every `REQ-###` to a change and a test;
- writes `docs/reviews/<id>-spec-review.md` with a PASS/FAIL verdict;
- stops if implementation already exists, deferring to `/review`.

**After implementation — `/review` (the `verifier` subagent):**

- reads the diff and reports static findings by severity with `file:line`;
- discovers and runs the project's test/check commands from `AGENTS.md` or the manifest;
- marks each Gherkin scenario pass/fail with evidence;
- writes `docs/reviews/<id>-verification.md`;
- never edits code; stops if there is no implementation yet, deferring to `/spec-review`.

Since no stack is chosen yet, the verifier reports "no automated checks configured" until
test commands exist.

## Future extension: automatic verification on session idle

Verification is currently command-driven: `/implement` ends with the verifier subagent,
and `/review` invokes it directly. If you want verification on *any* build session, add a
small local plugin:

```ts
import { Plugin } from "@opencode/plugin"

export default Plugin.define({
  id: "auto-verify",
  async setup(ctx) {
    const controller = new AbortController()
    void (async () => {
      for await (const event of ctx.event.subscribe({
        types: ["session.idle"],
        signal: controller.signal,
      })) {
        const sessionID = event.data.sessionID
        const session = await ctx.session.get({ sessionID })
        if (session.agent !== "build" || session.parentID) continue
        if (await ctx.storage.get(`verified:${sessionID}`)) continue
        await ctx.storage.set(`verified:${sessionID}`, true)
        await ctx.session.prompt({
          sessionID,
          text: "Launch the verifier subagent for this turn and include its report.",
        })
      }
    })()
    return () => controller.abort()
  },
})
```

**Guard against loops:** injecting a prompt keeps the session agent as `build`, so an
unguarded subscription re-triggers itself. The example above verifies each session once;
a smarter version should compare the turn's diff against the last verified state. This
plugin is documented but intentionally not included in the initial kit.

## Customising for a new project

- Add your own `R###`/`P###` documents using the templates in `docs/`.
- Once a stack is chosen, add `formatter`, `lsp` and test commands to `AGENTS.md` and
  `opencode.jsonc`, then extend the verifier prompt with the exact check commands.
- Keep `default_agent: "build"`; override individual built-ins rather than replacing them.
