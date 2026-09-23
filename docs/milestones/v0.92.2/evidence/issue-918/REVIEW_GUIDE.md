# #918 external-review publication packet

Status: reviewable draft; final acceptance pending. The original draft preceded #916/#917 integration; the current packet consumes their accepted assessment and documentation handoff. No reviewer should treat a passing integrity check as release approval or full #918 acceptance.

## Exact inputs and ancestry

#916 assessment PR #1151 is merged at `374ecc2a1237094e031ecb5f6d82853ea72ab819` and retains the `not_proven` qualification decision. #917 documentation handoff PR #1152 is merged at `c72c8cc1bf255ae2e26bb85b631ce0550fd886fc`; its acceptance is `assessment_handoff`, not product qualification. #919 internal review PR #1153 is merged at `52756b5bc7b02e3ebf6886170ee59b55f8dc10d1`; its frozen review records remain unchanged. No external review is claimed.

The historical #918 draft remains available at `5c4a6149771c637f3c805985b86231077965eab4`. This refresh starts from integrated drafting base `6af420a3cd0872cbafff612c6bbb918900ef97cf` and preserves original source-specific test identities. The predecessor manifest is checked at its reviewed source `e94ab771f9d3cfee5142aee584d57345a3ac6c29`, which must be the second parent of the recorded #1152 merge. At that merge, `adl/Cargo.toml` changed through integrated main: its historical manifest SHA-256 is `c546f8e9f4e5d8568c1c2c64c85d4169a89dd6061e35900458fcd8fde4c7ead4`, while merged bytes are `1db9bc6cb1e830648f2eb5ca491262784003dce082ae016308c807dea7e21d54`. The other 128 manifest documents match. Preserve the original manifest as source-time evidence; it is not a claim that every merged-tree byte was reviewed at that checkpoint.

PUBLICATION_MANIFEST.json binds exact byte identities, format, role, custody, version policy, proposed destination and pending approval for included documents. For new #918 files, source_revision denotes the drafting base, while SHA-256 denotes their actual candidate bytes; the PR head supplies final Git identity. No circular self-hash or invented final candidate identity is used.

## Review procedure

1. Read release notes, the inherited quality decision and original qualification deferral. Reconcile a delivered claim and an explicitly unproven claim with their sources. Preserve all 69 task and 24 prerequisite identities.
2. Run `python3 docs/milestones/v0.92.2/evidence/issue-918/validate_packet.py --self-test`. It checks bytes, candidates, paths and approval boundaries with negative cases. The dependency-free validation crate executes these same checks for native proof; it is not product testing.
3. Check DESTINATIONS.json and VERSION_INVENTORY.json. Private repository review is a proposed custody boundary, not verified remote access or permission to distribute.
4. Review missing outputs honestly: actual CodeFriend Markdown/HTML/PDF exports and final installed binary provenance are unresolved; no visual or content parity acceptance is recorded. Private manuscript bytes are not in this repository; sanitized metadata is a reference, not a fresh file inspection.
5. Report actionable findings before summary against the exact PR head. Check privacy/legal claims, licenses, unresolved destination permissions and every proposed public claim. Approval must identify actor, exact artifact hashes and action scope; none is granted here.

## Outstanding acceptance

Exact final installed candidate and artifact set; complete format inspection/parity; privacy/legal approval and destination access; required quality and external review; #1150 qualification after #1148/#1149 before Beta1 launch. These remain explicit gaps, not silent omissions or completed tests. #919 may inspect this draft but cannot infer release acceptance from it.

## Verification limits

This PR performs local deterministic document/manifest validation and independent review only. Inherited product-test results are attributed to their original checkpoints. No provider calls, source changes to product behavior, shared binary installation, deployment or public distribution. Original #917 manifest is checked against its Git checkpoint, rather than rewritten to bless downstream edits. Full external-link access and original private manuscript rendering have not been tested.

## Integrated repair checkpoint

All four repair groups are merged at the identities in [INTEGRATION_RECONCILIATION.json](INTEGRATION_RECONCILIATION.json). The accepted frozen #919 denominator remains **27 findings**. Follow-on #1172 is separately identified and is outside that denominator. This records repair integration; it does not rewrite the original changes-required review or establish external review, installed qualification, output parity, privacy/legal approval or release. The candidate revision is the containing Git commit; the manifest binds exact artifact bytes without a circular self-hash.
