# Issue 624 Design — Corporate operational-control hardening sidecar

## Objective

Complete the public, redacted sidecar record for post-move-in corporate operational-control hardening, separated from #497 corporate IP-transfer acceptance.

## Source authority

- GitHub issue #624 is the canonical issue contract.
- #497 / PR #613 accepted the corporate IP-transfer boundary and explicitly routed broader hardening to #624.
- #497 sidecar correction PR #634 records that #624 is nonblocking for Sprint 4 closeout and owns GitHub/CI, DNS/certificate, AWS guardrail, private custody, and deployment rollback readback gaps.
- Existing corporate readback files under `docs/operations/corporate/**` and `docs/milestones/v0.92.1/evidence/corporate/**` are seed evidence only.

## Bounded implementation

Add one redacted operational hardening register and machine-readable receipt that:

1. Defines the full #624 denominator separately from #497.
2. Classifies each hardening row as proven from existing retained evidence or decomposed into a narrow follow-on action.
3. Names owner role, action, evidence reference, mutation authority, and closeout condition for every row.
4. Preserves non-claims for credentials, account IDs, private custody artifacts, billing controls, DNS/certificate changes, GitHub administration, and deployment mutation.
5. Provides a focused validator proving denominator completeness, evidence references, secret/path hygiene, row disposition, and separation from #497 acceptance.

## Owned paths

- `docs/operations/corporate/control-transfer/operational-control-hardening-sidecar.md`
- `docs/milestones/v0.92.1/evidence/corporate/corp-sidecar-624/**`
- `.csdlc/prepared/issues/624/**`
- `.csdlc/issues/624/**`

## Non-goals

- No GitHub org, repository, workflow, reviewer, billing, or secret mutation.
- No AWS, DNS, certificate, account-recovery, billing, or custody mutation.
- No private custody artifact, credential, account ID, billing identifier, or recovery detail enters the repository.
- Do not reopen #497 or treat #624 as a blocker for already-accepted #497 transfer.

## Validation

Run the focused sidecar validator, JSON parse checks, and exact diff hygiene:

- `python3 .csdlc/prepared/issues/624/validate-corp-sidecar-hardening.py`
- JSON parse for the retained machine-readable evidence file.
- `git diff --check origin/main...HEAD`

## Review focus

Review should check that the register does not overclaim proof, does not leak sensitive identifiers, does not mutate live control planes, and gives every unproven hardening row a concrete follow-on owner/action/closeout condition.
