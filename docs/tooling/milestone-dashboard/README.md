# Historical milestone compression dashboard

This directory preserves a reusable static HTML dashboard and its historical
v0.90.4 dataset. It is not a current sprint status page. Use the
[roadmap](../../ROADMAP.md) and [v0.93.2 allocation](../../milestones/v0.93.2/README.md)
for current planning and the relevant issue/PR evidence for live state.

## Purpose

The dashboard gives one operator-facing, read-only view of:

- active WP wave state
- issue and PR/check posture
- validation profile expectations
- review-tail gates
- release blockers
- deferred findings and guarded non-claims
- immediate next operator action
- a bounded read-only workcell operator snapshot with optional Runtime v3
  Observatory read-feed posture

It is a visibility layer only. It does not mutate GitHub issues, PRs, branches,
task cards, Runtime state, release state, or closeout records. It is not
release authority and must not hide review findings or replace the canonical
milestone-compression YAML, milestone docs, task cards, PR checks, Runtime v3
Observatory authority, or human release ceremony.

## Current Dataset

The bundled dataset is `v0.90.4`; it remains the historical snapshot mirrored
in `data/v0.90.4.js` and refreshed from:

- [archived README.md](../../archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.90.4/README.md")
- [archived WBS_v0.90.4.md](../../archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.90.4/WBS_v0.90.4.md")
- [archived WP_ISSUE_WAVE_v0.90.4.yaml](../../archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.90.4/WP_ISSUE_WAVE_v0.90.4.yaml")
- [archived MILESTONE_CHECKLIST_v0.90.4.md](../../archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.90.4/MILESTONE_CHECKLIST_v0.90.4.md")
- [archived RELEASE_PLAN_v0.90.4.md](../../archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.90.4/RELEASE_PLAN_v0.90.4.md")
- [archived README.md](../../archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.90/milestone_compression/README.md")
- [archived CANONICAL_MILESTONE_STATE_v0.90.yaml](../../archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.90/milestone_compression/CANONICAL_MILESTONE_STATE_v0.90.yaml")
- [archived DRIFT_CHECK_REPORT_v0.90.md](../../archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.90/milestone_compression/DRIFT_CHECK_REPORT_v0.90.md")
- [archived FINISH_VALIDATION_PROFILES_v0.90.md](../../archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.90/milestone_compression/FINISH_VALIDATION_PROFILES_v0.90.md")
- a bounded read-only GitHub snapshot of the then-live v0.90.4 issue wave and
  PR posture taken at refresh time

The reviewer entrypoint selected at that historical refresh was:

- [archived RELEASE_READINESS_v0.90.5.md](../../archives/HISTORICAL_FILES_1395.md#find-and-restore-a-historical-path "Archived: docs/milestones/v0.90.5/RELEASE_READINESS_v0.90.5.md")

The same dataset also carries the #5500 WP-10A workcell operator snapshot under
`workcellOperator`. That snapshot is versioned as
`adl.workcell.operator.snapshot.v1`, marks retained/live/unknown/blocked and
non-authoritative evidence explicitly, records #5498/#5349 live-merge
dependency truth, and composes with the Runtime v3 Observatory only through an
operator-supplied HTTPS read feed. Live Runtime access is opt-in with:

```text
?runtime=v3&runtimeApiBase=<https-runtime-api-base>&live=1
```

The browser reads only `/v1/observatory`, requires a bearer token from
`sessionStorage`, applies origin/protocol, timeout, payload-size, and count
limits, and falls back to the retained snapshot on missing, partial, stale, or
failed evidence. Tokens must not be placed in URLs, snapshots, logs, or DOM
text.

Unknown, stale, or unverifiable evidence must be marked unknown/stale rather
than treated as green.

## Refresh Rule (historical policy)

The original policy proposed refreshing during WP-01 or the first milestone-
compression readiness pass. Any future reactivation needs a separately scoped
current dataset and review. That update would include:

- the current milestone data file under `data/`
- the `<script src>` target in `index.html` when switching milestones
- this Current Dataset section
- WP-to-issue mappings, freshness signals, PR/check posture, blockers, next
  actions, and guarded non-claims
- validation expectations in `adl/tools/test_milestone_dashboard.sh` when the
  dashboard contract changes

The dashboard may remain static temporarily as a historical visibility surface,
and is explicitly historical here. A future dataset must name its real target
milestone; neither the retained snapshot nor a README edit establishes live status.

## Files

- `index.html` - static dashboard shell
- `style.css` - visual system and responsive layout
- `dashboard.js` - renderer and stale/unknown-state handling
- `data/` - bounded milestone snapshot data files

## Validation

Run:

```bash
bash adl/tools/test_milestone_dashboard.sh
```

The validation checks that the static dashboard has the required compression
sections, is no longer a legacy-milestone-only artifact, does not contain
private local paths or obvious secret markers, and has JavaScript syntax that
can be parsed when `node` is available.

## Usage

Open `index.html` in a browser.

To adapt it for another milestone, add or refresh a dataset file in `data/`,
update the dataset script reference in `index.html`, and keep the read-only
boundary visible. The renderer should stay generic; the milestone snapshot
should carry the current mirrored truth and explicit unknown/stale notes.
