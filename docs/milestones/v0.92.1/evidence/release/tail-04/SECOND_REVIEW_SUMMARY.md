# Issue #520 second internal review

## Review identity

- Frozen candidate: `fb6cbc7f619daa54f901fd2d12f480add682ace3`
- Review gates: #718 / PR #809 and #758 / PR #805
- Denominator: 7,184 uniquely assigned references across nine mandatory lanes
- Sampling: none
- Result: **changes required**
- Findings: 14 total — five P1, seven P2, two P3

The JSON packet and per-lane reports are authoritative. The first-review prose
has been retained under `historical/first-review/` so its older candidate and
resolved findings cannot be mistaken for the current review.

## Current findings and owners

| Finding | Severity | Owner | Summary |
| --- | --- | --- | --- |
| `D520-RET-001` | P1 | #818–#821 | 198 required retained-proof rows remain unproved |
| `D520-V3F-001` | P1 | #817 | V3-F exact-head proof is stale |
| `D520-REL-001` | P1 | #817 | Current release-status projection is stale and invalid |
| `D520-DOC-003` | P2 | #817 | #519 terminal projection contradicts completed publication |
| `D520-DOC-004` | P2 | #817 | Fourteen canonical ownership links are malformed |
| `D520-EVID-001` | P3 | #817 | Eighteen GCP-E JSON readbacks contain YAML |
| `D520-EVID-002` | P3 | #817 | Four retained JSON artifacts are empty |
| `D520-RUNTIME-001` | P1 | #814 | Max-attempt recovery can exceed its bound and poison durable state |
| `D520-RUNTIME-002` | P2 | #814 | Retry replaces the stable ledger work identity |
| `D520-SEC-001` | P1 | #815 | GCP mutation approval is locally self-asserted |
| `D520-SEC-002` | P2 | #815 | AWS authorization is not bound to saved plan bytes |
| `D520-SEC-003` | P2 | #816 | OBS-B redaction validation is vacuous |
| `D520-SEC-004` | P2 | #814 | Local Shepherd proof accepts off-host Ollama URLs |
| `D520-TEST-001` | P2 | #816 | Hot-reload cancellation tests can pass without observing pending work |

## Repair topology

- #814: Runtime greeting recovery, stable identity, and local-model boundary.
- #815: authenticated AWS/GCP mutation authorization.
- #816: redaction and hot-reload validation integrity.
- #817: exact-candidate release truth and retained-evidence format repairs.
- #818: 17 corporate/Runtime retained-proof rows.
- #819: 152 C-SDLC v3 retained-proof rows.
- #820: 25 distributed-Runtime retained-proof rows.
- #821: four final TAIL-01 obligations after all prerequisites pass.

All eight issues are children of the #522 remediation wave by explicit parent
identity in their issue bodies. No finding may be marked resolved until its
own focused proof and exact-head review pass.
