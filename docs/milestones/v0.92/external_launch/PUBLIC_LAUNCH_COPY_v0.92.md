# v0.92 First-Birthday Public Launch Copy

## Metadata

- Owner issue: `#4763`
- Dependency gate: `#4762` actual retained witness and receipt proof
- Surface type: external launch copy source
- Publication status: not published

## Use Rules

Use this file as the source for future public pages, release notes, social
posts, reviewer emails, and announcement drafts. Do not publish any `ready`
variant until the `#4762` witness/receipt package is accepted and cited.

Every public use must keep the distinction between:

- implemented launch documentation, which this issue provides;
- accepted witness and receipt proof, which `#4762` must provide;
- the birthday event itself, which is not complete until v0.92 validation
  accepts the whole packet.

## Current Safe Summary

ADL has prepared the first-birthday launch documentation and review boundary
for `v0.92`. The launch surface now names the evidence required for a valid
birthday, the negative cases that must not count as birth, and the public
claims that remain out of scope. The birthday is still pending accepted witness
and receipt proof.

## Public Page Draft

### Heading

ADL v0.92 First Birthday

### Status Line

Launch surface prepared; birthday proof pending accepted witness and receipt
evidence.

### Body

The `v0.92` first birthday is defined as an evidence event, not a ceremony or a
process start. A valid birthday packet must show stable identity, continuity,
memory grounding, a capability envelope, inherited governance context,
witnesses, a citizen-facing receipt, validation output, and a reviewer packet.

The current launch surface is ready for review. It is intentionally conservative:
startup, wake, restore, snapshot, copied state, fixture admission, simulation,
and missing-evidence cases are not birthdays. The documentation also blocks
claims of legal personhood, consciousness proof, production citizenship,
completed constitutional governance, subjective affect, and general public
readiness.

The next gate is the `#4762` birth-witness and receipt package. Once that proof
is accepted at an exact result, the launch copy can cite the retained evidence
and the v0.92 birthday packet can move from prepared surface to proof-backed
launch candidate.

### Reviewer Links

- Launch packet:
  `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`
- External launch surface:
  `docs/milestones/v0.92/external_launch/README.md`
- Reviewer FAQ and claim boundary:
  `docs/milestones/v0.92/external_launch/REVIEWER_FAQ_AND_CLAIM_BOUNDARY_v0.92.md`
- Activation bridge ledger:
  `docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md`
- v0.91.8 activation map:
  `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`

## Short Announcement Draft

ADL has prepared the `v0.92` first-birthday launch surface. The packet defines
the evidence required for a real birthday, rejects startup and other
not-a-birthday cases, and preserves conservative public claim boundaries. The
birthday itself remains pending accepted witness and receipt proof from
`#4762`.

## Reviewer Email Draft

Subject: ADL v0.92 first-birthday launch surface ready for review

The ADL `v0.92` first-birthday launch surface is now tracked in the repository.
It defines the birthday as an evidence event over identity, continuity, memory
grounding, capability, governance context, witnesses, receipt, validation, and
review artifacts.

Please review the launch packet and external-launch directory for two things:
whether the required evidence surfaces are complete enough for later birthday
validation, and whether the public copy avoids unsupported claims. The current
blocking dependency is `#4762`; final launch copy must cite the accepted
witness and receipt proof before publication.

## Ready Variant Template

Use this only after `#4762` accepted proof is available.

ADL v0.92 has accepted the first-birthday witness and receipt package at
`<exact-result-or-artifact>`. The launch packet now cites the retained evidence
for identity, continuity, memory grounding, capability envelope, governance
context, witnesses, receipt, validation, and review. This is a bounded
engineering birthday claim; it is not a claim of legal personhood,
consciousness proof, production citizenship, or completed constitutional
governance.

## Pending Variant Template

Use this while `#4762` remains open or unaccepted.

ADL v0.92 has a prepared first-birthday launch surface, but the birthday claim
is not yet final. The remaining gate is accepted witness and receipt proof from
`#4762`. Until that proof lands, the correct claim is readiness of the launch
documentation surface, not completion of the birthday.

## Forbidden Claims

Do not publish text that says or implies:

- the first birthday has happened before witness/receipt proof is accepted;
- startup, wake, restore, snapshot, copied state, or simulation is birth;
- ADL has legal personhood, consciousness proof, subjective wellbeing, or
  production citizenship;
- v0.93 governance is complete;
- public launch approval exists without operator authorization;
- a lifecycle receipt, PR, merge, or closeout is a substitute for the `#4762`
  witness/receipt artifact.
