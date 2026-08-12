# Structured Review Prompt

Template: 1.0.0

Issue: 291

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-edit initialized decomposition recovery route, request/result schema, semantic card replacement, #114 read-only golden fixture tests, failure diagnostics, docs, and lifecycle truth for #291 only.

## Prompts

- Does the packet preserve R2 FAIL evidence in a digest-bound immutable artifact derived from task/git/session evidence, with exact historical findings and original line references clearly marked as historical rather than mutable live-file proof?
- Does the design preserve the invalid bootstrap approval as historical evidence while requiring exact old/new design-review authority capture for any recovery-cleared false approval such as operator:planning-1-assignment?
- Does the design define an implementable write-ahead journal transaction: validate every replacement first, write content-addressed staged blobs, fsync blobs/manifest/directories where supported, record preimage/postimage hashes, write a prepared manifest as the durable point of no return, write a commit marker only after postimage verification, and avoid partial values/rendered/index/audit visibility?
- Does the design specify deterministic recovery and idempotency for crash points before prepared-manifest fsync, after prepared-manifest fsync, after target replacement, before parent-directory fsync boundary, and after commit-marker fsync: abandon/pre-state only before durable prepared manifest, roll-forward/post-state after durable prepared manifest, and fail-closed on unexpected target hashes?
- Does the design enforce canonical request-root/cwd/repository identity, path containment, issue/path identity, symlink and escape rejection, and isolated #114 golden-root mutation tests while proving live #114 and root .csdlc/locks/114.lock remain unchanged?
- Is graph input generic and typed, with node IDs, roles, directed edges, parent integration owner, acyclicity/order/in-scope validation, and explicit rejection of missing, inverted, duplicate, out-of-scope, or cross-child trust-redefinition cases rather than hard-coded #114 child topology?
- Is the semantic replacement surface complete but closed across SIP/STP/SPP/VPP/SRP/SOR, preserving nonterminal initialized truth and consistent issue identity without enabling bind, publication, merge, closeout, GitHub writes, #112 mutation, or product behavior?
- Does the validation plan cover positive recovery and the required negatives for stale CAS, unsupported phase, missing/drifted evidence, bootstrap overwrite, root/cwd mismatch, path/symlink escape, journal/manifest/commit-marker/crash recovery failures, graph mismatches, closed-field violations, #114/root-lock mutation, and already-repaired-field preservation?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
