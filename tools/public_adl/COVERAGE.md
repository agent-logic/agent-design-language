# Public source coverage

`coverage.py` implements the public coverage producer for ADL #1223. Its selected
job is required by the stable `adl-coverage` aggregate; private owner obligations
remain separate until their retirement is reconciled. Passing its contract tests does not
establish an instrumented result or permit retirement of private test obligations.

The inventory assigns production files to the public CLI and five public
packages. Each package is measured separately with its exact Cargo selection;
private workspace members are not selected. Dependency measurements are assigned
to their owning package once. Missing packages, missing executable files,
unclassified zero-line files, duplicate measurements and unexpected repository
source fail the producer. Declaration-only exemptions are explicit in the
inventory. LLVM JSON and LCOV remain alongside the combined result.

Run from a committed checkout with an output directory outside the checkout:

```sh
python3 tools/public_adl/coverage.py --mode pull_request --base <base-commit> --output <fresh-output-directory>
python3 tools/public_adl/coverage.py --mode full --output <fresh-output-directory>
```

The existing policy remains 80% changed-file line coverage for PRs, and 90%
aggregate plus 80% per-file coverage for full validation. A source-retirement
percentage is not comparable to the former private-source denominator.

The runner records source HEAD, production file hashes, toolchain, selected
commands and the exact measured denominator. It requires a clean checkout before
and after measurement, excluding only native `.csdlc` operational metadata.
Synthetic facade tests use the instrumented ADL executable and the profiling
pattern declared by cargo-llvm-cov. Unix exec can discard pending facade counters;
the resulting measurement must be inspected rather than assuming those lines
were covered. Installed-package sandbox qualification remains a separate proof.

`python3 -B tools/public_adl/test_coverage.py` checks malformed/missing data,
zero-line bypasses, duplicate ownership, dependency accounting and the existing
threshold distinction. This is a deterministic PVF local-contract lane with no
network, provider, compiler or production mutation.
