## Metadata

- Skill: `repo-review-security`
- Target: `f0a011a5c59d46c763d669f69a10308b3f870ba4..c24f8fa65ce445b03ce6cd69007307291d78b60c`
- Reviewed checkout: detached read-only checkout at the exact candidate SHA
- Date: 2026-09-09T03:33:34Z
- Artifact: `docs/milestones/v0.92.1/evidence/release/tail-04/specialists/security-review.md`
- Depth: deep

## Findings

- **P2: Publication redaction passes without scanning the 791 published documents**
  File: `.csdlc/prepared/issues/519/validate-publication-candidate.rb:74`
  Role: security
  Scenario: A machine-local path, credential location, private payload, or token-shaped value exists in one of the 791 documents bound into the publication handoff, while the three small TAIL-03 packet files themselves are clean.
  Impact: `--redaction` and `--all` report success even though the publication candidate references content that violates its own redaction/path-portability acceptance criterion. A later external handoff can reveal operator identity, filesystem layout, credential locations, or actual credentials without failing the gate.
  Evidence: Lines 74-77 scan only files physically under `tail-03/`; lines 33-35 merely hash-check the 791 manifest documents and never call `redaction_safe?` on their bytes. The limitation is admitted in `docs/milestones/v0.92.1/evidence/release/tail-03/README.md:13` and `.csdlc/issues/519/cards/srp.md:42`, but STP AC-3 still says “The packet passes redaction and path-portability checks” at `.csdlc/issues/519/cards/stp.md:26`, and the SOR records the redaction purpose as proving machine-local paths absent at `.csdlc/issues/519/cards/sor.md:63-71`. A complete scan of the manifest’s 791 files found 35 machine-local-path occurrences. Concrete candidate examples include the operator worktree path and account email in `docs/milestones/v0.92.1/evidence/cloud/gcp-a/operator-reauth-proof-packet.md:33`, the exact local service-account key location in `docs/milestones/v0.92.1/evidence/cloud/gcp-b/bootstrap-identity-readiness.md:15`, and 11 retained machine-local FastWork paths beginning at `docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/ownership.json:82`. All three files are explicitly members of the handoff at `handoff-content.json:1323`, `:1335`, and `:1455`. Despite that, `ruby .../validate-publication-candidate.rb --all` returns `status: pass` and `final_acceptance: true`.
  Residual risk: The pattern scan found no private-key block or common AWS/GitHub/Google token signature in the 791 documents, but the validator’s incomplete denominator leaves unrecognized credential formats and private payloads unguarded.

- **P2: The active public Spot Runtime root permits an unrecoverable instance by default**
  File: `infra/aws/modules/csm-runtime-spot/variables.tf:52`
  Role: security
  Scenario: An operator applies the documented `infra/aws/csm-runtime-spot` fast path without overriding its SSH and key variables.
  Impact: Terraform creates a publicly addressed Runtime instance that has neither an EC2 key pair nor any SSH recovery ingress. If user data, Runtime, TLS, or observability fails, operators lose the required independent recovery path and may be forced to replace the node or expand access under pressure. This is an availability and incident-response control failure.
  Evidence: `ssh_ingress_cidrs` defaults to `[]` at lines 52-55 and `key_name` defaults to `null` at lines 58-61. The module still sets `associate_public_ip_address = true` and directly forwards `key_name` at `infra/aws/modules/csm-runtime-spot/main.tf:74-80`. The root repeats the unsafe defaults at `infra/aws/csm-runtime-spot/variables.tf:64-73`, and its checked-in example sets both `ssh_ingress_cidrs = []` and `key_name = null` at `infra/aws/csm-runtime-spot/terraform.tfvars.example:13-16`. The active README calls this the disposable Runtime host and describes the fast path but never requires a recovery key or SSH rule (`infra/aws/csm-runtime-spot/README.md:1-23`).
  Residual risk: SSM or replacement may sometimes recover the workload, but neither is required by this module. A separate hardened GPU-proof root requires `/32` SSH; that does not constrain this active root.

## Trust Boundaries Reviewed

- TAIL-02/Tail-03 publication boundary: exact reviewed source, 791-document manifest, 14 retained artifacts, candidate packet, and redaction validator.
- Durable lifecycle/evidence boundary: SRP/SOR statements, validation logs, cloud readbacks, and machine-local evidence references.
- AWS Runtime boundary: public ALB, public Spot Runtime module, private Runtime module, operator SSH access, Runtime ingress, IAM instance profile, IMDSv2, encrypted storage, and unrestricted bootstrap egress.
- GCP Runtime boundary: project/folder IAM, OS Login/IAP, two-node Runtime/Ollama network separation, metadata inputs, GCS artifact download, SHA-256 verification, service-account scopes, snapshot preparation, and cleanup evidence.
- Artifact supply boundary: S3/GCS model/runtime bundles, manifest/object digests, archive extraction, persistent warm storage, and retained receipts.
- Browser/public-edge boundary: exact origin lists, ACM/CloudFront/ALB TLS, WSS origin host/SNI, WAF rate limiting, and log retention.

## Assets And Attacker Capabilities Considered

- Assets: provider credentials, local service-account keys, GitHub/AWS/GCP tokens, operator identity and paths, model/runtime artifacts, Terraform state, KMS keys, checkpoint/log archives, Runtime/Guardian/Ollama control paths, and publication evidence.
- Attackers: unauthenticated Internet clients, compromised browser origin, compromised artifact bucket writer, over-privileged cloud principal, malicious or corrupted retained evidence, and an operator following checked-in defaults.
- Complete sensitive-value scan: every readable file in the 791-document publication manifest was scanned for machine-local paths, private-key headers, common GitHub/AWS/Google token formats, and bearer authorization. The scan produced 35 local-path hits and no recognized embedded private key/token values.
- Complete infrastructure scan: all 166 changed Terraform/template/shell infrastructure paths were enumerated and screened for public ingress, SSH/key configuration, IAM wildcards, metadata credentials, unauthenticated downloads, digest validation, and secret-bearing user data.

## Validation Performed

- `ruby .csdlc/prepared/issues/519/validate-publication-candidate.rb --all` — passed with `final_acceptance: true`; manual control-flow inspection and the complete 791-file scan prove the redaction result is non-proving for referenced content.
- Complete manifest scan — 791/791 declared documents opened when readable and checked against the declared sensitive patterns; 35 machine-local-path occurrences recorded by path/line/pattern without printing secret-shaped values.
- Complete changed-infrastructure census — 166/166 Terraform, `.tftest.hcl`, template, and shell files enumerated; trust-boundary patterns were inspected at their concrete definitions.
- No cloud mutation, network probing, credential read, exploit attempt, or reviewed-checkout edit was performed.

## Packet-Build Correction

The preliminary generic packet assigned security only `.csdlc/locks/*`. The #520 conductor is replacing it with the full issue-specific denominator; this report used the independently derived exact diff and publication manifest. The scaffold mismatch is not counted as a candidate security finding.

## Residual Risk

- Pattern screening is not a formal secret scanner and cannot recognize arbitrary private payloads; remediation must make the validator’s denominator complete and add negative fixtures for referenced-document leakage.
- Cloud IAM and network controls were reviewed statically. Live effective policy, organization-policy inheritance, DNS/TLS state, and residual resources require the dedicated cloud proof evidence.
- Dependency provenance and license conclusions belong to the dependency lane; archive extraction and artifact integrity were considered here only as trust-boundary inputs.
