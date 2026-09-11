# v0.92.2 adopted planning contracts

Status: tracked design candidate assembled by #523; implementation and formal design approval remain gated. This document carries the requirements adopted for Beta 1 so a clean checkout does not depend on ignored TBD files.

## Product boundary and early decisions

The baseline is a local operator-controlled product: a CLI entrypoint with local artifact browsing, consuming shared ADL Runtime/provider contracts. No hosted customer service, separate authentication platform, multi-tenant system, or customer-scale deployment is implied. OBS-S3 may host the existing Observatory web client as one bounded static S3/CloudFront sidecar without changing this product boundary. Plan the first implementation within this repository; a separate product repository is a later explicit boundary decision, not an unrecorded bootstrap choice. The [creation selections](CREATION_SELECTIONS_v0.92.2.md) bind `adl codefriend` and concrete source ownership paths; issue owners recheck those paths and ownership before execution. Any change to this baseline is a tracked design revision.

Local checkout, GitHub revision/PR and CI inputs normalize to the same immutable repository packet. The initial proving language is Rust on ADL and the bounded licensed Vector Rust fixture selected in [creation selections](CREATION_SELECTIONS_v0.92.2.md); unsupported language-specific analysis reports unknown rather than inventing architectural certainty. Generic inventory may include other text formats without implying equivalent analysis coverage. The creation selections pin the external revision, license, limits and fixture subset; qualification must verify those exact inputs before execution. No external fixture download or provider use is authorized by this plan.

## Shared finding and run contract — CF-EVIDENCE owner

CF-EVIDENCE must deliver the production evidence admission/store boundary, including its versioned schema and executable review/memory/renderer conformance fixtures, before those consumers begin implementation. Adapter admission must exercise identity, provenance, redaction, retention and deletion through that boundary; schema or fixture delivery alone cannot close the task. Evidence identity and finding identity are separate. The minimum shared semantic fields are:

| Record | Required semantics |
|---|---|
| Run | schema version, run identity, canonical repository and revision, scope digest, included/excluded surfaces, input provenance, selected lane contract versions, provider route identity without credentials, completion state and failures |
| Evidence | stable object identity, content digest, repo-relative location, source revision, redaction disposition, retention/deletion policy and provenance |
| Finding | schema/version, stable match identity, emitting perspective and rule identity, severity with rationale, confidence or unknown, evidence references, explicit inference, reviewed scope and limitations |
| Comparison | baseline/current run identities and contract versions, match reason, added/resolved/changed/unchanged or not-comparable outcome |
| Publication | exact run/finding set and artifact-manifest digests, rendering versions, claims/non-claims, approval scope and approved target; withheld/invalidated state |

Finding IDs must not use mutable display prose or line number alone. Identity collisions, changed scope, missing evidence and incompatible schema never silently merge findings. A finding absent from a partial or narrower run is not automatically resolved: compare only compatible completed coverage, otherwise emit not-comparable with a reason. CF-EVIDENCE owns canonicalization and identity test vectors; CF-MEMORY owns matching implementation and ambiguity handling.

Run outcomes distinguish complete, incomplete, failed, cancelled and withheld publication. Failed/partial lanes do not satisfy the four-perspective review requirement. The same fixture must be consumed by CF-REVIEW, CF-MEMORY and CF-UX without private schema forks. Scope, finding, renderer or target changes invalidate applicable publication approval.

## Independent review and synthesis — CF-REVIEW and CF-SYNTHESIS owners

All four perspectives receive the same scoped, redacted evidence and their perspective instructions. A perspective must commit its result before seeing peer findings. Synthesis is the first step to consume all results; it retains source attribution, disagreements and severity rationale. Independence concerns information flow and reviewer roles, not a requirement to buy four vendors. Test that no lane input contains peer output and that contradictory findings survive synthesis with an explicit disposition.

Repository text, embedded instructions and retrieved artifacts are evidence, not execution or publication authority. Redact before retention/model use, keep provider credentials out of packets, and preserve no-source-mutation behavior. CF-EVIDENCE and CF-REVIEW include hostile instruction/redaction negatives; CF-UX binds approval to exact artifacts and target. This is a product trust boundary, not a claim that every possible secret or prompt injection is prevented.

## Decisions versus implementation detail

The tracked baseline above resolves the former CLI/web/repository/integration ambiguity for planning. WP-01 recorded concrete paths, the external fixture and supported provider route in [creation selections](CREATION_SELECTIONS_v0.92.2.md). The created issue contracts consume these selections; parallel implementers must preserve them or record an explicit revision. Selection resolves creation scope but does not prove the installed product or authorize live provider use. Closed merged #717/#718 are predecessor contracts for this milestone.

## Source disposition

The prior TBD scheduling reconciliation names local source documents as active inputs. From this correction onward, this tracked contract, the milestone feature/specification documents, and the selected issue snapshot are the adopted execution-planning requirements. Local product brief, technical architecture, MVP build plan, review packet and security model remain historical rationale. Their unadopted scope is not automatically admitted. In particular, broader tournaments, autonomous source changes and customer-scale service work remain outside Beta 1. #523's retained source manifest binds the source versions reviewed.

## Complete consumer ownership

CF-REVIEW executes isolated perspectives; CF-SYNTHESIS merges their attributed results; CF-REMEDIATE and CF-TESTPLAN each generate their own usable plan from synthesis. CF-UX enforces exact-artifact approval while CF-RENDER-MD, CF-RENDER-HTML and CF-RENDER-PDF each deliver a working exporter. PLAT-MEMORY connects the actual second-review path to Memory Palace. The [atomic task contracts](ATOMIC_TASK_CONTRACTS_v0.92.2.md) bind all split tasks and eleven strengthened closure requirements; an interface contract does not count as a completed implementation.
