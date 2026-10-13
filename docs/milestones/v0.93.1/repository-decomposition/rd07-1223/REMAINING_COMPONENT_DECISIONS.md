# Remaining #1223 component decisions

This is one consolidated proposal, not additional deletion authority. Every original ID appears exactly once. Existing mappings and consumer findings are reused; absence from the 429-row consumer packet is explicitly unresolved.

651 csdlc-v3 test rows = 630 prior generic holds +16 no proven mapping +5 changed-source rollback holds. 483 cognitive-sdlc adl/tools rows =236 prior unmapped holds +226 generic holds +19 protected contracts +2 applied repairs. Prior hold categories and newer mapping evidence are separate dimensions.

| Component | Original rows | Proposal |
|---|---:|---|
| agent-logic-infrastructure-active-tool-contracts | 31 | Retain current ADL validation/installation/control-plane consumer until its replacement is implemented; owner assignment alone does not remove this consumer. |
| agent-logic-infrastructure-historical-docs | 1 | Retain the reusable operational guide and its ADL planning navigation; the grouping does not imply historical-only instructions. |
| agent-logic-infrastructure-infrastructure | 212 | Propose whole Terraform/deployment components with their scripts and docs after independent construction validation and exact consumer routing. No cloud apply or deletion is part of this decision. |
| agent-logic-infrastructure-mapped-tools | 118 | Propose complete tool-family retirement with caller routing to its qualified successor; changed mappings require behavioral review. Keep tools still required to validate retained ADL code until the caller cutover. |
| agent-logic-infrastructure-mixed-code | 2 | Resolve through the CLI/library/manifest component partition; preserve public behavior and qualify owner command adapters before deleting private implementation or tests. |
| agent-logic-infrastructure-other-consumers | 29 | Reconcile component ownership and current callers; propose exact removal only with successor qualification or explicit retained ADL contract disposition. |
| agent-logic-infrastructure-product-docs | 24 | Propose successor-owned active instructions plus ADL migration links; inspect each current inbound link and retain ADL-specific contracts/history. Unmapped pages require explicit destination or retention rationale. |
| agent-logic-runtime-active-tool-contracts | 4 | Retain current ADL validation/installation/control-plane consumer until its replacement is implemented; owner assignment alone does not remove this consumer. |
| agent-logic-runtime-historical-docs | 3 | Propose retaining ADL planning/history with explicit successor ownership links. Do not reinterpret historical examples as active production commands. |
| agent-logic-runtime-infrastructure | 4 | Propose whole Terraform/deployment components with their scripts and docs after independent construction validation and exact consumer routing. No cloud apply or deletion is part of this decision. |
| agent-logic-runtime-mapped-tools | 264 | Propose complete tool-family retirement with caller routing to its qualified successor; changed mappings require behavioral review. Keep tools still required to validate retained ADL code until the caller cutover. |
| agent-logic-runtime-mixed-code | 904 | Resolve through the CLI/library/manifest component partition; preserve public behavior and qualify owner command adapters before deleting private implementation or tests. |
| agent-logic-runtime-other-consumers | 254 | Reconcile component ownership and current callers; propose exact removal only with successor qualification or explicit retained ADL contract disposition. |
| agent-logic-runtime-product-docs | 108 | Propose successor-owned active instructions plus ADL migration links; inspect each current inbound link and retain ADL-specific contracts/history. Unmapped pages require explicit destination or retention rationale. |
| already-applied | 83 | Preserve existing approved disposition; no additional removal decision. |
| codefriend-historical-docs | 5 | Propose retaining ADL planning/history with explicit successor ownership links. Do not reinterpret historical examples as active production commands. |
| codefriend-mapped-tools | 81 | Propose complete tool-family retirement with caller routing to its qualified successor; changed mappings require behavioral review. Keep tools still required to validate retained ADL code until the caller cutover. |
| codefriend-mixed-code | 197 | Resolve through the CLI/library/manifest component partition; preserve public behavior and qualify owner command adapters before deleting private implementation or tests. |
| codefriend-other-consumers | 26 | Reconcile component ownership and current callers; propose exact removal only with successor qualification or explicit retained ADL contract disposition. |
| codefriend-product-docs | 41 | Propose successor-owned active instructions plus ADL migration links; inspect each current inbound link and retain ADL-specific contracts/history. Unmapped pages require explicit destination or retention rationale. |
| codefriend-unmapped-tools | 52 | Require explicit successor mapping or a specific obsolete-tool retirement rationale, then repair executable callers as one tool family. No byte-correspondence removal is proposed for these rows. |
| cognitive-sdlc-active-tool-contracts | 19 | Retain current ADL validation/installation/control-plane consumer until its replacement is implemented; owner assignment alone does not remove this consumer. |
| cognitive-sdlc-historical-docs | 6 | Retain the active ADL milestone ordering standard as current policy; classify remaining plans by their explicit status and preserve navigation. |
| cognitive-sdlc-mapped-tools | 258 | Propose complete tool-family retirement with caller routing to its qualified successor; changed mappings require behavioral review. Keep tools still required to validate retained ADL code until the caller cutover. |
| cognitive-sdlc-mixed-code | 4 | Resolve through the CLI/library/manifest component partition; preserve public behavior and qualify owner command adapters before deleting private implementation or tests. |
| cognitive-sdlc-other-consumers | 52 | Reconcile component ownership and current callers; propose exact removal only with successor qualification or explicit retained ADL contract disposition. |
| cognitive-sdlc-product-docs | 179 | Propose successor-owned active instructions plus ADL migration links; inspect each current inbound link and retain ADL-specific contracts/history. Unmapped pages require explicit destination or retention rationale. |
| cognitive-sdlc-unmapped-tools | 204 | Require explicit successor mapping or a specific obsolete-tool retirement rationale, then repair executable callers as one tool family. No byte-correspondence removal is proposed for these rows. |
| csdlc-authority-records | 3 | Retain immutable ADL-bound authority and reconciliation records. They are repository state, not duplicate product implementation. |
| csdlc-v2-history | 25 | Retain immutable transition/rollback authority history. No operational v2 fallback is introduced. |
| csdlc-v2-source-tests | 86 | Propose whole retired crate removal only after inventorying explicit rollback/transition consumers and replacing any supported producer path; keep operator history separate. |
| csdlc-v3-source | 78 | Propose whole-crate retirement after ADL owner installer and CI use the qualified external package; demonstrate Node/publication parity and include noncandidate current source. |
| csdlc-v3-tests | 651 | Propose removal only with the complete local C-SDLC crate consumer cutover, not test-only deletion. Reconcile changed/unmapped tests with successor regression coverage first. |
| excluded-website | 3 | Retain unchanged: operator excludes website repositories from this exercise. |
| public-prompt-contracts | 122 | Retain active prompt registry/schema contracts and historical version identities; replace source implementation without removing ADL document contracts. |
| repo-local-policy | 1 | Retain as ADL-specific execution policy; successor policy is not a substitute. |

## Execution order

1. Preserve the applied 70 removals and existing consumer repairs.
2. Retain ADL-bound policy, authority history and public prompt contracts with explicit reasons.
3. Cut over the complete C-SDLC installer/CI/source/test component after installed ADL owner qualification; reconcile changed and unmapped tests.
4. Resolve unmapped tool families using retained findings and actual caller audit; supply explicit successor or retirement rationale.
5. Route active product docs and executable callers together; retain ADL historical context.
6. Complete Runtime/CodeFriend CLI partition, then submit the exact combined removal/caller delta for one scoped decision.

## Remaining evidence limits

Producer build success does not discharge ADL consumer acceptance. The packet does not claim all consumer references have been audited, all holds resolved, or all 4,134 files should be deleted. Native review/publication/finish and final public-package proof remain required.

## Resolved retention

`RESOLVED_RETENTION.json` resolves 139 original rows as required retained
ADL contract/state/history or excluded website assets: 122 planning/prompt contract
files, three v3 authority records, six historical cutover/rollback records, one
registry-referenced native card shape, one repository worktree policy, and three
website assets, plus one active ADL milestone ordering standard, one reusable
operational guide, and its ADL planning navigation page. Each row records its
specific reason and evidence. The operational guide and active standard are
retained for their current purpose, not classified as immutable history.

The remaining 18 v2 operator skill/example files are still unresolved; their location
in an operator directory alone does not establish immutable-record status. Active
tool holds also remain unresolved. This changes dispositions only, not source bytes.
