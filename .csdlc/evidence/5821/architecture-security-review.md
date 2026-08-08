# WP-04 Independent Architecture And Security Review

## Review Identity

- Issue: #5821, v0.92 WP-04 Distributed Guardian architecture and security gate
- Reviewed revision: `e4f1572cf35b62f445d6aef54142f68da7d28cf6`
- Reviewer: `openai-codex:gpt-5:wp04-openraft-contract-independent-review:2026-08-08`
- Agent ID: `019fdf4a-5f9f-7420-9cfd-418ed63dee8c`
- Review role: independent architecture and security reviewer
- Review date: 2026-08-08

## Reviewed Scope

- `docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md`
- `docs/security/runtime-v3/DISTRIBUTED_GUARDIAN_THREAT_MODEL.md`
- `.csdlc/prepared/issues/5821/design.md`
- `docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md`
- `docs/adr/0054-runtime-v3-guardian-owned-kernel-and-api-boundary.md`
- `.csdlc/prepared/issues/5821/validate-architecture-security-review.rb`
- `.csdlc/prepared/issues/5821/validate-child-wave.rb`
- Canonical approved designs, typed indexes, and SIP/STP/VPP values for issues #5865, #5869, #5870, #5875, and #5876
- Live WP-04-IMP issue #5862 and child issue #5863 through #5878 contract validation

The review was pinned to the committed content at the revision above. The authoritative surface had neither unstaged nor staged changes during review.

## Prior Finding Dispositions

1. **`authority-linearization`: One-owner authority lacked an authoritative serialization point - resolved.** The architecture now assigns authority to a minimum three-voter, majority-replicated OpenRaft ledger with joint membership, committed-index linearization, fail-closed quorum loss, and explicit mutation-sink enforcement.
2. **`activation-incarnation`: Cloned state was indistinguishable from the legitimate holder - resolved.** Authority is bound to a fresh non-persistent activation incarnation, proof of possession, a committed epoch, and a lease safety window.
3. **`certificate-session-lifecycle`: Established sessions could outlive certificate expiry or revocation - resolved.** Session lifetime is bounded, revocation and generation changes close sessions, and every authority-bearing operation revalidates purpose, generation, revocation, and expiry.
4. **`maintained-transport`: The maintained transport and framing choice was unspecified - resolved.** The architecture selects `quinn`, supported `rustls` integration, and length-delimited `prost`, with reviewed lockfile pinning and no custom cryptography or wire framing.
5. **`candidate-status`: Architecture and threat documents prematurely claimed frozen status - resolved.** Both identify themselves as candidate gates pending independent review and retain explicit non-claims.
6. **`authority-certificate-definition`: Authority certificates were not cryptographically defined - resolved.** `AuthorityCertificateV1` defines canonical signed fields, a domain-separated SHA-256 digest, strict-majority distinct control-key endorsements, voter-generation binding, certificate lifecycle checks, activation possession, and malicious-leader/minority denial.
7. **`child-authority-contracts`: Child ownership, proof, and rollback contracts did not carry the repaired authority guarantees - resolved.** The parent ledger, reapproved child designs, typed SIP/STP/VPP records, and live contracts now assign and prove majority authority, joint membership, canonical certificates, activation possession, monotonic-time safety, mutation-sink enforcement, fence-boundary migration, and quorum-only recovery.
8. **`review-identity-and-revision`: Retained review evidence was not exact-revision or independently identity-bound - resolved.** Validation pins the latest authoritative revision, the actual nonempty reviewer and agent provenance, independent role, author separation, report digest, authoritative Git blobs, and finding dispositions while rejecting superseded revisions and authoritative drift.
9. **`staged-authoritative-drift`: Staged authoritative changes were not rejected - resolved.** Validation checks both worktree-versus-index and index-versus-HEAD drift across the complete authoritative review surface.
10. **`normalized-possession-contract`: Enrollment possession validation used an over-broad normalized term - resolved.** The section-term gate now normalizes whitespace while requiring the exact phrase `proves possession`, so unrelated proof language cannot satisfy the enrollment possession requirement.
11. **`openraft-cots-contract-parity`: WP-04.03 could omit OpenRaft despite sole manifest ownership - resolved.** Issue #5865's design and typed SIP/STP/SPP/VPP now require the reviewed `quinn`, `rustls`, `prost`, and `openraft` set, preserve WP-04.07 source ownership, and are digest-bound by the architecture review validator and checked by the live child-wave validator.

## Validation Performed

- Confirmed the exact authoritative review revision is `e4f1572cf35b62f445d6aef54142f68da7d28cf6`.
- Confirmed the latest authoritative revision resolves to the same full commit.
- Parsed the five reviewed child indexes and SIP/STP/VPP values as JSON.
- Confirmed #5865's distributed COTS design is reapproved, the four authority designs remain reapproved, and all five child records have null claim, branch, and worktree state.
- Ran `validate-child-wave.rb`: it passed the live umbrella and sixteen-child mapping, 38 exclusive paths, dependency graph, authority contracts, proof targets, and rollback checks.
- Checked both review validators for Ruby syntax.
- Confirmed both unstaged and staged authoritative surfaces were clean.
- Re-reviewed partition, replay, stale lease, cloned state, wrong trust domain, certificate compromise and expiry, relocation and rollback failure, split brain, child ownership, the four-dependency COTS contract, proof boundaries, and unsupported completion claims.

## Residual Risks

- Compromise of a majority of current authority-ledger voters defeats automated ownership safety and requires operator trust-domain reconstruction.
- Loss of an authority-ledger majority intentionally halts mutation and relocation until quorum recovery.
- Host compromise can expose active purpose-bound keys until revocation and fencing propagate.
- Severe clock uncertainty can reduce availability; nodes outside the configured bound become non-authoritative.
- Simultaneous durable-store loss can leave a lineage unavailable and require recovery from retained audit and continuity evidence.
- Vulnerabilities or abandonment in OpenRaft, quinn, rustls, prost, or other maintained dependencies remain supply-chain risks governed by pinned-version review and replacement policy.
- This architecture review does not prove implementation, deployment security, multi-node behavior, or native portability; those remain child and WP-04.16 obligations.

## Outcome

No unresolved actionable architecture or security findings remain for the reviewed revision and declared gate scope.

Verdict: accepted
