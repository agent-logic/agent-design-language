# Sprint Plan — v0.92.1

## Opening

- Close #432 with canonical, ancestral repository-authority proof.
- Preserve closed #431 as planning provenance without assigning it future work.
- The merge makes WP-01 eligible but does not create it. When the operator declares v0.92.1 ready, the operator creates number-free WP-01; WP-01 creates the remaining planned issues and verifies their exact numbering and dependency links.

## Sprint sequence

The original six execution lanes remain, with Runtime v2/v3 decoupling, provider profiles, and GCP portability admitted as explicit parallel tracks.

1. **Sprint 0 — opening and wave creation:** #432 and the reviewed planning-package merge establish eligibility; the operator later declares v0.92.1 ready and creates WP-01; WP-01 then creates the remaining wave. #431 and #457 are consulted only as planning/provider provenance.
2. **Sprint 1 — independent foundations:** CORP-A through CORP-D serially; HOT-01, DEC-01, and PROV-A may run in parallel after the opening gate.
3. **Sprint 2 — C-SDLC v3 foundation:** V3-A, V3-B, V3-C.
4. **Sprint 3 — C-SDLC v3 delivery and cutover:** V3-D, V3-E, V3-F.
5. **Sprint 4 — distributed Runtime qualification:** DRT-A, #345, DRT-B, DRT-C, then optional DRT-D GCP replay; every paid proof remains operator-gated.
6. **Sprint 5 — product lanes:** existing podcast #51/#261-#264/#342; Observatory OBS-A; and #251, #122, #345, plus #84 preparation may run in parallel. #84 final Unity proof consumes #251 and #122, and OBS-B consumes OBS-A plus that completed Unity/public-exposure convergence.
7. **Sprint 6 — provider comparison and convergence:** PROV-B follows PROV-A; INT-01 waits for every root named by the issue wave, then TAIL-01, TAIL-02, TAIL-03. #188 informs convergence and quality admission.
8. **Sprint 7 — review and remediation:** TAIL-04, TAIL-05, TAIL-06, strictly serial.
9. **Sprint 8 — handoff and release:** TAIL-07 carries #190 successor intent, TAIL-08, TAIL-09, then TAIL-10 carries #189 ceremony intent.

Each lane owns its issues, validations, review, and closeout. The conductor tracks dependencies and collisions but does not absorb implementation. The catalog is a creation plan, not evidence that any unnumbered issue exists.

## Integration and release

- Review cross-lane API, authority, privacy, and evidence dependencies.
- Rebaseline explicitly if Runtime v4 becomes canonical.
- Hand remaining CodeFriend work to v0.92.2 CodeFriend Beta 1, retaining the v0.95 integrated-beta deadline.
