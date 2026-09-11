# v0.92.2 Execution Readiness

Status: the new execution wave remains gated. Existing admitted issue #720 uses its own typed readiness. Closed merged #717 and #718 are predecessor inputs rather than new-wave rows.

## Opening Gates

- [ ] For new issue creation, prior v0.92.1 closure and release truth are reconciled; existing admitted issues are exempt from this new-wave creation gate.
- [ ] v0.92.1 has delivered the shared Runtime/provider/C-SDLC/Observatory foundations actually required by admitted Beta 1 work, or each remaining dependency has an explicit non-blocking boundary.
- [ ] This complete planning package has passed focused validation and independent review.
- [ ] The operator has authorized issue creation.
- [ ] WP-01 has assigned canonical issue numbers without changing dependency order or release-tail semantics.
- [ ] Each implementation issue has owned paths, acceptance criteria, PVF lanes, stop conditions, and non-goals.

## Parallel Readiness

CF-ADAPTER opens local ingestion; CF-ADAPTER-GITHUB and CF-ADAPTER-CI follow it. CF-EVIDENCE admits local packets. CF-REVIEW follows CF-EVIDENCE and RT-PROVIDER; CF-SHELL then proves a real review journey. CF-SYNTHESIS follows the lane runner; CF-REMEDIATE and CF-TESTPLAN follow synthesis. CF-COG provides structure, followed by CF-COG-IMPACT and CF-COG-RATIONALE; CF-COG-DRIFT additionally consumes CF-MEMORY. CF-GOV-CI follows the working local CF-GOV runner. CF-UX owns approval; CF-RENDER-MD consumes approval, synthesis and both action plans; CF-RENDER-HTML and CF-RENDER-PDF follow Markdown for parity. RT-COST precedes PLAT-PROVIDER (also consuming merged #622), then RT-PROVIDER/#855. PLAT-MLX and PLAT-PAIR consume PLAT-PROVIDER. PLAT-MEMORY consumes CF-EVIDENCE and CF-MEMORY. CF-INTEGRATE connects the complete product consumers, all ingestion routes, RT-PROVIDER and PLAT-MEMORY first. CF-PROOF then independently qualifies that installed integrated product on ADL and the pinned external repository. TAIL-01 consumes both completed results.

PLAT-UTS, PLAT-RUST, OPS-AWS, OPS-GCP, PUB-MEDIUM, PUB-CSDLC, SPEC-RETEST and ARCH-ADR keep their post-WP-01 readiness. OBS-S3 waits for WP-01 and OBS-LIVE plus separate apply authority. The exact graph, including Runtime qualification and C-SDLC decomposition ordering, is enforced by the atomic-task manifest. No row starts until its concrete selection gates and production consumer are bound. No task closes from an outline, unused library, placeholder UI or zero-scenario proof.

## Fail-Closed Conditions

Execution pauses for missing issue authority, scope conflict, unavailable evidence/privacy controls, provider or Runtime contract ambiguity, or a required planning surface that cannot be resolved context-free. A deferred track is not a blocker unless explicitly admitted.

OBS-S3 additionally pauses for an `agent-logic-admin` identity mismatch, an unexpected destructive Terraform plan, missing DNS/certificate authority, missing logging or security headers, failed invalidation, missing Runtime HTTPS/WSS reachability, or absent apply authorization. ARCH-ADR pauses rather than inventing a decision owner, source, status, or acceptance.

## Early contract and existing-issue gates

CF-EVIDENCE merges the shared finding/run contract and review/memory/renderer conformance fixtures before CF-REVIEW, CF-MEMORY or CF-UX executes against it. #720 remains independently executable. Closed merged #717 and #718 retain their v0.92.1 acceptance evidence, including #718 model-generated reply proof and separately bounded provider authorization; v0.92.2 consumes only those reviewed merged outcomes.

## First SIM sprint

SIM-01 has its own readiness and can start alongside Runtime without WP-01 or unrelated prior closeout. SIM-02 through SIM-09 follow in order; SIM-UMBRELLA coordinates from startup and completes after SIM-09. Dedicated sprint issue creation is complete as #866–#875; native execution readiness, binding and disjoint owner assignment still precede execution. The CodeFriend opening gates above do not apply to this independently admitted sprint. Writer-pause activation requires separate explicit authorization. TAIL-01 requires the completed umbrella; CF-INTEGRATE does not.

TAIL-10 waits for TAIL-09, OBS-S3 and ARCH-ADR. Final acceptance separately verifies authenticated Observatory deployment and the completed, source-grounded ADR set with explicit decision status. These obligations do not block CF-INTEGRATE or the early TAIL-01 quality gate, and do not change the canonical ten-step sequence.
