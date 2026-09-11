## Outcome

Make AWS and GCP cloud-mutation authorization independently authentic and bind approval to the exact plan bytes that can be applied.

## Parent and findings

- Parent remediation issue: #522
- Source review: #520 at `fb6cbc7f619daa54f901fd2d12f480add682ace3`
- Findings: `D520-SEC-001`, `D520-SEC-002`

## Acceptance criteria

- A repo-writing process that holds provider credentials cannot manufacture valid operator approval by editing a local JSON packet.
- GCP mutation scripts consume an authenticated, immutable operator authorization receipt bound to repository, issue, account/project, resource bounds, expiry, and exact mutation plan.
- AWS validation hashes the saved `.tfplan` bytes directly and proves that any inspected JSON projection derives from those same bytes.
- Negative fixtures reject forged approval, altered plans, altered digest sidecars, expired authority, and cross-account/project replay.
- Existing read-only proof remains usable without mutation authority.

## Owned paths

- `.csdlc/prepared/issues/727/**`
- `.csdlc/prepared/issues/731/**`
- tightly coupled cloud-authorization documentation and tests

## Validation

- All authorization negative fixtures.
- Terraform plan-byte identity regression.
- No paid cloud mutation is required for this repair.

## Non-goals

- Running a new paid AWS or GCP proof.
- Broad credential-management redesign.
