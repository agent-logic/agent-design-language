# WP-11 Runtime v2 Cognitive Control Evidence Packet

Issue: `#4638`
Version: `v0.91.7`
Status: `umbrella_evidence_packet`
Date: `2026-07-10`

## Purpose

This packet ties the merged WP-11 child and follow-on work into one retained
review surface for the Runtime v2 cognitive-control claim. It is not a release
approval and does not replace the issue-local SRP/SOR cards.

## Integrated Claim

WP-11 has landed the repository implementation slices needed to treat reasoning
graphs, bounded loops, `adl.skill.v1`, AEE/ObsMem handoff, and Godel/GHB runtime
state as connected Runtime v2 control-plane surfaces.

The supported claim is intentionally scoped:

- Reasoning graphs are implemented as Runtime v2 graph/state objects with
  validation and proof hooks.
- Loops are implemented as bounded, replayable Runtime v2 execution objects
  bound to reasoning graph and runtime state.
- `adl.skill.v1` is implemented as a concrete skill contract and dispatch
  surface.
- AEE/ObsMem/PVF handoff proof is implemented as a Runtime v2 evidence handoff
  slice.
- Godel snapshot/diff and GHB recursive loop work has landed as Runtime v2
  state and agent-runtime follow-on work.

## Evidence Index

| Slice | Issue | PR | Evidence status |
| --- | --- | --- | --- |
| Reasoning graph runtime | `#4694` | `#5091` | Merged PR evidence reviewed by `V0917_WP11_REASONING_LOOPS_SKILL_REVIEW_4638.md`. |
| Loop runtime | `#4695` | `#5104` | Merged PR evidence reviewed by `V0917_WP11_REASONING_LOOPS_SKILL_REVIEW_4638.md`. |
| `adl.skill.v1` runtime contract | `#4696` | `#5099` | Merged PR evidence reviewed by `V0917_WP11_REASONING_LOOPS_SKILL_REVIEW_4638.md`. |
| AEE/ObsMem/PVF trace handoff | `#4697` | `#5101` | Merged PR evidence reviewed by `V0917_WP11_REASONING_LOOPS_SKILL_REVIEW_4638.md`. |
| Godel snapshot/diff protocol | `#4912` | `#5106` | Merged PR evidence reviewed by `V0917_WP11_REASONING_LOOPS_SKILL_REVIEW_4638.md`. |
| GHB recursive self-improvement loop | `#5096` | `#5127` | Merged PR evidence reviewed by `V0917_WP11_REASONING_LOOPS_SKILL_REVIEW_4638.md`; broad-lane caveat retained. |
| GHB as Runtime v2 agent runtime | `#5136` | `#5138` | Merged PR evidence reviewed by `V0917_WP11_REASONING_LOOPS_SKILL_REVIEW_4638.md`; focused and exact-rerun proof retained. |

## Child Card Supersession

The umbrella review found that several local child lifecycle cards under
`.adl/v0.91.7/tasks/` are stale relative to merged PR evidence. For WP-11
release consumption, this packet explicitly supersedes those stale local child
card statements with the merged PR evidence reviewed in
`V0917_WP11_REASONING_LOOPS_SKILL_REVIEW_4638.md`.

This supersession is narrow:

- It applies only where a local child card contradicts merged PR and reviewed
  validation evidence.
- It does not rewrite or silently normalize the ignored local cards.
- It does not convert focused child validation into broad release readiness.
- It does not apply to future changes after this packet date.

| Child issue | Superseded local-card risk | Current evidence to consume |
| --- | --- | --- |
| `#4694` | Local card text still describes incomplete PR/merge path. | Consume merged PR `#5091` and its validation summary from the WP-11 review packet. |
| `#4695` | Local SRP/SOR card truth may still read pre-run/not-run. | Consume merged PR `#5104` and its validation summary from the WP-11 review packet. |
| `#4696` | Local SRP/SOR card truth may still read pre-run/not-run. | Consume merged PR `#5099` and its validation summary from the WP-11 review packet. |
| `#4697` | Local SRP/SOR card truth may still read pre-run/not-run. | Consume merged PR `#5101` and its validation summary from the WP-11 review packet. |
| `#4912` | Local SRP/SOR card truth may still read pre-run/not-run. | Consume merged PR `#5106` and its validation summary from the WP-11 review packet. |
| `#5096` | Local SRP/SOR card truth may still read pre-run/not-run. | Consume merged PR `#5127` and its bounded validation caveat from the WP-11 review packet. |

Issue `#5136` was separately closeout-repaired before this umbrella packet and
its merged PR `#5138` remains the current implementation evidence.

## Validation Summary

The umbrella review packet records the child validation summaries. This packet
does not rerun the child validations. The retained validation claim is limited
to the evidence reviewed in:

- `docs/milestones/v0.91.7/review/V0917_WP11_REASONING_LOOPS_SKILL_REVIEW_4638.md`

The closeout-truth repair for `#4638` additionally ran:

- `bash adl/tools/validate_structured_prompt.sh --type srp --phase pre_run --input .adl/v0.91.7/tasks/issue-4638__v0-91-7-wp-11-implement-reasoning-graphs-loops-and-adl-skill-v1-in-full/srp.md`
- `bash adl/tools/validate_structured_prompt.sh --type sor --phase pre_run --input .adl/v0.91.7/tasks/issue-4638__v0-91-7-wp-11-implement-reasoning-graphs-loops-and-adl-skill-v1-in-full/sor.md`

Both structured prompt checks passed.

## Non-Claims

- This packet does not claim `#4638` is closed.
- This packet does not claim v0.91.7 release readiness.
- This packet does not claim live hosted-provider invocation for GHB/Godel
  agent runtime.
- This packet does not claim broad no-regression coverage beyond the validation
  surfaces recorded by the individual PRs.
- This packet does not claim stale local child lifecycle cards are current where
  they contradict merged PR evidence.
- This packet supersedes stale local child-card statements only for WP-11
  umbrella release consumption; it does not globally repair those ignored local
  cards.
- This packet does not promote ignored local proof outputs into retained release
  evidence.

## Remaining Closeout Work

- Finish `#4638` closeout through the normal repo-native lifecycle.
- Keep this packet's child-card supersession boundary visible in release
  evidence.
