# #919 Documentation review

Candidate: `5c4a6149771c637f3c805985b86231077965eab4`; base `c9cdb2f13ff77a06dcc79ad4e4c332f444b2e14f`.

**CHANGES REQUIRED — four P2 findings.**

## DOC-001 — Route pending qualification to its approved v0.93 successors (P2)

`docs/milestones/v0.92.2/README.md:3`

Current status headers still describe #915 as pending, while retained SPRINT10_DEFERRAL records it CLOSED/NOT_PLANNED and routes remaining qualification to #1148/#1149/#1150 in v0.93. Current overlays and feature acceptance still send readers to the closed predecessor and imply its completion blocks this milestone rather than preserving the approved deferral and prelaunch gate.

Trigger: A reviewer/operator follows current milestone status and acceptance ownership for incomplete Beta1 qualification.

Fix: Update current status/owner overlays consistently to incomplete and deferred, preserving original #915 historical requirements and the #1150 pre-Beta1-launch qualification gate. Do not relabel as PASS or rewrite historical evidence.

Owner: #921 / milestone documentation and #917 handoff owner. Acceptance: Accurate current release gate and successor routing.

Validation: Independent static source comparison; no live tracker query. Frozen candidate contains both contradictory claims.

## DOC-002 — Remove native-only timestamps from the semantic disposition example (P2)

`docs/csdlc-v3/NO_PR_CLOSEOUT.md:16`

The documented finish ISSUE --disposition JSON includes two timestamp fields rejected by the strict semantic Disposition parser, so copying the sole example produces intent_finish_disposition_invalid instead of terminal reconciliation.

Trigger: Operator follows the current documented command and JSON construction procedure.

Fix: Remove caller timestamps and explain they are authenticated internally; validate the exact documented object against the semantic parser.

Owner: #921 / C-SDLC operator documentation owner. Acceptance: Executable operator and closeout instructions.

Validation: Static exact parser/schema comparison; no lifecycle command executed. Generated roff parity passed for all34 pages.

## DOC-003 — Use semantic plan and change schemas in the operator walkthrough (P2)

`docs/csdlc-v3/man/manual.json:1700`

The installed workflow shows prepare --plan and edit --changes but immediately directs the operator to construct LocalPreparationRequest and design-edit.card_updates with expected_lifecycle_digest. These are retired/native-owner shapes; semantic commands require intent_plan and strict intent_changes with cards/amendment. Following the fresh-user walkthrough cannot initialize/edit the issue as described.

Trigger: Operator follows the current documented command and JSON construction procedure.

Fix: Rewrite workflow step1 around current semantic plan/changes examples; retain native request details only under clearly separate compatibility reference. Regenerate roff and check executable schema parity.

Owner: #921 / C-SDLC operator documentation owner. Acceptance: Executable operator and closeout instructions.

Validation: Static exact parser/schema comparison; no lifecycle command executed. Generated roff parity passed for all34 pages.

## DOC-004 — Include validator replacement in the published intent schema (P2)

`docs/csdlc-v3/intent-request.schema.json:289`

The changes definition rejects validators as an unknown property and its oneOf only permits cards or publication. The plan definition additionally requires program=cargo. Supported requests accepted by native source and documented in INTENT_COMMANDS are therefore rejected by the published machine-readable contract.

Trigger: A schema-driven client validates the documented semantic edit containing validators, or prepares a supported python3/git/manual-review declaration.

Fix: Bring the schema into parity with every supported intent content variant; add positive parity checks for validator replacement and admitted non-cargo declarations alongside negative guards.

Owner: #921 / C-SDLC contract owner. Acceptance: Machine-readable operator contract and amendment recovery.

Validation: Static comparison of exact candidate schema, native deserializer and current guide; no lifecycle execution.

## Coverage and independence

Applied repo-review-docs. This reviewer did not author the candidate or #917 handoff. The same reviewer supplied security and provider lanes; these are different review scopes, not three independent reviewers. Per-path authored coverage and explicit cohort/cross-lane dispositions are in docs-coverage.json (317 assigned rows). Current milestone, all feature docs, #917 handoff/validator, #918 guide, Runtime/provider operational docs, UTS guides, C-SDLC operator contracts and all manual-source page paragraphs were inspected. All generated 34 roff pages received generator parity checking, not a separate rendered visual inspection. Historical drafts/retained receipts are not current acceptance evidence.

The original #917 frozen handoff check reports document_inventory_mismatch and changed RELEASE_NOTES bytes when run against #918. This is expected checkpoint preservation, explicitly covered by #918 REVIEW_GUIDE; it is not an added product finding. The #918 finalization packet validator passes while final acceptance and release remain withheld.

## Checks performed

- `python3 docs/milestones/v0.92.2/evidence/issue-917/validate_handoff.py --self-test`: exit 1; 129 documents, 69 tasks, 24 prerequisites, six negative cases; later-stage checkpoint drift classified above.
- `python3 docs/milestones/v0.92.2/evidence/issue-918/validate_packet.py --self-test`: pass, 12 artifacts and 25 negative cases; no final acceptance/release claim.
- `python3 docs/csdlc-v3/man/render.py --check`: pass, 34 pages.

## Limits

No product or lifecycle execution, provider calls, remote-link freshness checks, cloud operations, or credential reads. Earlier tool-side restriction remains honored. Cloud documentation/assurance is explicitly unreviewed. ADR/decomposition coverage is recorded by the complementary reviewer in adr-coverage.json. OpenAPI semantic changes, all five milestone JSON/YAML contracts and all three planning validators are now reviewed; duplicate completion-contract objects were equality-checked against full-read YAML definitions. Lead owns CodeFriend and v0.93 documentation as separately recorded; this report does not convert another reviewer’s pending work to PASS. No product files or GitHub state changed.

## Final contract verification

Pure local atomic-contract validation passed, including 44 negative cases and equality checks on all 37 completion contracts. Pure sprint assignment validation passed with 28 negative cases. These checks called reviewed functions directly and did not launch retained birth-evidence scripts, contact GitHub, or mutate lifecycle state. OpenAPI review covered every semantic JSON change while excluding formatting-only churn. No additional actionable finding arose from these final contract checks.
