# GHB implementation planning packet

Date: 2026-10-10. Status: proposed implementation program, not approved delivery or demonstrated capability.

Requested by the operator following repository and literature research. Audience: ADL and Runtime implementers, reviewers, and program owners.

- [Design and research](DESIGN_AND_RESEARCH.md): current evidence, gaps, normative algorithm proposal, runtime architecture, research sources, and conformance criteria.
- [Sprint umbrella and issue list](ISSUE_LIST.md): 14-issue roster, proposed titles, repositories, dependencies, acceptance criteria, validation and scope boundaries; [machine-readable candidates](issue_candidates.json).
- [Execution plan](IMPLEMENTATION_PLAN.md): work packages, dependencies, acceptance tests, delivery waves, ownership, and release gates.
- [Time and token estimate](ESTIMATE.md): bottom-up effort, elapsed schedule, token accounting, uncertainty, and recalibration procedure.

**Required deployment target: the installed Agent Logic Runtime (`agent-logic-runtime`).** Its normal resident execution path must run GHB and consume the accepted successor. An ADL-only demonstration, standalone research runner, or prepared receipt is insufficient for acceptance.

The recommended delivery target is a governed, persistent agent that empirically improves its own body and eventually bounded parts of its improvement process. Formal global-optimality claims are excluded from this target. The G0–G5 labels in this packet are proposed internal terminology, not an established external standard.

This is a local planning packet in the operator-requested `.adl/docs/TBD/` location. It does not create tracked issues, allocate inference spend, authorize production activation, or replace canonical specifications. Implementation should enter the normal issue-bound FastWork workflow. Existing birthday and continuity owners must be reused.

Repository sources were inspected at these local revisions; this is a research snapshot, not a claim that all remotes or active worktrees were audited:

| Repository | Local revision |
| --- | --- |
| agent-design-language | `89b14992c1c21b4388a113c128ee1af563b10659` |
| agent-logic-runtime | `add3c1b696f4779853fdee946a2d68c0db31dc85` |
| godel-hadamard-bayes-paper | `8fb694cfaba81f2d1dfb26beb2383ca07d0b39f3` |

Runtime issue #112 was independently read through GitHub and was open during research. Refresh its state before scheduling overlapping implementation.

Validation performed: checked local document links, Markdown fence balance, referenced implementation paths, work-package estimate sums, and experiment-run arithmetic. Primary tracked checkout remains clean. No implementation tests or live experiments ran for this documentation task. The packet has not had independent technical review; estimates should be recalibrated after WP00.
