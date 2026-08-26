# Structured Planning Prompt

Template: 1.0.0

Issue: 291

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Preserve the #291 root pre-bind recovery bundle, obtain fresh design approval first, then bind #291 to implement the narrow csdlc-edit initialized decomposition recovery contract, prove it against an isolated read-only #114 gen35 golden fixture and focused negatives, obtain #119-compliant implementation review, and publish a ready unmerged PR.

## Plan

Revision 12

## Steps

[
  {
    "id": "S1",
    "action": "Preserve the root pre-bind recovery bundle, retain digest-bound R2 failure evidence, and obtain fresh design approval before any bind or implementation.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement exact old/new design-review authority capture for false approval reversal while preserving append-only audit and historical design/diagram bytes.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement validation-before-write, content-addressed staged blobs, prepared manifest, fsync ordering, commit marker, deterministic startup recovery, cleanup/idempotency semantics, and crash-point injection tests.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement canonical root/cwd identity, containment, issue/path checks, symlink and escape rejection, and isolated #114 golden-root mutation tests.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Implement preparation-only SOR replacement that keeps execution, publication, merge, closeout, and terminal states nonterminal.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Implement closed identity replacement/disposition with consistent propagation across every card projection.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Implement historical-design/reference evidence handling and bootstrap-overwrite rejection.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S8",
    "action": "Implement generic typed graph validation for nodes, roles, directed edges, parent integration owner, acyclicity, in-scope checks, and forbidden trust redefinition.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S9",
    "action": "Prove recovered packets remain initialized/pre-review until valid #119 design/card approval and that ordinary initialized edits remain rejected outside the recovery route.",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S10",
    "action": "Prove typed JSON failure diagnostics for every CAS, evidence, scope, phase, graph, root, path, symlink, unsupported durability fallback, journal/manifest/commit-marker atomicity, and #114/root-lock mutation negative.",
    "acceptance_ids": [
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- No #114 mutation occurs while validating the golden fixture; mutation tests use an isolated issue-owned copy.
- Initialized recovery is CAS-guarded, validates all replacements before write, stages post-state bytes as content-addressed blobs, uses a prepared manifest plus commit marker, and records one generation and one audit event.
- A durable prepared manifest is the recovery point of no return: transactions without one are abandoned/pre-state, while any prepared transaction rolls forward to the exact post-state.
- Unexpected target hashes outside the manifest preimage/postimage set fail closed and are not overwritten by recovery.
- Recovery records exact old/new design-review authority truth when correcting false approval and never infers approval from assignment or planning.
- Retained R2 failure evidence is digest-bound to an immutable historical snapshot rather than mutable live file lines.
- Historical design/diagram bytes are preserved and any historical/reference-only status is explicit.
- Recovered SOR remains preparation-only and nonterminal.
- Shared title/slug/version identity updates are consistent across every card projection.
- Graph validation uses generic typed nodes, roles, directed edges, parent integration owner, acyclicity, in-scope checks, and forbidden trust-redefinition checks.
- Request root, cwd, repository identity, issue paths, and fixture paths are canonicalized, contained, and symlink/escape-safe before mutation.
- Normal initialized edit rejections remain intact outside the explicit recovery contract.
- Bootstrap regeneration cannot overwrite preserved initialized history.

## Risks

- A broad recovery route could become bootstrap overwrite by another name.
- False design approval could be hidden unless exact old/new review authority is recorded.
- Partial values/rendered/index/audit writes could make a corrupted packet look recovered.
- A cwd or root mismatch could mutate the live #114 fixture or wrong issue bundle.
- Hard-coded #114 graph handling could fail for the next decomposed parent.
- A loose replacement surface could allow implementation, publication, merge, closeout, or terminal claims into initialized recovery.
- Fixture tests could miss symlink, path escape, root-lock, or already-repaired-field preservation regressions.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/291/design.md

Digest: e54e499d61f9accfbb1c24160d0b5464737fa7e841db6a9c3c259351160899d4

## Diagram

.csdlc/prepared/issues/291/diagram.mmd

Digest: 1b1f1163776de17040b90623510fe9ab3336f7c0e3152e2a17fe4f350a364b54

## Stop Conditions

- A fresh design review has any unresolved actionable finding.
- Typed v2 cannot represent exact old/new false-approval reversal audit truth without hand-editing audit.
- R2 historical findings cannot be retained in a digest-bound immutable artifact derived from task/git/session evidence.
- The route cannot validate every replacement before writing staged blobs or cannot use a prepared manifest, preimage/postimage hashes, commit marker, and deterministic recovery protocol.
- The implementation cannot make durable prepared manifest the point of no return, abandon only transactions without a durable prepared manifest, roll forward every prepared transaction to exact post-state, and fail closed on unexpected target hashes.
- Crash injection before prepared-manifest fsync, after prepared-manifest fsync, after target replacement, before parent-directory fsync boundary, or after commit-marker fsync cannot prove the declared abandon-or-roll-forward rule.
- Unsupported platform fsync/durability behavior cannot be typed and fail-closed for publication-grade recovery.
- Crash, stale-CAS, repeated-request, journal cleanup, or idempotency behavior cannot be made deterministic.
- Canonical request root and cwd cannot be proven identical to the target repository identity.
- Path containment, symlink rejection, issue/path identity, or absolute/parent escape checks cannot be enforced before mutation.
- The #114 fixture cannot be copied to an isolated golden root for mutation tests while proving live #114 and root .csdlc/locks/114.lock remain unchanged.
- Graph input cannot be expressed generically as typed nodes, roles, directed edges, parent integration owner, acyclic order, and in-scope status.
- The replacement field set cannot remain closed and nonterminal for initialized recovery.
- Focused proof, full csdlc-v2 tests, strict Clippy, diff hygiene, or #119-compliant review fails.

## Handoff

Proceed only after doctor readiness.
