# Structured Review Prompt

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/208
.csdlc/prepared/issues/208
.csdlc/evidence/208
adl/.config/nextest.toml
adl-runtime-kernel/Cargo.toml
adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/continuity_control.rs
adl-runtime-kernel/src/governance.rs
adl-runtime-kernel/src/ingress.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/src/operations.rs
adl-runtime-kernel/src/reasoning.rs
adl-runtime-kernel/tests/configuration.rs
adl-runtime-kernel/tests/kernel_continuity_control.rs
adl-runtime-kernel/tests/production_acip_wss.rs
adl-runtime-kernel/tests/support/runtime_init.rs
adl-runtime/src/bin/adl-runtime-guardian.rs
adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
adl-runtime/src/distributed/polis_runtime.rs
adl-runtime/src/guardian.rs
adl-runtime/src/kernel_continuity_client.rs
adl-runtime/src/lib.rs
adl-runtime/tests/guardian_cli.rs
adl-runtime/tests/kernel_continuity_client.rs
infra/runtime-v3/runtime-init.toml

## Prompts

- Do both production binaries construct the private session and live participant registry before readiness, with no library-only or synthetic closure?
- Does TLS 1.3 mTLS plus exporter binding reject wrong peers, stale succession, captured sessions, bearer-only input, and every public/distributed identity before dispatch?
- Does the exact domain row prove canonical RFC 8785 acceptance and rejection of duplicate keys, noncanonical encoding, unknown fields, NaN/infinity, trailing bytes, decode/re-encode mismatch, and unknown operation kind, while the TLS row separately proves exporter mismatch?
- Do SourceContinuityEffectPort, ContinuityBundleSourcePort, and TargetContinuityEffectPort expose only opaque signed exact-effect handles and receipts, with #208 retaining every kernel/filesystem effect?
- Does TargetContinuityEffectPort revalidate signed key generation, catalog entry order/schema/range/digest and chunk index/range/digest/predecessor before any stage write, without caller-substitutable descriptors?
- Does TargetCleanupPermit remain discard-only and valid after transfer expiry/cancellation until TargetDiscardReceipt or TargetActivationReceipt, with exactly one cleanup owner in #208?
- Can #210 return only VerifiedTransferPossession and never delete/activate, while #204 alone owns the migration/control decision adapter and #208 performs the effect?
- Do crash/restart/reply-loss/succession/quiesce/stage/validate/activate/discard paths reconcile exact receipts without false no-effect, duplicate effect, or residue?
- Does diff hygiene compare the recorded execution base through exact proving source, reject nonancestry and dirty protected paths, and avoid working-tree-only evidence?
- Does exact proof bind fifty-six cases, the eight-row sixty-four-subassertion map and SHA-256, serial tests/Clippy/diff/producer/review/validator order, protected-source drift, immutable evidence, and squash topology?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted replacement CI and live merge state remain pending until the exact reviewed head is republished.

## Review Result

Revision: Some("git-blake3:428ba2c23328f4b36266e5a057445d391f656502:841198f1b02d1c8cbefbafb5f4ad881cacbd07c97e1b2cba20d902f887dd1fea")

Reviewer: Some("codex:/root/review_208_guardian_cli_final")

Result: pass
