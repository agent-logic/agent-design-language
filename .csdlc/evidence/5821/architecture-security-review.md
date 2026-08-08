# WP-04 Independent Architecture And Security Review

## Review Identity

- Issue: #5821, v0.92 WP-04 Distributed Guardian architecture and security gate
- Reviewed revision: `29243a6a5400467ceb0fe98b69baf855f8249b54`
- Reviewer: `openai-codex:gpt-5:wp04-authority-certificate-independent-review:2026-08-08`
- Agent ID: `codex-independent-review:29243a6a5400467ceb0fe98b69baf855f8249b54`
- Review role: independent architecture and security reviewer
- Review date: 2026-08-08

## Reviewed Scope

- The WP-04 architecture, threat model, parent design, feature contract, ADR, and both focused validators.
- The approved designs, typed indexes, and SIP/STP/SPP/VPP values for safety-relevant children #5863, #5864, #5865, #5867, #5869, #5870, #5875, and #5876.
- The live WP-04-IMP umbrella #5862 and all child issues #5863 through #5878.

The review was pinned to the committed content at the revision above. The exact-review manifest contains 55 authoritative paths: seven parent surfaces and six surfaces for each of eight safety-relevant children. Every reviewed SPP is therefore included in the exact artifact digest.

## Finding Dispositions

1. **`authority-linearization`: resolved.** Authority is serialized by a minimum three-voter OpenRaft ledger with committed-index linearization and fail-closed quorum loss.
2. **`activation-incarnation`: resolved.** Authority binds to a fresh activation incarnation, proof of possession, committed epoch, and lease safety window.
3. **`certificate-session-lifecycle`: resolved.** Session lifetime, revocation, generation changes, and per-operation revalidation are explicit.
4. **`maintained-transport`: resolved.** The contract selects pinned `quinn`, `rustls`, `prost`, and `openraft` surfaces and prohibits custom cryptography or framing.
5. **`candidate-status`: resolved.** Architecture and threat documents retain truthful candidate-gate and non-claim language.
6. **`authority-certificate-definition`: resolved.** The certificate body, endorsement, payload, exact protobuf fields, two-stage domain separation, Ed25519 strict verification, encodings, ordering, and rejection rules are frozen.
7. **`child-authority-contracts`: resolved.** Child designs and typed cards carry the repaired authority, possession, fencing, recovery, and proof obligations.
8. **`review-identity-and-revision`: resolved.** Review evidence is independently attributed and pinned to the latest authoritative revision and exact Git blobs.
9. **`staged-authoritative-drift`: resolved.** The validator rejects both staged and unstaged authoritative drift.
10. **`normalized-possession-contract`: resolved.** Enrollment requires the exact normalized phrase `proves possession`.
11. **`openraft-cots-contract-parity`: resolved.** #5865 and the live validator require the same four-dependency COTS set while preserving source ownership boundaries.
12. **`spp-exact-review-scope`: resolved.** All eight safety-relevant SPP values files are included in the authoritative manifest and artifact digests.
13. **`joint-membership-certificate-quorum`: resolved.** Stable membership requires a strict majority of committed voters; joint membership requires strict majorities of both old and new configurations. A union majority missing either constituent majority is rejected.
14. **`certificate-signature-suite`: resolved.** The algorithm identifier, Ed25519 key/signature widths, strict verifier, canonical protobuf restrictions, domain bytes, and exact signed payload are specified.
15. **`threat-joint-quorum-parity`: resolved.** T3 and T10 use the same majority-of-both rule and union-majority negative case as the architecture.
16. **`certificate-wire-canonicalization`: resolved.** Closed operation values, minimal varints, field order, unknown-field rejection, duplicate rejection, and decode/re-encode equality are required.
17. **`endorsement-signer-binding`: resolved.** Endorsements cover the body digest, signer identity, certificate generation, and algorithm; no endorsement metadata remains unsigned.
18. **`duplicate-effective-control-key`: resolved.** Enrollment, promotion, rotation, and quorum verification reject duplicate effective control keys and deduplicate by both voter identity and key.
19. **`per-surface-contract-validation`: resolved.** The child-wave validator checks each design and typed card surface independently, including SPP, rather than allowing aggregate keyword masking.
20. **`domain-separation-parity`: resolved.** Child contracts use the two frozen certificate-body and endorsement domains, and the validator rejects the superseded single-domain spelling except in explicit negative tests.
21. **`owner-contract-propagation`: resolved.** Enrollment, rotation, promotion/snapshot/replay, and lease-verification owners carry the key-uniqueness and joint-quorum obligations in their own typed records.

## Validation Performed

- Confirmed the latest authoritative revision is `29243a6a5400467ceb0fe98b69baf855f8249b54`.
- Confirmed all 55 authoritative files are present and digest-bound at that revision.
- Ran `validate-child-wave.rb`: PASS for live #5862 plus sixteen children and 38 exclusive implementation paths.
- Ran the focused architecture validator through all contract checks; before this evidence refresh its only expected failure was stale retained-review revision.
- Ran typed doctors for #5821, #5863, #5864, #5867, and #5869: PASS.
- Checked both Ruby validators for syntax and ran `git diff --check`: PASS.
- Independent Codex exact-head review: accepted with no actionable findings.
- Gemini 3.1 Pro exact-head review: accepted with no actionable findings.
- Claude review was attempted through the documented ADL provider route. The endpoint was reachable, but the bounded attempts consumed their response budget as reasoning and returned no usable review text. Claude is recorded as unavailable and is not counted as an approval.

## Residual Risks

- Compromise of a majority of current authority-ledger voters defeats automated ownership safety and requires operator trust-domain reconstruction.
- Loss of an authority-ledger majority intentionally halts mutation and relocation until quorum recovery.
- Host compromise can expose active purpose-bound keys until revocation and fencing propagate.
- Severe clock uncertainty can reduce availability; nodes outside the configured bound become non-authoritative.
- Simultaneous durable-store loss can leave a lineage unavailable and require recovery from retained audit and continuity evidence.
- Dependency vulnerabilities or abandonment remain supply-chain risks governed by pinned-version review and replacement policy.
- This architecture review does not prove implementation, deployment security, multi-node behavior, or native portability; those remain child and WP-04.16 obligations.

## Outcome

No unresolved actionable architecture or security findings remain for the reviewed revision and declared gate scope.

Verdict: accepted
