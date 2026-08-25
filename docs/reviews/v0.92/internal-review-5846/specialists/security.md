# Security And Privacy Specialist Review

## Findings

- P1: The v0.92 security claim-boundary gate is still unproved.
  File: `docs/milestones/v0.92/MILESTONE_CHECKLIST_v0.92.md:83`
  Role: security
  Scenario: A release or external-facing handoff consumes the feature coverage statement that ACIP/A2A transport is `implemented_with_evidence` while the canonical checklist still leaves the required ACIP transport cases and the security claim-boundary denial proof unchecked.
  Impact: The release packet can imply authenticated production WebSocket transport or red/blue CAV readiness without the canonical gate proving those claims or explicitly scoping them out. This is a release-blocking security-evidence ambiguity, not evidence that the underlying runtime control is exploitable.
  Evidence: Lines 83-89 leave both the ACIP transport proof and the security claim-boundary proof unchecked; `docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md:39` labels ACIP/A2A transport `implemented_with_evidence`; and the third-party handoff requires blockers and non-claims to remain explicit at `docs/milestones/v0.92/review/THIRD_PARTY_REVIEW_HANDOFF_v0.92.md:23-25`.
  Required fix: Reconcile the canonical checklist against exact retained ACIP security evidence. Check the gates only if the required positive and denial cases are proven at the reviewed revision; otherwise name the missing proof and retain an explicit non-claim before release or external publication.

- P1: The generated security assignment does not cover the repository's security surface.
  File: `docs/reviews/v0.92/internal-review-5846/specialist_assignments.json:142`
  Role: security
  Scenario: A synthesis or meta-review treats the generated `security` assignment as proof that the repository-wide security lane covered the target.
  Impact: The assignment can yield a false-negative security review because it contains only 30 zero-byte lifecycle lock files and omits the runtime API authentication, TLS, redaction, AWS workflow/remote-runner, ACC authority, private-state, and protocol admission surfaces that carry the actual security behavior.
  Evidence: The security array at lines 142-172 contains only `.csdlc/locks/*.lock`; the packet simultaneously claims 23,622 tracked files and 17 CI files in `repo_scope.md:59-65`, and says the packet contains metadata rather than source excerpts at `repo_scope.md:41-45`. This specialist manually inspected representative omitted surfaces, but that bounded inspection does not repair the packet denominator.
  Required fix: Regenerate or amend the assignment with an explicit, deterministic security inventory covering the actual authn/authz, TLS, secret custody, redaction/logging, external-input, cloud/CI, filesystem, and network boundaries. Synthesis must not mark the security lane complete until that inventory is reviewed or each omitted class is explicitly recorded as a limitation.

## Metadata

- Skill: `repo-review-security` with `redaction-and-evidence-auditor`
- Reviewer: Codex security specialist (`review_313_security`)
- Repository: `agent-logic/agent-design-language`
- Exact target: `c6792e54df1db5969fa28c59b6dfe4c714ed5559`
- Review packet: `docs/reviews/v0.92/internal-review-5846`
- Review depth: targeted security/privacy review over packet evidence plus representative high-risk source surfaces
- Date: 2026-08-25 UTC
- Finding count: 2 (`P0`: 0, `P1`: 2, `P2`: 0, `P3`: 0)
- Verdict: `CHANGES_REQUESTED`

## Trust Boundaries Reviewed

- Runtime API bearer-token storage, rotation, revocation, gateway identity, origin admission, control-message signatures, replay, capability, and authority checks.
- TLS identity loading and TLS 1.3 server/client configuration, including mutual-TLS configuration entrypoints.
- Runtime observability endpoint policy and structured-field redaction.
- AWS OIDC/workflow secret references, remote-validation SSH custody, IMDSv2 use, and operator-gated retry posture.
- ACC/UTS authority separation and rejection of model-self-reported authority.
- Review-packet portability, private-path, secret, prompt/tool-argument, internal-URL, and publication boundaries.
- v0.92 ACIP transport-security and public-claim boundaries.

## Assets And Attacker Capabilities Considered

- Assets: runtime API credentials, TLS private keys, provider/AWS credentials, agent capability grants, private runtime state, telemetry payloads, external-review evidence, and release/publication authority.
- Capabilities: an unauthenticated network client; a client with a stolen or stale credential; a malicious model/tool payload; an untrusted repository contributor influencing CI or shell inputs; a local user able to read permissive files; and an external reviewer receiving over-broad or machine-private evidence.

## Reviewed Surfaces

- `docs/reviews/v0.92/internal-review-5846/{run_manifest.json,repo_scope.md,repo_inventory.json,evidence_index.json,specialist_assignments.json}`
- `adl-runtime/src/runtime_api_auth.rs`
- `adl-runtime-kernel/src/tls.rs`
- `adl-runtime-kernel/src/observability.rs`
- `adl-runtime-kernel/src/observability/redaction.rs`
- `adl/src/acc/validation.rs`
- `adl/src/uts.rs`
- `tools/aws_remote_validation/src/aws_remote_validation.rs`
- `tools/aws_remote_validation/scripts/remote_validation_runner.sh`
- `.github/workflows/aws-codefriend-build.yaml`
- `.github/workflows/aws-spot-remote-validation.yaml`
- v0.92 feature coverage, quality gate, milestone checklist, and third-party handoff security claim surfaces.

## Validation Performed

- Deterministic packet redaction audit for audience `local_only`: 7 files scanned; status `pass`; 0 blockers, 0 warnings, 0 info findings. This proves only the current packet files are clean under the helper's patterns.
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml runtime_api_auth --lib`: 13 passed, 0 failed, 326 filtered out. This exercised credential permissions, rotation/revocation, gateway identity, origin/admission, signature, authority, and replay cases.
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml observability::redaction --lib`: 0 tests selected and therefore non-proving. Source inspection confirmed the narrow HTTPS-or-loopback OTLP rule and field/text redaction logic, but no test result is claimed.
- Exact-target secret-pattern scan across tracked text found no non-fixture credential value. Matches were documentation/test markers and branch-name substrings; no secret value is reproduced here.
- Source inspection was performed against Git object `c6792e54df1db5969fa28c59b6dfe4c714ed5559`; later issue-313 artifact commits were not treated as product authority.

## Redaction And Publication Boundary

- Packet manifest policy is `privacy_mode: local_only` and `publication_allowed: false`; this review preserves both restrictions.
- Publication recommendation: `allow_internal` only. Customer-facing or public sharing remains blocked until an explicit pre-publication audit reviews the final assembled report and an operator changes publication authority.
- No full credential, private key, private prompt, provider payload, machine-local path, or raw tool argument is included in this report.
- The deterministic audit scanned packet artifacts, not all 23,622 repository files or every evidence object named by the index; it cannot certify the repository or a future synthesized report as redaction-safe.

## Verified Non-Findings

- The inspected runtime API credential store uses cryptographic random material, constant-time token comparison, mode-0600 credential files on Unix, atomic replacement, explicit revocation, bounded rotation overlap, and fail-closed malformed/missing authorization handling.
- The inspected WSS admission policy requires an exact HTTPS origin, authenticated credential, signed control payload, allowed capability and authority, bounded frame/sequence values, and replay denial before dispatch.
- The inspected TLS configuration pins TLS 1.3 and provides a separate mutual-TLS construction path; no insecure certificate-verification bypass was found in the reviewed slice.
- The reviewed GitHub AWS workflows use OIDC with pinned actions and secret references rather than embedded live AWS credentials.
- The packet itself contains no detected private host path, internal URL, secret-like value, or execution-input disclosure under the deterministic audit.

## Limitations And Residual Risk

- This was a targeted repository security review, not a full threat model, penetration test, dependency advisory audit, live cloud audit, or exhaustive review of 23,622 files.
- Runtime behavior beyond the focused authentication tests was not executed. TLS, observability export, AWS deployment, remote SSH, provider calls, and cross-polis transport were inspected statically only.
- Generated/vendor/cache surfaces were excluded. Dependency and CI supply-chain depth remains owned by the dependency specialist.
- Because the generated security assignment is materially incomplete, absence of additional findings must not be interpreted as repository-wide security approval.
- No product code, secrets, GitHub state, cloud resources, or release state were changed.
