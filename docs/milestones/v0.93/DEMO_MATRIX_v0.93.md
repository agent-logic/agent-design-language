# v0.93 Demo Matrix: Candidate Constitutional Governance Proofs

## Status

Candidate demo planning only. Commands and artifacts will be finalized when the
v0.93 implementation WPs exist.

## Purpose

The v0.93 demo program should prove that constitutional citizenship, bounded
social cognition, and polis governance are evidence-bearing runtime behavior,
not rhetoric. It should also prove that enterprise-security controls are
reviewable behavior rather than perimeter language.

## Demo Coverage Summary

| Demo ID | Candidate demo | Milestone claim | Primary proof surface | Status |
| --- | --- | --- | --- | --- |
| D1 | Constitutional review of a challenged action | A citizen action can be evaluated against rights, duties, policy, trace, outcome, and standing evidence. | Review packet with finding, evidence references, and appeal state. | Planned candidate |
| D2 | Standing degradation and restoration | Standing changes require evidence and can include restoration when conditions are met. | Standing transition fixture and reviewer summary. | Planned candidate |
| D3 | Human guest versus citizen-mode boundary | Human input does not become citizen action unless mediated through CSM identity, Freedom Gate, signed trace, and temporal anchoring. | Two-case fixture showing guest-only and mediated citizen-mode paths. | Planned candidate |
| D4 | Delegated and upstream authority chain | Delegated or upstream-escalated action is allowed or denied based on explicit authority, policy, provenance, and verification boundaries. | Delegation/upstream-delegation/IAM fixture with allow/deny decision events. | Planned candidate |
| D5 | Communication without inspection | Communication does not grant private-state inspection. | Communication event, redacted projection, and failed inspection attempt. | Planned candidate |
| D6 | ToM and reputation boundary | Private ToM can inform later cognition only through authorized, redacted, evidence-grounded projections; reputation is not the private model. | Private model fixture, signed update event, reputation projection, redaction report, denied unauthorized inspection. | Planned candidate |
| D7 | Conflict and decay in shared social memory | Stale or contradictory social cognition is marked, downgraded, and reviewable rather than silently overwritten. | Conflict fixture, decay event, social-memory projection, arbitration impact note. | Planned candidate |
| D8 | Polis governance health packet | Reviewers can inspect governance health without scalar moral verdicts, private ToM leakage, or raw private-state exposure. | Governance report with evidence references, caveats, and redactions. | Planned candidate |
| D9 | Zero-trust denied action | A citizen, service, tool, or operator action crossing a trust boundary is denied without identity, standing, capability, and policy authority. | Deny-by-default fixture with policy decision, boundary record, and redacted explanation. | Planned candidate |
| D10 | Key rotation and revocation | Cryptographic trust changes when a key is rotated or revoked, and stale signatures/messages/sealed-state access fail. | Key lifecycle fixture, accepted-before/denied-after cases, and audit record. | Planned candidate |
| D11 | Audit and incident evidence packet | Security review can inspect what happened without leaking private state or claiming external certification. | Tamper-evident audit entries, incident record, redaction report, and reviewer packet. | Planned candidate |
| D12 | Isolation and data-governance leakage prevention | Cross-polis, cross-tenant, or cross-citizen data access is blocked or redacted according to classification and retention policy. | Isolation negative case, data-classification record, retention/projection decision, and denial proof. | Planned candidate |

## Coverage Rules

- Every demo must identify the actor class: citizen, guest, human provider,
  operator, service actor, tool, or external counterparty.
- Every governance finding must cite evidence rather than narrative alone.
- Every private-state boundary must have a redaction or denial proof.
- Every private-ToM boundary must have a redaction or denial proof.
- Every denial must explain authority and policy without leaking protected data.
- Every security demo must cite the trust boundary, policy decision, key or
  secret lifecycle state where relevant, audit evidence, and redaction outcome.
- Demo outputs should distinguish engineering evidence from policy
  interpretation.

## Demo Details

### D1) Constitutional Review Of A Challenged Action

The demo should replay a synthetic incident where a citizen action is challenged
under the polis constitution.

Expected proof:

- citizen identity and standing snapshot
- relevant policy and rights/duties context
- moral trace event references
- outcome and attribution references
- finding and appeal disposition
- redaction notes

### D2) Standing Degradation And Restoration

The demo should show a standing transition that is neither arbitrary nor
permanent by default.

Expected proof:

- evidence-backed degradation or restriction
- challenge or review context
- restoration criteria
- restored or still-restricted disposition with rationale

### D3) Human Guest Versus Citizen-Mode Boundary

The demo should show that human participation is allowed while preserving the
citizen boundary.

Expected proof:

- guest-mode human input remains guest/operator activity
- citizen-mode action requires identity binding, Freedom Gate mediation, signed
  trace, and temporal anchoring
- direct out-of-band human action is rejected as citizen conduct

### D4) Delegated And Upstream Authority Chain

The demo should show a delegated or upstream-escalated action request where
authority is either accepted or denied based on explicit policy, provenance,
and verification boundaries.

Expected proof:

- actor identity and standing
- delegation source
- capability or IAM record
- allowed/denied action decision
- trace evidence for the final disposition

### D5) Communication Without Inspection

The demo should show two citizens or a citizen and guest communicating without
private-state inspection.

Expected proof:

- governed communication event
- consent or authorization record where needed
- redacted projection
- failed inspection attempt

### D6) ToM And Reputation Boundary

The demo should show that a private ToM model can be updated from evidence and
then projected into reputation only through explicit policy.

Expected proof:

- signed ToM update event with evidence references
- model diff with confidence basis
- private model retained as non-public evidence
- reputation projection with redaction notes
- unauthorized inspection refusal

### D7) Conflict And Decay In Shared Social Memory

The demo should show that social cognition remains uncertain and temporal.

Expected proof:

- two contradictory or aging model entries
- conflict group or decay event
- downgraded or unresolved confidence state
- shared social-memory projection that preserves uncertainty
- arbitration or review note showing that stale/conflicted ToM does not become
  a final verdict

### D8) Polis Governance Health Packet

The demo should generate a review packet over a small polis state.

Expected proof:

- standing distribution
- social cognition and reputation projection summary
- open challenges or appeals
- governance findings
- redaction report
- caveats and unresolved risks

### D9) Zero-Trust Denied Action

The demo should show that no actor receives implicit trust merely because it is
inside the polis.

Expected proof:

- actor identity and standing
- requested boundary crossing
- required capability, IAM, delegation, or tool authority
- deny-by-default decision
- redacted explanation that does not leak protected state

### D10) Key Rotation And Revocation

The demo should show that cryptographic trust is lifecycle-managed.

Expected proof:

- initial accepted signed or encrypted action
- key rotation or revocation record
- stale signature, message, or sealed-state access denied after revocation
- audit record connecting the trust change to the denial

### D11) Audit And Incident Evidence Packet

The demo should generate a security review packet for a synthetic incident.

Expected proof:

- tamper-evident audit entries
- incident scope and actor boundary
- policy, key, isolation, or provenance evidence
- redaction report
- explicit non-certification language

### D12) Isolation And Data-Governance Leakage Prevention

The demo should show a blocked or redacted data access across a protected
boundary.

Expected proof:

- data classification
- tenant/polis/citizen boundary
- retention, deletion, or projection rule
- denied or redacted access result
- leakage-prevention assertion

## Non-Claims

- These demos do not prove production citizenship.
- These demos do not establish legal personhood.
- These demos do not replace v0.91 moral trace or v0.92 identity work.
- These demos do not expose raw private state.
- These demos do not expose raw private ToM.
- These demos do not make reputation, standing, or constitutional judgment from
  private ToM without authority and redaction.
- These demos do not prove external enterprise certification or production
  compliance approval.

## Metadata

Planning template set: 1.1.0. Target: v0.93. Authoring issue: #1047; current reconciliation: #922 in v0.92.2. Accountable planning role: milestone owner; named implementation owners are assigned before opening.

## How To Use

D1-D12 retain the governance/security scenarios above. D13-D16 extend coverage to guilds, migration, mandatory Runtime v4 and CodeFriend launch. Each owning work package supplies executable commands and exact artifacts when implemented; this planning issue does not claim those runs exist.

## Scope

| ID | Additional demonstration | Owner / candidate | Required positive and refusal/recovery evidence |
|---|---|---|---|
| D13 | Governed guild action | Runtime / GOV-15, DEMO-GOV | Membership-authorized action succeeds; revoked member and excess authority fail with trace |
| D14 | Independent products after split | ADL coordination / RD-11 | Public clean checkout without private access; each private product installed independently; authentic C-SDLC recovery and supported rollback |
| D15 | Runtime v4 complete plugin lifecycle | Runtime / RV-08 | Native/process/WASM parity, successful state/config migration, two-phase reconfiguration, dependency-safe removal, fenced crash recovery and refused transition preserving prior generation |
| D16 | CodeFriend launch journey | CodeFriend with website / CF-05 through CF-07 | ADL/external-OSS/PR review, source-backed diagram, meaningful test, documentation, report quality and privacy refusals; authorized live onboarding plus rollback |

## Runtime Preconditions

D1-D13 consume the accepted Runtime v4 artifacts after RD-11 and RV-08. D14 qualifies extraction of the accepted predecessor product baseline before feature work. D15 qualifies the completed Runtime v4 candidate. D16 identifies the installed CodeFriend/Runtime/site versions, approved repository classes, audience and resource limits. Real provider/cloud/customer activity requires its declared authorization.

## Cross-Demo Validation

Match every demo to owning candidate acceptance and the release lockset. Run the real consumer, retain actual scenario counts and distinguish synthetic security mechanics from enterprise qualification. Missing negatives, incompatible artifacts, stale evidence or zero executed cases block the owning result.

## Determinism Evidence

Record immutable source refs, fixture hashes, policy/config versions, seeds where used and expected transition order. Provider output may vary; validate evidence/provenance and behavioral invariants rather than asserting identical natural-language output. Record retry, cancellation and recovery dispositions.

## Reviewer Sign-Off Surface

Each reviewer records exact versions, scenarios observed, pass/fail/skipped outcomes, findings, limitations and disposition. TAIL-01 consumes independent QUALIFY evidence. An author demo or generated packet is not independent release acceptance.

## Exit Criteria

All applicable D1-D16 scenarios have current evidence tied to their actual consumer and no unresolved release-blocking finding. Publication/customer admission remains separately authorized; preview proof alone cannot close CF-07.


## Required #922 demonstrations

| ID | Outcome | Owner | Proof |
|---|---|---|---|
| D17 | Fill, preview, edit and export branded artifacts including full 4+1 | CT-05 / CF-05 | Every enabled kind represented; missing fields, bad version and malformed output rejected |
| D18 | Same-identity migration with interrupted handover | CM-02 / CM-04 | One active holder, replay denial, recoverable custody and lineage |
| D19 | Governed descendant creation | CM-03 / CM-04 | Distinct identity, selected inheritance, scope/privacy, idempotent retry |
| D20 | Temporal and inherited governance integration | GOV-01 / GOV-11 / GOV-14 / INTEGRATE | Cognitive/instinct action admission; remote refusal/replay; expired/missed/causally invalid commitments |
