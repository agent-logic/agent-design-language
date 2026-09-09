# v0.92.2 Execution Readiness

Status: the new execution wave remains gated. Existing admitted issues #717, #718 and #720 use their own typed readiness; #718 is urgent and need not await these new-wave opening gates.

## Opening Gates

- [ ] For new issue creation, prior v0.92.1 closure and release truth are reconciled; existing admitted issues are exempt from this new-wave creation gate.
- [ ] v0.92.1 has delivered the shared Runtime/provider/C-SDLC/Observatory foundations actually required by admitted Beta 1 work, or each remaining dependency has an explicit non-blocking boundary.
- [ ] This complete planning package has passed focused validation and independent review.
- [ ] The operator has authorized issue creation.
- [ ] WP-01 has assigned canonical issue numbers without changing dependency order or release-tail semantics.
- [ ] Each implementation issue has owned paths, acceptance criteria, PVF lanes, stop conditions, and non-goals.

## Parallel Readiness

CF-SHELL and CF-ADAPTER become ready after WP-01. CF-COG, CF-GOV, CF-REVIEW, and CF-MEMORY become ready after the evidence contract merges and may run in parallel. No track waits for individual closeout bookkeeping; it waits only for its declared merged authority.

PLAT-UTS, PLAT-RUST, OPS-AWS, PUB-MEDIUM, PUB-CSDLC, and SPEC-RETEST become ready after WP-01. PLAT-PROVIDER additionally waits for merged v0.92.1 issue #622, and PLAT-MLX waits for PLAT-PROVIDER. PLAT-MEMORY waits for CF-EVIDENCE and CF-MEMORY. CF-INTEGRATE waits for the product tracks, shared provider definitions and the admitted production memory consumer. TAIL-01 separately converges all other admitted supporting and existing issues; article/paper/inventory completion does not block product integration.

## Fail-Closed Conditions

Execution pauses for missing issue authority, scope conflict, unavailable evidence/privacy controls, provider or Runtime contract ambiguity, or a required planning surface that cannot be resolved context-free. A deferred track is not a blocker unless explicitly admitted.

## Early contract and existing-issue gates

CF-EVIDENCE merges the shared finding/run contract and review/memory/renderer conformance fixtures before CF-REVIEW, CF-MEMORY or CF-UX executes against it. #718 starts first among ready existing work; #717 and #720 are independent except for explicit owned-path collision checks. Their linked issue acceptance remains authoritative, including #718 model-generated reply proof and its separately bounded provider authorization. A missing GitHub milestone assignment alone does not reclassify an explicitly admitted issue as backlog.

## First SIM sprint

SIM-01 has its own readiness and can start alongside Runtime without WP-01 or unrelated prior closeout. SIM-02 through SIM-07 follow in order; SIM-UMBRELLA coordinates from startup and completes after SIM-07. Dedicated sprint issue creation and disjoint owner assignment precede execution. The CodeFriend opening gates above do not apply to this independently admitted sprint. Writer-pause activation requires separate explicit authorization. TAIL-01 requires the completed umbrella; CF-INTEGRATE does not.
