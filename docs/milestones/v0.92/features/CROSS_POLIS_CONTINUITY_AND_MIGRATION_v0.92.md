# v0.92 Feature: Cross-Polis Continuity And Migration Planning

## Status And Authority

- Milestone: `v0.92`
- Work package: WP-17 / issue `#5835`
- Status: implemented design contract; no operational migration claim
- Evidence baseline: merged WP-09, WP-10, and WP-16 outputs
- Detailed design: `docs/milestones/v0.92/design/CROSS_POLIS_CONTINUITY_TRANSFER_DESIGN_v0.92.md`

This feature defines how a later migration design may evaluate references to
landed birthday evidence. It does not move an agent, duplicate state, establish
cross-polis trust, or decide citizenship or standing.

## Decision Boundary

WP-17 owns the documentation-only classification of continuity-transfer
inputs. WP-04 retains runtime, storage, networking, distributed-polis, fencing,
recovery, and operational migration mechanisms. v0.93 retains citizenship,
standing, rights, duties, and governance decisions. A row marked `candidate`
below is therefore an input to a future decision, not permission to transfer.

The only admissible movement unit in this contract is a repository-relative,
digest-bound evidence reference. Raw state, mutable provider state, process
state, credentials, private memory, signing material, and copied snapshots are
not movement units.

## Transfer Dispositions

- `candidate`: a digest-bound reference may be presented to a future verifier.
- `local_only`: the value or state remains in its source authority domain.
- `quarantine`: conflicting, incomplete, unverifiable, or ambiguous evidence
  is retained for review but cannot advance continuity.
- `defer`: a future transport-security or governance decision is required
  before the reference may be consumed.
- `reject`: the input cannot be used as continuity evidence.

No disposition changes ownership of the underlying artifact.

## Field-Level Continuity-Transfer Matrix

Every row names the landed schema or proof surface that supplies it. `Yes` in
the governance or transport column means the future consumer must stop until
that downstream authority exists.

| Artifact / field | Landed source and lineage binding | Portable reference | Local-only state | Requires v0.93 governance | Requires transport security | Redaction posture | Fail-closed disposition |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Stable name | `adl.birthday.identity_record.v2`; bind `stable_name` to `identity_root` and `record_sha256` | Candidate: identity-record digest plus stable-name field | Alias registry mutation and naming authority | Yes, before social or legal recognition | Yes, before any remote acceptance | Public only when the governed projection permits it | Reject an unbound display name, invalid canonical record, or alias-authority conflict |
| Identity root | `adl.birthday.identity_record.v2`; bind `identity_root`, origin, provenance, governed projection, and `record_sha256` | Candidate: identity-record digest and identity root | Private lineage record, trust roots, signer registries, and projection policy | Yes, before standing or citizenship effects | Yes, with authenticated source and target authority | Never export raw private lineage or trust material | Reject substituted roots; quarantine conflicting canonical records for one root |
| Continuity head | `adl.birthday.continuity_record.v1`; bind `identity_root`, `identity_record_sha256`, predecessor, authority-context digest, ordered cycles, and `record_sha256` | Candidate: continuity-record digest, head, and ordered cycle references | Checkpoint contents, signer keys, and source runtime ledger | Yes, before deciding consequences of a verified lineage | Yes, including freshness, replay, and source/target authentication | Cycle references only; no checkpoint payload | Reject missing/reordered/duplicate cycles or root substitution; quarantine competing valid heads |
| Memory-grounding references | `adl.memory_palace.context_packet.v1`; bind identity/continuity digests, trace reference, temporal anchors, visibility, redaction-policy digest, and packet digest | Candidate: packet digest and governed public/redacted citations | Payloads marked private/raw-private, source rooms, working-set cache, and retrieval state | Yes, before a target may change disclosure or retention | Yes, before a target resolves any protected citation | Public or explicitly redacted projections only | Reject raw-private content, unsafe paths, stale anchors, or authority mismatch; quarantine unresolved citations |
| Capability envelope | `adl.capability_envelope.v1`; bind identity and birthday digests, evidence, policy digest, limits, grants, denials, and envelope digest | Candidate: envelope digest plus bounded declarations and denials | Credentials, provider sessions, tool handles, runtime grants, and policy authority | Yes, before granting target-polis authority | Yes, before accepting provider, tool, or skill assertions | Export identifiers, limits, denials, and digest-bound provenance; never secrets | Treat all grants as non-operative on arrival; reject limit escalation, missing denial, stale evidence, or unsupported authority |
| Cognitive profile | `adl.cognitive_profile.v1` and `adl.cognitive_profile.public.v1`; bind identity, continuity, capability, policy, evidence, revision, and profile digest | Candidate: public projection or governed redacted profile reference | Private evidence fields, authority registry, policy signing material, and inferred traits | Yes, before any standing, reputation, rights, or personhood inference | Yes, before resolving non-public evidence | Prefer public projection; preserve `no_personhood_inference`, `no_reputation_inference`, and `no_rights_inference` | Reject self-authorized policy/evidence, stale revision chains, private-field disclosure, or profile-as-identity claims |
| Adaptive-learning history | `adl.adaptive_learning.history.v1`; bind profile/capability/policy digests, sequence, prior history, mutation evidence, rollback, and history digest | Defer: digest-bound history reference may be reviewed only after continuity and governance checks | Mutable reasoning graph, pending state, execution cache, grants, and rollback state | Yes, before accepting learned effects in another polis | Yes, including ordering, replay, and principal isolation | Rationale and bounded decision metadata only; private state stays local | Reject copied mutable state, missing predecessor, unauthorized grant, replay, or rollback mismatch; quarantine divergent histories |
| ACIP transport-readiness proof | Replacement authority `agent-logic/agent-design-language#209` / PR `#215`; bind `.csdlc/evidence/209/local-validation-manifest.json`, `.csdlc/evidence/209/native-validation-manifest.json`, exact merge `a77519c3fca9f64752af41c9a2ebd396468891f7`, and `adl.acip_native_platform_proof.v2` receipts | Defer: exact reviewed/merged proof and public-schema references only | Live session state, replay table, authenticated principal state, payload contents, and pressure reservations | No governance effect by itself | Yes; v0.92 carrier proof is not cross-polis trust | Schemas may be public; message contents remain governed | Reject superseded #5832/PR 76 authority, carrier-as-authority, unauthenticated source, wrong replay domain, stale proof, or schema/content-access conflation |
| Witness set | `adl.birth_witness.set.v1`; bind candidate, evidence set, roster, policy, attestations, and witness-set digest | Candidate: validated public witness summaries and witness-set digest | Private witness evidence, signing keys, and witness-policy registry | Yes, before a target assigns institutional meaning | Yes, before trusting remote attestations or revocation state | Public summaries only; no private witness material | Reject missing roles, duplicate identity/key/role, signature failure, roster mismatch, or candidate mismatch |
| Citizen-facing receipt | `adl.birth_witness.citizen_receipt.v1`; bind candidate and witness-set digests, disposition, `birth_event_status`, public evidence, caveats, and receipt digest | Candidate: receipt and its public evidence references | Any non-public evidence behind the receipt | Yes; the receipt grants no citizenship, standing, rights, or personhood | Yes, before remote provenance is trusted | Public evidence and caveats only | Reject a receipt that claims birth authority, omits caveats, exposes private evidence, or mismatches its witness set |
| WP-16 review inventory | `adl.v092.first-birthday-review-evidence.v1`; bind issue/code repositories, PR, reviewed revision, merge commit, evidence path/digest, and public projection | Candidate: exact inventory entry and packet digest | Reviewer working material and any governed child evidence | Yes, before policy consequences are attached | Yes, before remote retrieval is trusted | Public projection must remain narrower than retained proof | Reject stale digest, nonterminal/unreviewed authority, wrong repository, non-ancestral merge, or publication overclaim |

## Admission Algorithm For Future Consumers

A future cross-polis design must evaluate one proposed reference as follows:

1. Classify it against exactly one matrix row; unknown types are rejected.
2. Resolve the source repository, revision, authority context, and redaction
   policy from the trusted anchor set in the detailed design. Caller-supplied
   values are claims to compare, never authority to establish an anchor.
3. Resolve the repository-relative source path at that accepted revision and
   recompute its digest.
4. Verify the source schema, canonical record digest, identity root, continuity
   head, predecessor chain, and authority context required by that row.
5. Verify the source issue/PR review and merge ancestry when the reference is a
   retained work-package proof.
6. Apply the row's redaction rule before any content leaves the source domain.
7. Stop with `defer` when transport-security or v0.93 governance authority is
   required but absent.
8. Place competing heads, contradictory witness sets, or unresolved lineage in
   quarantine without choosing a winner.
9. Emit only an admission decision and evidence digest. Never reconstruct,
   copy, activate, or mutate source state as part of this design contract.

## Ambiguity, Copy, And Privacy Rules

- Byte-identical state in a second location is still copied state. It does not
  prove a continuous identity, authorized movement, or an accepted predecessor.
- Two evidence-backed heads for one identity root are not merged by timestamp,
  majority, display name, or narrative. They remain quarantined pending a
  later authority decision.
- A public schema does not make protected message contents public.
- A redacted reference is not permission to retrieve its private source.
- Missing evidence is not interpreted as consent, standing, continuity, or
  approval.

## What This Contract Proves

It proves that the landed v0.92 birthday evidence can be classified
deterministically into candidate references, local-only state, rejected state,
and downstream decision gates. It also makes copied-state, lineage ambiguity,
redaction, and ownership boundaries reviewable.

## Non-Claims

- No production migration or federation is implemented.
- No cross-polis key lifecycle, encryption, rotation, revocation, or trust mesh
  is complete.
- No identity, capability, learning state, or private memory is transferable by
  virtue of this document.
- No citizenship, standing, reputation, rights, duties, legal personhood, or
  consciousness determination is made.
- No birthday demo, release, or publication is authorized.

## Validation And Evidence

The deterministic positive and negative contract checks are owned by
`.csdlc/evidence/5835/validate-continuity-transfer.rb`. Dependency authority is
retained in `.csdlc/evidence/5835/dependency-authority.json` and is checked
against current Git ancestry and the merged WP-16 inventory.

## Downstream Handoff

- WP-04 may later implement mechanics only under its own runtime, storage,
  transport, fencing, recovery, and rollback contracts.
- v0.93 may consume candidate references to design governance consequences; it
  must not reinterpret a candidate or receipt as citizenship or standing.
- Later transport work must establish cross-polis authentication, key custody,
  replay/freshness, per-message authorization, and revocation before protected
  references can be resolved remotely.
