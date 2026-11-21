---
name: Context7 docs
description: Fetch current, version-specific library documentation with the Context7 MCP before writing code that uses a third-party library. Use whenever a task involves an external API, framework version or unfamiliar library.
---

# Context7 documentation lookup

Context7 is configured as the `context7` MCP server. It exposes
`context7_resolve-library-id` and `context7_query-docs`. Look up docs **before** writing
code that calls a library API.

## Workflow

1. **Resolve the library ID.** Call `context7_resolve-library-id` with a short, specific
   query such as `next.js app router` or `supabase row level security`. If you already
   know the canonical ID (for example `/vercel/next.js` or `/supabase/supabase`), skip
   this step.
2. **Query the docs.** Call `context7_query-docs` with the resolved ID and a specific
   question. Include the version when it matters, for example:
   "How do I configure middleware in Next.js 15?"
3. **Refine if needed.** If results are off-target, ask a narrower question or request a
   different section. Do not guess an API from training data.
4. **Record the source.** Put the resolved library ID (and version, if relevant) in the
   implementation plan when the design depends on non-obvious APIs.

## Good queries

- Good: "How do I upload a file with Supabase Storage in the JS client?"
- Poor: "How does Supabase work?"
- Good: "Cloudflare Workers cache API `caches.default` put options"
- Poor: "cloudflare docs"

## Notes

- Sign in through `/mcps` if the server reports that it needs authentication.
- MCP tools consume context. Resolve and query only the libraries you actually need.
- If the MCP is unavailable, say so explicitly rather than substituting remembered APIs.
