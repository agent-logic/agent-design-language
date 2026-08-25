# Decisions — v0.92.1

1. **Repository authority first.** #432's reviewed implementation is merged and ancestral before the milestone operator creates WP-01 or an execution root consumes it. Administrative closeout is asynchronous. No tracked planning artifact depends on a local untracked path.
2. **The original six lanes remain intact, with three explicit additions.** Runtime v2/v3 decoupling, provider inference profiles, and the GCP six-resident sidecar are first-class planned tracks rather than hidden work inside another lane.
3. **Planning reconciliation does not execute the wave.** #316 may reconcile v0.92.1 and v0.92.2 planning truth, but it creates no execution issue and claims no implementation.
4. **UTS is a workload.** Distributed qualification may use a UTS test cycle, but does not create a second Runtime architecture.
5. **Hot reload starts stateless.** Strings, flags, limits, and templates may be reloaded; database pools, credentials, and authority-bearing objects require separate designs.
6. **Observatory data is authentic.** Invented, mocked, or status-only authority is prohibited from release proof.
7. **Runtime v4 triggers explicit replanning.** It is a risk and future input, not hidden milestone scope.
8. **CodeFriend follows.** v0.92.2 owns CodeFriend Beta 1; the product must reach integrated beta availability by v0.95.
9. **Issue creation begins with a viable future conductor.** Closed packets #149–#190 and closed #431 are reconciliation inputs, not active execution. Premature placeholders #433–#438 are closed, #439 is redundant, and none is used as authority. After this package lands, the milestone operator creates number-free WP-01; WP-01 then creates the remaining catalog.
10. **Four existing backlog issues are promoted.** #251, #122, #84, and #345 are active v0.92.1 scope. #251, #122, and #345 may execute in parallel; #84 preparation may overlap them, but its final proof consumes #251 and #122.
11. **The release tail follows the established ten-step standard.** Quality, docs/release truth, publication finalization, internal review, external review, remediation/preflight, next-milestone planning, closeout planning, next-milestone review, and ceremony remain distinct serial issues.
12. **Runtime generations are separated before v4.** DEC-01 owns v2/v3 authority separation and compatibility proof; Runtime v4 stays deferred to a later milestone.
13. **Provider configuration is shared and bounded.** PROV-A defines the common profile and deterministic Ollama materialization; PROV-B may compare local-model results only as non-authoritative shadow execution. #457 is historical provenance, not a live dependency.
14. **GCP is a sidecar, not a replacement.** DRT-D follows DRT-C, repeats the exact six-resident contract, and requires separate provider identity, cost, and cleanup proof. It does not execute #269.
15. **Integration provenance is not interchangeable.** #188 routes to INT-01 and TAIL-01, #190 routes to TAIL-07, and #189 routes only to TAIL-10.
16. **Dependencies consume merges, not closeout.** Downstream work may require reviewed merged authority and an explicit product gate, but never waits for an individual issue's finish receipt, cleanup, or administrative closeout.
