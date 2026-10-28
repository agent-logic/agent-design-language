# Repo Architecture Review Scaffold

## Metadata

- Skill: repo-architecture-review
- Repo: agent-design-language
- Packet: packet
- Date: 2026-09-23T16:43:44Z

## Findings

- No findings have been written yet. Replace this section with findings-first architecture review output after inspection.

## Architecture Map

- Use the evidence below to map modules, layers, runtime boundaries, state ownership, integration points, and drift surfaces.

## Reviewed Surfaces

- .csdlc/evidence/872/conversion-rehearsal/snapshots/current-observations/970/git-common/csdlc-v3/semantic/issues/970/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/current-observations/981/git-common/csdlc-v3/semantic/issues/981/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/negative-observations/970/git-common/csdlc-v3/semantic/issues/970/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/negative-observations/970/semantic/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/negative-observations/981/git-common/csdlc-v3/semantic/issues/981/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/negative-observations/981/semantic/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/local/locks/113.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/local/locks/122.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/local/locks/3.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/local/locks/497.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/local/locks/505.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/local/locks/511.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/local/locks/517.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/local/locks/868.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/local/locks/970.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/local/locks/981.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/113/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/122/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/3/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/497/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/505/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/511/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/517/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/970/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/981/state.lock (manifest): manifest or dependency/build surface
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/state.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/10.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/100.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/101.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/111.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/112.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/113.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/114.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/116.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/133.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/143.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/146.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/149.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/150.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/151.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/152.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/153.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/154.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/155.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/156.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/157.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/158.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/159.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/160.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/161.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/162.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/163.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/164.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/165.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/166.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/167.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/168.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/169.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/17.lock (manifest): manifest or dependency/build surface
- .csdlc/locks/170.lock (manifest): manifest or dependency/build surface

## Candidate Diagram Tasks

- .csdlc/evidence/872/conversion-rehearsal/snapshots/current-observations/970/git-common/csdlc-v3/semantic/issues/970/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author
- .csdlc/evidence/872/conversion-rehearsal/snapshots/current-observations/981/git-common/csdlc-v3/semantic/issues/981/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author
- .csdlc/evidence/872/conversion-rehearsal/snapshots/negative-observations/970/git-common/csdlc-v3/semantic/issues/970/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author
- .csdlc/evidence/872/conversion-rehearsal/snapshots/negative-observations/970/semantic/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author
- .csdlc/evidence/872/conversion-rehearsal/snapshots/negative-observations/981/git-common/csdlc-v3/semantic/issues/981/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author
- .csdlc/evidence/872/conversion-rehearsal/snapshots/negative-observations/981/semantic/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/113/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/122/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/3/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/497/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/505/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author
- .csdlc/evidence/872/conversion-rehearsal/snapshots/pre-effect-converted/semantic/issues/511/state.lock: High-signal architecture surface likely benefits from a visual boundary or lifecycle view. Handoff: diagram-author

## Candidate ADRs

- None identified by scaffold.

## Candidate Fitness Functions

- None identified by scaffold.

## Validation Performed

- Scaffold generation only; no repository validation commands were run by this helper.

## Residual Risk

- This scaffold is not a review finding artifact. A reviewer must inspect the selected surfaces and record findings or an explicit no-material-findings result.
