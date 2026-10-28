# RD13 — transferred Sprint 1 issue identities

The six existing destination-owned issues have moved after RD12's accepted
bootstrap (PR #1202, merge `a59d3048538f4a4408b4182e0cd6c0a804a6e46b`).
`issue-map.json` retains authenticated source/destination observations, transfer
events, original-URL redirects, dependencies and source journal digests. No
replacement issues were created.

| Work package | Original ADL issue | Current issue |
| --- | --- | --- |
| RD04 C-SDLC | [1186](https://github.com/agent-logic/agent-design-language/issues/1186) | [cognitive-sdlc#1](https://github.com/agent-logic/cognitive-sdlc/issues/1) |
| RD05 Runtime | [1187](https://github.com/agent-logic/agent-design-language/issues/1187) | [agent-logic-runtime#1](https://github.com/agent-logic/agent-logic-runtime/issues/1) |
| RD06 Infrastructure | [1188](https://github.com/agent-logic/agent-design-language/issues/1188) | [agent-logic-infrastructure#1](https://github.com/agent-logic/agent-logic-infrastructure/issues/1) |
| RD09 Enterprise security | [1189](https://github.com/agent-logic/agent-design-language/issues/1189) | [agent-logic-enterprise-security#1](https://github.com/agent-logic/agent-logic-enterprise-security/issues/1) |
| RD10 CodeFriend | [1190](https://github.com/agent-logic/agent-design-language/issues/1190) | [codefriend#1](https://github.com/agent-logic/codefriend/issues/1) |
| RD08 Demo qualification | [1191](https://github.com/agent-logic/agent-design-language/issues/1191) | [agent-logic-runtime#2](https://github.com/agent-logic/agent-logic-runtime/issues/2) |

## Dependencies and retained evidence

RD04 depends on ADL #1184. RD05 depends on ADL #1185 and the moved RD04.
RD06, RD09 and RD10 each depend on moved RD05. RD08 depends on moved RD06,
RD09 and RD10. ADL RD07 (#1192) still depends on moved RD08. All six retain
ADL #1180 as their sprint parent. The map records all nine outgoing dependency
edges and the incoming RD07 edge with full original and resolved URLs.
The original URLs redirect to the authenticated current issues. GitHub also
rewrote some references to their new destination URLs during transfer; both
forms resolve to the mapped identity. This records body-declared planning
relationships, not a claim that GitHub structured dependency APIs were populated.

WP01 #1178, umbrella #1180, RD01 #1181, RD02 #1182, RD12 #1183, RD13 #1184,
RD03 #1185, RD07 #1192 and RD11 #1193 stay in ADL. Historical PRs, evidence
and source journals retain their original location. The old execution plan's
`existing_issue` fields remain historical creation identities; use this map to
resolve the current owner. Do not resume mutations through stale ADL issue
identities or retag/copy their semantic journals into a destination.

## Actual transfer behavior and corrections

GitHub assigned new database and node IDs. Continuity is supported by the original
URL redirect, a destination `transferred` timeline event, retained title, creation
time, substantive body and assignees. All six original comment sets were empty;
this is not a nonempty-comment transfer qualification. Pre-transfer source
journals remain unchanged after dispatch; their exact generations and hashes
are retained in the map.

GitHub dropped version labels and milestone placement where the destination did
not contain them. Destination metadata was restored to `version:v0.93.1` and
milestone `v0.93.1`, then read back. This is explicit correction, not an assertion
that metadata survived unchanged. Destination milestone descriptions identify
the transferred Sprint 1 scope; the original milestone identity remains source
history.

Source issue references were qualified through the native issue editor before
transfer. An overbroad initial conversion incorrectly linked the session label
`Worker #1`; that conversion was corrected. GitHub also qualified that label
during transfer, so every final body was checked and corrected to `Worker #1`.
One post-transfer readback observed a short redirect propagation delay; only
read-only observation was repeated. No transfer was dispatched twice.

The authenticated administrative transport is GitHub's existing transfer route,
which has no native issue-transfer action. Destination metadata restoration uses
existing administration against the new repositories, where native lifecycle
authority is not installed. This does not invent native transfer receipts or
convert the preserved ADL source journals into destination records.

## Validation and handoff boundary

PVF: `planning_contract`, deterministic offline, bounded local CPU/files,
required RD13 acceptance gate. Run:

```sh
cargo test --offline --manifest-path csdlc-v3/Cargo.toml --test rd13_1184_mapping
```

Four focused tests check the six-row map and reject duplicate source/destination
identity, missing outgoing/incoming dependency, wrong repository, and lost
milestone metadata. These tests check recorded evidence consistency; they do not
replace the authenticated remote observations or claim GitHub is immutable.
Raw command/readback evidence remains in the issue-local evidence directory;
the tracked map includes selected observations and SHA-256 references without
credentials or machine-local paths.

Destination native lifecycle is **not initialized**. Portable C-SDLC installation
and independent lifecycle/recovery qualification belong to RD04. That installation
is not an RD13 mapping prerequisite. No source extraction, product qualification,
release, deployment, provider spending or later-sprint issue creation is claimed.
