# Sprint Plan — v0.92.1

## Opening

- Close #432 with canonical, ancestral repository-authority proof.
- Preserve closed #431 as planning provenance without assigning it future work.
- The merge makes WP-01 eligible but does not create it. When the operator declares v0.92.1 ready, the operator creates number-free WP-01; WP-01 creates the remaining planned issues and verifies their exact numbering and dependency links.

## Sprint sequence

The original execution lanes remain, with cloud move-in, cross-cloud Terraform conversion, bounded Rust refactoring, Runtime v2/v3 decoupling, provider profiles, and GCP portability admitted as explicit tracks.

1. **Sprint 0 — opening and wave creation:** #432 and the reviewed planning-package merge establish eligibility; the operator later declares v0.92.1 ready and creates WP-01; WP-01 then creates the remaining wave. #431 and #457 are consulted only as planning/provider provenance.
2. **Sprint 1 — independent foundations:** CORP-A and CORP-B run serially; HOT-01, DEC-01, PROV-A, and RUST-01 may run in parallel after the opening gate.
3. **Sprint 2 — parallel cloud foundations:** AWS-A through AWS-E and GCP-A through GCP-D advance through their provider-local ordered results; #122/#251/#84/#345 remain existing independently owned issues.
4. **Sprint 3 — cloud convergence:** XCL-01 follows AWS-E and GCP-D; AWS-F consumes AWS-E and #122; AWS-G follows AWS-F and XCL-01; GCP-E follows GCP-D under separate paid authority.
5. **Sprint 4 — corporate acceptance:** CORP-C consumes CORP-A/B, AWS-G, and GCP-D; CORP-D follows CORP-C.
6. **Sprint 5 — C-SDLC v3 foundation:** V3-A, V3-B, V3-C.
7. **Sprint 6 — C-SDLC v3 delivery and cutover:** V3-D, V3-E, V3-F.
8. **Sprint 7 — distributed Runtime qualification:** DRT-A, #345, DRT-B, DRT-C, then DRT-D after GCP-E and XCL-01; every paid proof remains operator-gated.
9. **Sprint 8 — product lanes:** existing podcast #51/#261-#264/#342; Observatory OBS-A; and #251, #122, #345, plus #84 preparation may run in parallel. #84 final Unity proof consumes #251 and #122, and OBS-B consumes OBS-A plus that completed Unity/public-exposure convergence.
10. **Sprint 9 — provider comparison and convergence:** PROV-B follows PROV-A; INT-01 waits for every root named by the issue wave, then TAIL-01, TAIL-02, TAIL-03. #188 informs convergence and quality admission.
11. **Sprint 10 — review and remediation:** TAIL-04, TAIL-05, TAIL-06, strictly serial.
12. **Sprint 11 — handoff and release:** TAIL-07 carries #190 successor intent, TAIL-08, TAIL-09, then TAIL-10 carries #189 ceremony intent.

Each lane owns its issues, validations, review, and closeout. The conductor tracks dependencies and collisions but does not absorb implementation. The catalog is a creation plan, not evidence that any unnumbered issue exists.

## Integration and release

- Review cross-lane API, authority, privacy, and evidence dependencies.
- Rebaseline explicitly if Runtime v4 becomes canonical.
- Hand remaining CodeFriend work to v0.92.2 CodeFriend Beta 1, retaining the v0.95 integrated-beta deadline.
