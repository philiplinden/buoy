# Issue tracker: Linear

Issues and specs for this repo live in Linear. Workspace **Linden Scientific**, team **DEV** (key `DEV`). Use the `claude.ai Linear` MCP tools for all operations — there is no CLI for this tracker.

GitHub is used for code only (branches, commits, PRs). Linear's GitHub integration is connected for this workspace: pasting a `github.com/philiplinden/buoy/...` PR or issue URL into a Linear issue body auto-links it. GitHub issues on this repo are retired — do not create new ones; if you find an open one, migrate it to a Linear issue and close the GitHub original with a pointer to the Linear issue's URL.

## Conventions

- **Create an issue**: `save_issue` with `team: "dev"`, `title`, and `description` (Markdown). Omit `id`.
- **Read an issue**: `get_issue` with `includeRelations: true` when blocking/related links matter.
- **List issues**: `list_issues`, filtered by `team`, `parentId`, `label`, `state`, `assignee` as needed.
- **Comment on an issue**: `save_comment`.
- **Apply / remove labels**: `save_issue` with `labels: [...]` — this **replaces** the full label set, so include every label you want kept, not just the one you're adding.
- **Close**: `save_issue` with `state: "Done"` (or `"Canceled"` if abandoned rather than resolved).

## Pull requests as a triage surface

**PRs as a request surface: no.** _(Set to `yes` if this repo treats external PRs as feature requests; `/triage` reads this flag.)_

This repo's PRs stay on GitHub as normal — only the issue/spec tracker moved to Linear. `gh pr` commands still work unchanged for PR review, diffs, and merging (`origin` → `philiplinden/buoy` is `gh`'s pinned default repo — see `gh repo set-default`).

## When a skill says "publish to the issue tracker"

Create a Linear issue via `save_issue` with `team: "dev"`.

## When a skill says "fetch the relevant ticket"

Run `get_issue` with the issue's identifier (e.g. `DEV-12`).

## Wayfinding operations

Used by `/wayfinder`. The **map** is a single Linear issue with **child** issues (via `parentId`) as tickets.

- **Map**: a single issue labelled `wayfinder/map` (label group `wayfinder`, sub-label `map`), holding the Destination / Notes / Decisions-so-far / Fog body. `save_issue` with `team: "dev"`, `labels: ["<map label id>"]`.
- **Child ticket**: `save_issue` with `parentId: "<map identifier>"` and label `wayfinder/<type>` (`research`/`prototype`/`grilling`/`task` — sub-labels of the `wayfinder` group). Once claimed, `assignee: "me"`.
- **Blocking**: Linear's **native** issue relations — `save_issue`'s `blockedBy` (append-only; pass the blocker's issue identifier, e.g. `["DEV-4"]`). A ticket is unblocked when every issue in its `blockedBy` relation is closed (`Done` or `Canceled`).
- **Frontier query**: `list_issues` with `parentId: "<map identifier>"`, filter out anything with `state` of `Done`/`Canceled`, an `assignee`, or an open `blockedBy` relation (check via `get_issue` with `includeRelations: true`); first in map order wins.
- **Claim**: `save_issue` with `id: "<ticket>"`, `assignee: "me"` — the session's first write.
- **Resolve**: `save_comment` with the answer, then `save_issue` with `state: "Done"`, then append a context pointer (gist + link) to the map's Decisions-so-far via `save_issue`'s `patch`.

## Label vocabulary gap

`docs/agents/triage-labels.md` maps triage roles (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`) to tracker label strings under the assumption of a GitHub-style flat label set. Those labels do not yet exist on the Linear `DEV` team (only the defaults `Bug`/`Feature`/`Improvement` plus the `wayfinder` group exist as of 2026-08-09). Create them in Linear (`create_issue_label`, `teamId` for the `dev` team) before relying on `/triage` against this tracker.
