# #918 external-review publication packet

Status: reviewable draft; final acceptance pending. The user explicitly authorized a draft PR before #916/#917 integration. No reviewer should treat a passing integrity check as release approval or full #918 acceptance.

## Exact inputs and ancestry

#916 draft PR #1151: `e119223cd13ebbb6a07c0349ba6772f8ee2ecece`. #917 draft PR #1152: `c10757270098aea35e36453c91054ea8b4f947db`. Both are draft inputs with acceptance pending. This branch includes their ancestry against main so reviewers must distinguish inherited changes from #918 changes. The #918 delta starts after its merge of #917; compare against drafting base `3c6ebcb1e42ee993a1bc9152857c3008be655181` to inspect only finalization work. That merge also incorporates the newer main baseline; a direct comparison to #917 includes those unrelated already-landed differences. Integrate predecessors first and refresh source identities, affected hashes and independent review after any substantive upstream change.

PUBLICATION_MANIFEST.json binds exact byte identities, format, role, custody, version policy, proposed destination and pending approval for included documents. For new #918 files, source_revision denotes the drafting base, while SHA-256 denotes their actual candidate bytes; the PR head supplies final Git identity. No circular self-hash or invented final candidate identity is used.

## Review procedure

1. Read release notes, the inherited quality decision and original qualification deferral. Reconcile a delivered claim and an explicitly unproven claim with their sources. Preserve all 69 task and 24 prerequisite identities.
2. Run `python3 docs/milestones/v0.92.2/evidence/issue-918/validate_packet.py --self-test`. It checks bytes, candidates, paths and approval boundaries with negative cases. The dependency-free validation crate executes these same checks for native proof; it is not product testing.
3. Check DESTINATIONS.json and VERSION_INVENTORY.json. Private repository review is a proposed custody boundary, not verified remote access or permission to distribute.
4. Review missing outputs honestly: actual CodeFriend Markdown/HTML/PDF exports and final installed binary provenance are unresolved; no visual or content parity acceptance is recorded. Private manuscript bytes are not in this repository; sanitized metadata is a reference, not a fresh file inspection.
5. Report actionable findings before summary against the exact PR head. Check privacy/legal claims, licenses, unresolved destination permissions and every proposed public claim. Approval must identify actor, exact artifact hashes and action scope; none is granted here.

## Outstanding acceptance

Accepted #916 decision and #917 handoff; exact final installed candidate and artifact set; complete format inspection/parity; privacy/legal approval and destination access; required quality and external review; #1150 qualification after #1148/#1149 before Beta1 launch. These remain explicit gaps, not silent omissions or completed tests. #919 may inspect this draft but cannot infer release acceptance from it.

## Verification limits

This PR performs local deterministic document/manifest validation and independent review only. Inherited product-test results are attributed to their original checkpoints. No provider calls, source changes to product behavior, shared binary installation, deployment or public distribution. Original #917 manifest is checked against its Git checkpoint, rather than rewritten to bless downstream edits. Full external-link access and original private manuscript rendering have not been tested.
