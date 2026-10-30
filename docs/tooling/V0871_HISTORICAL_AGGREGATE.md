# Retired v0.87.1 runtime aggregate

The ADL source-tree lifecycle, environment, trace, failure, recovery, restart,
D8 review, D9 integration, D10 documentation, D11 quality, D12 release and WP-13
suite wrappers are retired together with their shared helper and four aggregate
tests. Their complete original and last-current sources remain in Git and the
#1223 rollback archive. This is explicit historical retirement, not a claim of
Runtime replacement equivalence or new successful execution.

The shared helper already refuses the sunset v1 mock-workflow route. The suite
calls that route before its remaining children; the D8 test also names the retired
private tooling dispatcher. Retaining those aggregate entrypoints would advertise
an execution contract that ADL no longer provides.

The retired tests checked environment schema, lifecycle ordering, trace manifest,
failure/crash classification, resume-ready continuity, plan-hash restart guards,
D8 manifest/README and D6/D7 ordering, D9/D10/D11/D12 identities, quality results,
and the complete WP-13 proof-file list. These assertions remain historical;
they are not replaced with a generic successful stub or credited to green CI.

Runtime's separately qualified operator and state demos retain their own bounded
mock-consumer results. Those results do not prove this aggregate or its recovery,
quality, release and private review-CLI behavior. Prior repaired caller preimages
are preserved, including their explicit Runtime source/binary selection.

Individual local/mock/HTTP/ChatGPT provider demos, their parallel tests, both
multi-agent demos and their tests, provider helpers, public coverage/CI tools,
and existing skill/demo documents remain under separate dispositions. Removing
this aggregate does not retire their assertions, establish live provider proof,
or authorize AWS/provider execution. Historical milestone and review records
remain unchanged and can be inspected at their original Git revisions.
