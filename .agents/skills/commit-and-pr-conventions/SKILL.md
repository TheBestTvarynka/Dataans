---
name: commit-and-pr-conventions
description: Follow the adopted commit and pull-request conventions. Use whenever creating a commit or opening, editing, reviewing, or preparing to squash-merge a PR, including requests to commit, push, or open a PR.
---

Treat the PR title and initial body as the exact commit message that squash-and-merge will land.

## Commit message

- Follow Conventional Commits: `<type>[optional scope][!]: <description>`.
- Separate subject, body, and footers with blank lines.
- For standalone commits, wrap body lines at 72 characters. Explain why the change was needed, how it solves it, and material side effects.
- Add applicable Conventional Commit footers and `Co-authored-by: Name <email>` trailers for collaborators.
- Choose the type by the affected surface, not by whether the change fixes a defect.
- Use `ci` for CI, workflow, and automation changes, including fixes to that automation.
- Use `fix` for product or runtime bugs and non-feature changes to externally consumed public API boundaries.
- Use `feat` for features, `build` for build or dependencies, `chore` for non-product tools or configuration, `docs` for documentation only, `style` for non-semantic edits, `refactor` for internal restructuring, `test` for tests.
- Mark breaking changes with `!` or a `BREAKING CHANGE` footer without changing an otherwise accurate type.
- Keep every non-squashed development commit coherent and conventional.

```text
fix(backend): image pasting from clipboard;

On KDE with the Wayland protocol, the `arboard` crate failed to load
the image from the clipboard.

Remove clipboard data type checking. Now it tries to paste the image;
if that fails, it pastes the clipboard data as text. Enabled the
`wayland-data-control` feature of the `arboard` crate. This feature
prioritizes Wayland protocol over X11. It should not break X11-only
desktops.
```

```text
ci: run clippy and tests in parallel;
```

## Commit scope

Use one of the following scopes when creating a commit message/PR title:

- `backend`: the Tauri application backend only changes.
- `frontend`: the Tauri application frontend only changes.
- `web-server`: the `web-server` improvements only changes.
- `sync`: any improvements or changes in synchronization algorithm or its implementation.
- `crypto`: cryptography-related improvements and fixes.
- `agents`: agent instructions and reusable skills.

You may combine scopes but never invent a catch-all scope; explain secondary effects in the body.
Do not use types or cross-cutting aspects as scopes.
Use no scope for genuinely repository-wide changes.

## Pull request

- Set the title to the intended squash commit subject exactly.
- Set the initial body to the intended squash commit body and footers exactly. Exclude PR-only preambles, checklists, and implementation journals.
- Do not manually wrap the PR body; GitHub handles display wrapping.
- Keep permanent history concise: record motivation, material behavior, and side effects.
- Put experiments, alternatives, extensive validation, and implementation chronology in follow-up comments.
- Post a follow-up comment with the lines changed, split at least into production code (what ships to users), tests, and other files, with CI, build scripts, and developer tooling counted as other files, so a reviewer sees where the change's weight lies before reading it; how to count and present them is yours, and refresh it when the diff changes materially.
- Update the title and initial body as the implementation evolves.
- Before opening, updating, or merging, read `title + blank line + body` as one commit message and correct any mismatch.