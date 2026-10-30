# Sprint 2 execution validation contract

Preparation review: Planning #12, turn `01a15734-a55f-7bd2-899b-5c05efe23150`, accepted concrete execution contracts without adding wrappers during preparation. Validator registration is the child's first execution setup step before native proof/publication. Current git-diff registration proves hygiene only. No product tests have run here.

## CodeFriend #120

Entry inputs: an operator-agreed candidate and representative repository corpus; exact agent, gateway and website identities; repository revisions; platform/configuration; agreed scenario denominator. Selecting these is the assessment's first work, not a prerequisite to preparing this plan.

Create the four assessment outcomes under `docs/codefriend/beta1/assessment-120/`: candidate/corpus register, scenario evidence matrix, prioritized gap register, operator acceptance decision. Use machine-readable records alongside readable assessment prose. Each scenario needs a stable ID, expected/actual result, status, candidate/corpus reference and local evidence reference. Cover report/export readability, login, consent, pairing, first review, results and recovery, retaining failed, blocked and skipped cases. Each gap needs reproduction evidence, impact, priority, measurable acceptance and existing issue mapping (#121/#123/#125/#126). Record an explicit operator decision reference; never infer approval from a passing checker.

Execution owner supplies and registers `python3 tools/validate_beta1_assessment.py` against those agreed records. Its checks must reject missing/duplicate identities, invalid evidence links, counts inconsistent with the agreed denominator, gaps without reproduction or routing, and an absent operator decision. It must report the actual checked denominator and distinguish evidence completeness from product success. Focused positive/negative fixtures must show those failures are detected. The validator and its concrete schema are authored with the assessment's actual inputs, not invented during sprint preparation. Native success marker: `beta1 assessment evidence complete`; only print it after all completeness checks pass. Independent review inspects actual readable outputs and journey evidence; mechanical checks cannot prove readability or supply human acceptance.

## Enterprise-security #9

Entry inputs: accepted #120 output identities and denominator; supported Runtime interface; authenticated archive matching the consumer and runner platform. Retained baseline `tools/ci/runtime-policy-input.json` declares Linux x86_64, Runtime source `97f30f046cefbf8aac86e81f91f0ec5837652768`, artifact `11132075121`, run `36789819771`, payload SHA256 `4ab1e60cf0ebb263858ea16460492f65c3b97961744450ff4f2ce846934bfa4c`. Verify these against current accepted inputs before reuse; this is not a macOS artifact. No download or infrastructure provisioning is required during preparation.

On the compatible Linux runner, with cached Cargo dependencies, run:

```text
python3 tools/validate_rd09.py --runtime-interface ARCHIVE --sha256 4ab1e60cf0ebb263858ea16460492f65c3b97961744450ff4f2ce846934bfa4c
```

ARCHIVE is the verified local artifact, not a source checkout. The existing baseline requires four adapter unit tests and six independently installed conformance cases and emits `adl.enterprise.conformance.v1`. Add only missing cases from accepted #120 inputs and #9 invariants: principal/tenant/repository/service/paired-agent boundaries, least privilege, per-message authorization, revocation, redaction, audit integrity, tenant isolation, deletion, fail-closed behavior, implicit internal trust and connection-as-permission rejection. Preserve Runtime final authority; adapter restrictions must never add grants. Update the expected denominator deliberately when cases are added; passing the historical ten cases alone cannot prove new requirements.

Before proof, register an admitted executable route to this exact command. The current native Python adapter only accepts a tracked script with no arguments or `--self-test`; the existing validator needs arguments. Resolve with a narrow tracked no-argument entrypoint using explicit issue-local artifact inputs if the admitted owner still has that limit. The entrypoint must verify the artifact identity, invoke the existing validator, preserve failures and report actual counts. Do not substitute manual-review or git-diff PASS. This routing is execution setup, not an excuse to claim preparation ran installed proof.

Acceptance requires all required current cases to run and pass, retained exact identities/counts and independent review. Failed, missing, skipped or unsupported required cases block issue acceptance. Missing Runtime functionality becomes a bounded dependency. Existing CI cache population may use its normal dependency lane; no paid provider or cloud execution is authorized here.

## Preparation versus dispatch

The preparation can be accepted with execution owners unassigned and security waiting for #120 outputs. Assign one owner before dispatch. Complete native child cards before declaring the preparation complete. Do not run product implementation, publish an umbrella-closing PR, or claim sprint acceptance from this packet.

## Current execution interpretation (2026-10-20)

The preparation-only restrictions above describe the original preparation phase. The operator subsequently authorized execution with Worker #30 and the minimal owner prerequisite slices required for the full security #9 outcome. This is not pending approval. CodeFriend #120 is delivered and its assessment is accepted; see `evidence/codefriend-120-reviewed-assessment.json` and `evidence/codefriend-120-terminal-finish.json`.

Runtime #189 supplies the missing installed baseline evaluator interface. Its delivery and installed consumer proof remain required; source tests alone cannot satisfy security #9. The first consumer enforcement slice includes trusted principal/tenant/repository resolution and cross-tenant refusal. Minimal security #10/#12/#14/#15 work may precede #9 acceptance, while broader downstream issue completion remains downstream. CodeFriend #127 contributes bounded installed callsite proof without becoming a whole-issue predecessor. See `evidence/sequencing-review.json`.

The original Runtime artifact identifiers above are historical baseline references. Execution must record the recovered or newly delivered producer identity and verify the exact accepted bytes; availability of a newer artifact does not by itself authorize a behavior change.

Rendering #121 proceeds alongside security, except where a concrete export security boundary intersects. Only the new #130 templates are selected. Six unfinished examples support preliminary pipeline measurements; they do not establish final acceptance of the 25 document types and their declared formats. #130 remains read-only. The planned Journal type is tracked separately. Require generated-output checks, variable-content pagination and overflow checks, isolation checks, production integration measurements, and the remaining format-specific evidence before claiming rendering acceptance. Authored Markdown inventory is not generation timing; PDF tagging is not accessibility acceptance. See `evidence/new130-sample-rendering-observation.json`.

Sprint acceptance and umbrella closeout still require full security #9 evidence, independent review, native delivery, and truthful reconciliation of the applicable rendering work and final-template dependency. Rendering reconciliation means retaining completed measurements and explicitly routing remaining #121 and final-template acceptance work; it does not require all final templates before Sprint 2 can close. CodeFriend launch acceptance still requires the applicable final-template validation. No source-only or adapter-only result substitutes for the full security #9 boundary.
