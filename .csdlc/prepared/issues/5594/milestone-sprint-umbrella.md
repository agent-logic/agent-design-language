## Summary

Coordinate the single v0.91.8 ADL Core Rearchitecture milestone sprint. This
umbrella owns dependency order, bounded parallelism, integration sequencing,
and terminal sprint convergence; the child issues retain all implementation,
review, and closeout authority for their own scopes.

## Child Topology

- Opening gate: WP-01 #5594.
- ADL v2 critical path: #5336, #5337, #5339, #5338, #5340, #5342, #5341,
  #5349, #5526, and #5345.
- Distributed workcell umbrella: #5497 with #5499, #5498, #5500, #5502, and
  #5501.
- Parity, soak, cutover, and deletion: #5350, #5344, #5343, #5346, and #5347.
- C-SDLC v2 acceptance: #5358 with independent defect inventory #5540, #5541,
  #5548, and #5558.
- Runtime v3 acceptance umbrella: #5361 with parity children #5591, #5592,
  #5589, and #5590.
- Integrated acceptance umbrella: #5384 and its declared WP-14A children.
- Release tail: #5354, #5351, #5360, #5356, #5357, #5363, #5362, #5355,
  #5359, and #5348.
- Operational sidecar: #5587 remains independently owned and may not block or
  broaden the core sprint unless its explicit dependency is accepted.

## Execution Contract

- At most four writable issue/worktree actors are active across the milestone.
- Read-only reviewers and watchers do not consume writable slots.
- Shared planning, interface, selector, publication, merge, post-merge
  validation, and terminal closeout work enters one serialized integration
  queue.
- Cards are prepared no more than one dependency wave ahead.
- No implementation issue starts without issue-specific SIP, STP, SPP, VPP,
  protected paths, and a design-ready review checkpoint.
- External model agents are read-only evidence producers. They cannot mutate
  lifecycle state, create scope, merge, close, or approve release.

## Opening Wave

After WP-01 closes, admit only:

1. #5336 stale-worktree recovery and architecture reconciliation.
2. #5337 typed card preparation.
3. #5358 typed acceptance-card preparation.
4. #5361 typed acceptance-card preparation.

Runtime v3 parity cards follow #5336 integration. Parity-A #5591 precedes
Parity-B #5592, Parity-C #5589, and Parity-D #5590. The latter three may write
concurrently only after their protected-path manifests prove disjointness.

## Definition Of Done

- Every child reaches truthful terminal state or an operator-approved,
  evidence-backed blocker/defer disposition.
- Required parity, rollback, acceptance, deletion, review, remediation, and
  release-tail gates converge in dependency order.
- The canonical v0.91.8 documents and live issue graph agree at exact revision.
- No AWS use, Runtime v2 deletion before reviewed cutover, autonomous merge, or
  v0.92 activation is authorized by this umbrella.
