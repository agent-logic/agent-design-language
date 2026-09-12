# #876 implementation and bounded proof

Base: `57a82b08bf687afd0b1573fd8d78f4b2b2268dba` (merged #854/PR941).
Merged prerequisites #622/PR646 and #864/PR865 are ancestors. Work is bound to
`codex/876-provider-definitions`; primary main remained inspection-only.

## Findings preserved and corrections

- Profile-only definitions were validated by the substrate before profile
  expansion, causing the empty unexpanded type to be rejected. Concrete
  substrate and adapter admission now run after expansion and before promotion.
- Credential recognition was restricted to named scalar containers. Neutral
  nested object/array strings now undergo recognition. Explicit typed model,
  shadow-data and credential-reference locations preserve their semantics;
  neutral nested lookalikes do not gain exemptions. Public account SHA256 is
  validated as a digest. Raw malformed input could escape via parser errors;
  loader errors and watcher diagnostics now retain bounded categories only.
- Expanded unusable endpoints could pass transport classification. Existing
  adapter constructors now validate the candidate before promotion, without
  calling completion. Constructors retain reference names; actual credential
  resolution, network/process execution and shadow evidence writing occur on
  completion. Bedrock region/profile/account-hash and Ollama timeout defaults
  remain non-secret constructor configuration; no new auth-availability gate.

## Executed proof

- Three new `provider_definitions` tests pass: real production runner + loopback
  request/release barrier; invalid initial and watcher-parser matrix with secret
  redaction; reference and local-shadow admission without execution.
- A mutation restoring pre-expansion validation and disabling recursive scalar
  checks made both behavioral regressions fail (2 failed, 1 passed). Restoring
  the repair made all three pass (0.43 seconds). Evidence:
  `original-defect-mutation.log`, `repaired-regression-proof.log`.
- Final source proof passes 25 focused tests: 3 definition regressions, 7
  existing reload tests, 1 existing in-flight test, and 14 profile tests.
  Clippy `--lib -- -D warnings`, formatting and diff whitespace checks pass.
  Logs: `final-definitions.log`, `final-reload.log`, `final-inflight.log`,
  `final-profiles.log`, `clippy-final.log`. The first clippy run identified a
  nonminimal boolean; the equivalent simplified condition passes lint.
- The production fixture observes model and temperature on the exact old/new
  endpoints, preserves an old in-flight request, samples a complete two-provider
  map concurrently, and executes the retained new snapshot after an invalid
  expanded endpoint. Exactly one old and two new requests prove no reload probes.
- Initial fixture failure is preserved in `production-dispatch.log`: accepted
  sockets inherited nonblocking mode on macOS. Explicit blocking mode with a
  bounded read timeout corrected the fixture; no production workaround or gate
  weakening was used. The first shadow-admission check also caught rejection of
  a legitimate long evidence filename; exact declared-field handling fixed it.

PVF: deterministic local integration/contract, bounded CPU/filesystem/loopback,
required provider-platform gate. No paid inference, deployment, hosted-provider
qualification, lifecycle rewrite, second registry or second watcher. Existing
logging destinations remain unchanged; loader rejection text excludes candidate
values and paths. Independent review and current-head GitHub CI remain required.
