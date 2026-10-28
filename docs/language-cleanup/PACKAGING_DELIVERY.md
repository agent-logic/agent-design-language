# Native package and public CLI contracts (#1349)

The 26 Python paths assigned to #1349 in the accepted inventory are replaced by
`tools/public_contracts`, a standalone Rust tool and focused native test suite.
No interpreter source is renamed, embedded, hidden with Linguist, or archived in
this change. Historical records remain provenance, not executable entrypoints.

## Behavior and caller mapping

- `runtime-dispatch`, `cli-refusals`, `session`, `codefriend-dispatch`,
  `runtime-owner`, and `codefriend-owner` exercise the real public CLI or the
  actual Rust owner-selection modules against synthetic layouts. They preserve
  literal arguments, statuses, output channels and explicit refusal checks.
- `godel` and `remote-distribution` retain exact source provenance, frozen archive
  hashes and byte reproduction, create-only extraction and refusal of changed,
  missing, duplicate, aliased or unsafe members. `remote-consumer` builds the
  verified distribution offline and rejects unexpected local dependencies.
- `transpiler-pack`, `transpiler-install`, and `transpiler-test` retain committed
  source selection, digest/member/mode checks and six installed demo cases.
- `compatibility` and its 29-case native suite preserve the seven-role contract,
  exact evidence, dependency obligations, bundle uniqueness and historical
  Runtime qualification limitations. They never grant acceptance or activation.
- `install-proof` and `install-validator --issue NUMBER` produce five-package
  evidence with macOS OS sandbox enforcement, five classified negatives, eight
  public consumer cases, 21 inert UTS cases and 23 canonical UTS cases. The
  network refusal probe is native Rust. No environment-only fallback exists.
- `uts-install` retains the narrower standalone crate packaging/consumer proof.
- `docs --self-test`, `resume-args --binary BASELINE`, and
  `portable-workflow --binary BASELINE` retain documentation, diagnostic and
  separate-build contracts. Native tests preserve package-runner argument/order
  checks, parser diagnostics and demo output preservation.

The public runner, coverage producers, CI, native validator adapters, Makefile,
current package documentation and cleanup aggregate use these replacements.
The immutable previous transitions are composed after `packaging-transition.json`;
its exact before/after blobs and preimage SHA-256 values bound these removals.

## Proof classification and limitations

Native unit/contract suites are PVF local_contract: deterministic local CPU and
scratch disk, no providers, cloud or lifecycle writes. Synthetic owner selection
is not installed product qualification. macOS sandbox tests are platform-gated;
the install proof fails closed on unsupported platforms. Public CLI coverage
continues to consume the instrumented public executable and its LLVM profile.

The local filesystem does not retain requested setuid permission bits. The owner
probe reports that individual 4755 case as an explicit platform limitation;
other mode and owner checks run. CI on a supporting filesystem must cover it.

Source-retirement's pre-existing podcast transition mismatch is tracked in
#1356. C-SDLC setup regressions #330/#331 remain separate, unimplemented tooling
issues; Git/gh delivery uses the operator-authorized fallback and does not claim
successful native finish or cleanup.

## Local validation

The initial committed candidate passed the 45 portable native tests and the
separate macOS sandbox test, all 29 compatibility adapter tests, all five archive
adapter tests, the 52 CI contract tests, strict Clippy, public CLI validation,
current docs checks, six resume comparisons, separate portable workflow build,
standalone UTS packaging, offline remote artifact consumption and the five-package
sandboxed install validator. The latter validated all 8/5/21/23 denominators above.

Independent review found one remaining source-retirement install adapter caller;
it now invokes the same native validator with issue 1223 and preserves failure
propagation. A focused CLI regression also pins compatibility refusal exit 2 and
its stderr event channel. Final candidate validation and CI remain separate from
these initial local results; no unrun check is represented as passed here.
