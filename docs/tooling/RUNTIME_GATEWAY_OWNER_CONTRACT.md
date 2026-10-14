# Runtime gateway contract routing

The retained WP-07 selector invokes `python3 adl/tools/run_runtime_gateway_owner_contract.py`.
Set `ADL_RUNTIME_SOURCE_ROOT` to an absolute Runtime Git checkout containing
revision `7d0db620ad79c50ff5d8fabf0fb543183583d05c` and `RUNTIME_TEST_CSM` to an absolute executable CSM binary.
Both are mandatory. Missing inputs fail before execution; this route never builds
Cargo or selects an ADL target binary.

The adapter reads and SHA-256 checks the unchanged owner gateway test and validator
from that revision's Git objects, then copies only those two resources into a
private temporary fixture. The original test creates its own fake AWS and curl
commands. Its original success, sixteen route obligations, authorization failures,
malformed requests and redaction assertions execute against the selected binary;
the original exit status is preserved. Binary selection is explicit operator input,
not authenticated artifact provenance or a promise that that binary will pass.

Runtime164 CI run38455831412 qualified the original gateway assertions and the
real CSM/Guardian cross-binary configuration cases: matching configuration,
changed init content, incompatible binary generation and malformed receipt.
Those are existing results, not new execution by this adapter. Earlier Runtime162
failures remain recorded. Cross-binary resources and assertions live in Runtime;
ADL's retired refusal is removed rather than counted as successful validation.

The separate live gateway producer and summary validator remain in ADL pending
an independent disposition. This mock contract does not qualify their AWS resource
mutation, live weather routing, provider or deployment behavior. Historical recipes
and full original/current preimages remain in Git and #1223 rollback custody.
