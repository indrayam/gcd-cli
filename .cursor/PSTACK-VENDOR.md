# Vendored pStack

Copied verbatim from https://github.com/cursor/plugins/tree/850d7f2e9f5e55dff7eb00a0a479337fe79eb22f/pstack (plugin.json version 0.15.25).
Author: Lauren Tan (poteto). License: MIT, see .cursor/PSTACK-LICENSE.

Layout:
- pstack/skills/*  -> .cursor/skills/*   (project skills; cloud agents read these from the repo)
- pstack/agents/*  -> .cursor/agents/*   (project subagents: poteto-agent, comment-sicko)
- pstack/docs/guide -> .cursor/docs/guide (keeps poteto-help's ../../docs/guide links valid)

Why: on a Cursor cloud VM the pstack plugin was not present under ~/.cursor/plugins/, and /poteto-mode was not recognized.
Known gap: agents/poteto-agent.md falls back to searching ~/.cursor/plugins/ when it gets no path. poteto-mode tells the parent to always pass the absolute path to .cursor/skills/poteto-mode/SKILL.md, so the fallback should not be needed.
To update: re-copy from a newer cursor/plugins commit and change the SHA above.
