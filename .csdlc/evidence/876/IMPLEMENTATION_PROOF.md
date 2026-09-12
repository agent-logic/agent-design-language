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

## PR953 user URL/Bedrock alias repair

User findings at `9495c0e94e325c4e9d24050ab2c7213eb91b4f51` are retained
in REVIEW.md. Both account digest aliases now share string/64-hex validation;
constructor precedence is unchanged. URL parsing rejects userinfo, decoded
credential query names and explicit credential markers in decoded values.
Ordinary query values do not receive the opaque-length heuristic. No endpoint
is contacted during validation and no credential value is resolved.

Four current provider-definition tests pass (`url-alias-final3-tests.log`):
production profile-loader admission of both Bedrock aliases; malformed digest
negatives; initial endpoint/base URL rejection and safe URL positives; seven
actual rejected watcher updates with unchanged generation/digest and real
last-known-good dispatch after every rejection. Existing reference and nested
credential tests remain included. Initial alias fixture attempts incorrectly
used explicit Bedrock type unsupported by the document validator; failed logs
are preserved in url-alias-final-tests.log and url-alias-final2-tests.log. The
fixture now uses supported bedrock:nova-lite-v1, without changing production
provider compatibility. Earlier passing four-test run is url-alias-tests.log.

PVF remains bounded local provider integration/contract. No AWS, paid inference,
external provider qualification or model-health probes were performed. Logs are
retained raw, including trailing blank lines; whitespace checks apply to changed
source/docs only. Independent exact-head review and current-head CI are pending.
