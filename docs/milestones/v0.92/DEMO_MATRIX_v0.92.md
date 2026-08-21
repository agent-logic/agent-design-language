# v0.92 Demo Matrix

## Metadata

- Milestone: `v0.92`
- WP owner: `WP-20`
- Current issue: `agent-logic/agent-design-language#308`
- Legacy predecessor: `danielbaustin/agent-design-language#5840`
- Reconciled after legacy gate closure for `#5836`, `#5837`, `#5838`, and
  `#5839`.

## Status

This matrix is release-gate truth, not demo execution proof. It records which
demo claims have accepted exact-revision evidence and which remain blocked,
planned, or non-claimed. Planned rows must not be read as passed demos.

## Scope

The matrix covers birthday proof, negative cases, continuity, memory grounding,
capability, cognitive-profile evidence, adaptive-learning boundaries, ACIP
schema/public transport readiness, governance handoff, and the WP-20 proof
coverage validator.

## Demo Coverage Summary

| Demo ID | Demo / proof surface | Milestone claim | Primary proof surface | Status | Artifact index row |
| --- | --- | --- | --- | --- | --- |
| D1 | First birthday proof | A named identity can cross the birth boundary with required evidence. | Birthday record, witness set, receipt, and review packet. | blocked_with_evidence | AEE-014 |
| D2 | Not-a-birthday negative suite | Startup, wake, snapshot, admission, copied state, and fixtures are not birth. | Negative fixtures and validation report. | blocked_with_evidence | AEE-014 |
| D3 | Continuity across bounded cycles | Identity persists across multiple bounded cycles with evidence. | Cycle artifacts, continuity record, witness links. | blocked_with_evidence | AEE-014 |
| D4 | Memory grounding proof | Birth references witnessed memory artifacts without exposing raw private memory. | Memory-grounding packet, redacted projection, and denial proof. | blocked_with_evidence | AEE-009 |
| D5 | Capability envelope proof | The birth record declares provider, model, tool, skill, authority, and limit context. | Capability envelope inputs, validation, and missing-envelope denial. | blocked_with_evidence | AEE-009 |
| D6 | ACP / cognitive profile proof | Birth packet includes a bounded profile record grounded in evidence. | Profile fixture, update rationale, redacted reviewer packet, validation. | blocked_with_evidence | AEE-010 |
| D7A | Adaptive Learning DAG boundary proof | Adaptive learning is distinguished from bounded loop execution. | Evaluation bindings, accepted/rejected graph deltas, and replay negatives. | blocked_with_evidence | AEE-010 |
| D7 | ACIP binary schema and WebSocket carrier proof | Binary ACIP remains inspectable through public schemas while message contents remain governed. | Schema catalog, JSON projection, denied-access case, authenticated WSS trace. | blocked_with_evidence | AEE-011 |
| D8 | Birthday-to-governance handoff | v0.93 governance can consume v0.92 identity evidence without redefining birth. | Handoff packet mapping identity evidence to future governance. | blocked_with_evidence | AEE-017 |
| D9 | WP-20 coverage validator | Demo/proof rows cannot be passed without exact evidence and negative proof. | `adl/tools/validate_v092_demo_proof_coverage.py` and shell test harness. | blocked_with_evidence | AEE-018 |

## Coverage Rules

- Every accepted demo must distinguish birth from ordinary runtime activity.
- Every accepted birthday claim must cite exact-revision evidence.
- Every private-state boundary must have a redaction or denial proof.
- Every capability claim must include limits and authority context.
- Every cognitive-profile claim must cite evidence and remain distinct from
  identity, reputation, standing, or consciousness claims.
- Every adaptive-learning claim must distinguish bounded loop execution,
  evaluated adaptation, policy-governed graph modification, and future Adaptive
  Learning DAG proof.
- Every binary ACIP claim must prove public-schema decodeability,
  deterministic JSON projection, and separate message-content authorization.
- Planned or blocked rows are non-claims and must not support release approval.

## Demo Details

### D1) First Birthday Proof

Expected proof:

- stable name
- identity root
- continuity evidence
- memory-grounding references
- capability envelope
- witness set
- citizen-facing receipt
- reviewer finding

Current state: blocked with evidence pending accepted WP-18 proof.

### D2) Not-A-Birthday Negative Suite

Expected rejected cases:

- process startup
- snapshot
- wake
- admission
- copied state
- named test fixture without continuity evidence

Current state: blocked with evidence pending accepted WP-18 negative proof.

### D3) Continuity Across Bounded Cycles

Expected proof:

- prior and successor cycle artifacts
- continuity record
- witness links
- ambiguity handling or clear continuity grade

Current state: blocked with evidence pending accepted WP-18 continuity proof.

### D4) Memory Grounding Proof

Expected proof:

- witnessed memory-artifact references
- redacted projection
- reviewer packet that can inspect grounding without raw private-state exposure

Current state: blocked with evidence pending accepted memory-grounding proof.

### D5) Capability Envelope Proof

Expected proof:

- capability envelope inputs
- bounded authority and limit context
- missing-envelope or missing-limit denial proof

Current state: blocked with evidence pending accepted capability proof.

### D6) ACP / Cognitive Profile Proof

Expected proof:

- profile fixture
- source evidence references
- update rationale
- privacy/redaction policy
- validation report

Current state: blocked with evidence pending accepted profile proof.

### D7A) Adaptive Learning DAG Boundary Proof

Expected proof:

- loop-runtime status checklist
- evaluation-binding fixture
- bounded state-delta fixture
- policy decision for a proposed graph modification
- accepted and rejected graph-delta examples
- negative replay cases

Current state: blocked with evidence pending accepted adaptive-learning proof.

### D7) ACIP Binary Schema And WebSocket Carrier Proof

Expected proof:

- ACIP protobuf schema
- public schema catalog fixture
- deterministic JSON projection
- governed message-content access decision
- denied unauthorized inspection case
- authenticated full-duplex WebSocket session trace from the real Runtime carrier

Current state: blocked with evidence pending accepted ACIP/A2A proof.

### D8) Birthday-To-Governance Handoff

Expected proof:

- identity evidence map
- standing/governance handoff notes
- explicit non-claim that governance is not completed by the birthday itself

Current state: blocked with evidence pending accepted WP-19 proof.

### D9) WP-20 Coverage Validator

Expected proof:

- matrix, coverage, activation-ledger, and artifact-index rows agree on owner,
  command, status, and exact revision
- validator rejects missing artifact paths, duplicate owners, planned-as-passed
  status, synthetic proof, and unsupported platform claims

Current state: blocked with evidence until typed exact-head review and
publication make the validator evidence consumable outside the issue branch.

## Non-Claims

- This matrix does not prove legal personhood.
- This matrix does not prove production citizenship.
- This matrix does not complete constitutional governance.
- This matrix does not expose raw private state.
- This matrix does not turn cognitive profiles into public reputation or
  consciousness claims.
- This matrix does not prove full autonomous adaptive learning or unconstrained
  graph mutation.
- This matrix does not prove production WebSocket security, cross-polis
  networking, or signed/queryable trace completion.
- This matrix does not claim Observatory-owned work; Observatory integration is
  handled by its separate component/session.

## Reviewer Sign-Off Surface

Reviewers should receive this matrix, the feature coverage table, the activation
ledger, the AEE artifact index, validator output, negative-case report, and
residual-risk notes.

## Exit Criteria

- Every milestone claim has an accepted, blocked, deferred, or planned proof
  row with an artifact-index owner.
- No demo claims completion before exact-revision evidence and required
  negative proof exist.
- The validator rejects release-credit substitutions and synthetic proof.
