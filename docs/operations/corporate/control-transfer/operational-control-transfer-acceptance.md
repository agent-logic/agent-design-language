# Corporate Operational-Control Transfer Acceptance

- Issue: #497
- Sprint umbrella: #532
- Machine-readable packet: `docs/operations/corporate/control-transfer/operational-control-transfer-acceptance.v1.json`
- Evidence directory: `docs/milestones/v0.92.1/evidence/corporate/corp-c/`

This packet accepts CORP-C as a repository-local operational-control transfer surface. It deliberately does not claim that every live provider, billing, credential, DNS, certificate, legal, or private-custody action has been performed. Completed evidence, deferred actions, blocked actions, and operator-authorized actions are separated so Sprint 4 can proceed without turning a truthful corporate register into a fog machine.

## Prerequisite Gate

The prerequisite gate passes for #497:

| Lane | Issue | Closing PR | Merge commit | Ancestral to `origin/main` |
| --- | ---: | ---: | --- | --- |
| CORP-A critical-asset schedule | #482 | #545 | `e2c1d1649b0c930a5a1254575a07ef2a4496d48d` | yes |
| CORP-B corporate account custody register | #483 | #562 | `4a0b49c0071bacdaab19d6d9eb8c44380beb51be` | yes |
| GCP-D private platform foundation | #493 | #587 | `c0bf217934508d6dbc70d78633e6a95d5ddd9d06` | yes |
| AWS-G CloudFormation retirement decision | #496 | #599 | `83077ca029d52c9d613ed5a373da30f1dd42d9b3` | yes |

Evidence: `docs/milestones/v0.92.1/evidence/corporate/corp-c/prerequisite-ancestry.v1.json`.

## Accepted Repository-Local Evidence

- CORP-A provides the critical-asset schedule and redacted custody evidence:
  `docs/operations/corporate/asset-register/critical-asset-schedule.md`,
  `docs/operations/corporate/asset-register/critical-asset-schedule.v1.json`,
  and `docs/milestones/v0.92.1/evidence/corporate/corp-a/custody-receipts.v1.json`.
- CORP-B provides the corporate account custody register and readback receipts:
  `docs/operations/corporate/account-custody/corporate-custody-register.md`,
  `docs/operations/corporate/account-custody/corporate-custody-register.v1.json`,
  and `docs/milestones/v0.92.1/evidence/corporate/corp-b/readback-receipts.v1.json`.
- AWS business-profile identity was read back using `agent-logic-admin` through
  the retained AWS-G/Sprint 4 evidence boundary without provider mutation,
  credential capture, billing change, IAM change, DNS change, or workflow
  mutation by #497.
- GCP-D is accepted here only as a closed, merged, ancestral prerequisite; this
  issue does not repeat or extend GCP provider proof.

## External Action Classification

The canonical classification record is
`docs/milestones/v0.92.1/evidence/corporate/corp-c/external-action-classification.v1.json`.

| Class | Count | Meaning |
| --- | ---: | --- |
| Completed evidence | 4 | Repository-local or read-only provider evidence captured without mutation. |
| Authorized action | 0 | No external mutation was authorized or performed for #497. |
| Deferred action | 5 | Follow-up service, custody, or private-process work outside this issue's safe public packet. |
| Blocked action | 0 | No required repository-local acceptance action is blocked after the prerequisite gate. |

Deferred actions include hosted-zone and DNS transfer, certificate renewal ownership, GitHub organization and Actions billing readbacks, private vault and recovery-factor custody, payment-method custody, executed-instrument custody, deployment rollback readback, and private legal diligence. Those surfaces require explicit operator authorization and, where applicable, rollback or break-glass evidence before any mutation.

## Acceptance Result

CORP-C is accepted with deferred external actions.

This acceptance means:

- Sprint 4 has a truthful corporate operational-control packet grounded in
  merged CORP-A, CORP-B, AWS-G, and GCP-D prerequisites.
- No production/provider mutation, billing change, credential transfer, DNS
  change, certificate action, workflow mutation, or private custody transfer was
  performed by #497.
- The packet is safe to publish as repository-local operational-control
  acceptance after bounded review.

This acceptance does not mean:

- live provider cutover is complete;
- all cloud parity or billing custody has been proved;
- private legal, diligence, vault, recovery-factor, payment-method, or executed
  instrument material is present in the repository;
- Sprint 7 #345 AWS GPU execution, Sprint 8 #84 Unity work, or CORP-D #498 diligence acceptance has been performed.

## Validation

The issue-local validator
`.csdlc/evidence/497/validate-readiness.rb` checks:

- prerequisite issue/PR/merge ancestry evidence;
- account-authority readback boundary fields;
- external action classifications and mutation flags;
- packet-to-evidence references;
- absence of obvious credential/private-key material in the CORP-C packet.
