# Disable implicit curl configuration during observation

This #867 / PR #948 supplement addresses the P2 finding against revision
`6ebfbd7c5226f536d86a317ebcf9db7f5147aca3`. Curl reads its default configuration
even with `--config -`, and may discover a home directory through the account
database when `HOME` is absent. Its `-q` option disables that lookup only when
placed first. See the [official curl manual](https://curl.se/docs/manpage.html#-q).

The read-only adapter now supplies `-q` before all invocation arguments, retaining
the explicit stdin credential configuration and minimal child environment.
Operational mutation adapters are outside this bounded repair.

## Proving regression

`observational_curl_disables_default_config_before_other_arguments` runs real curl
against a local `file://` payload. A test wrapper points curl to an isolated
`CURL_HOME` containing a synthetic default configuration that redirects output to
a fixture file. The control proves that default configuration writes the file.
The observation must instead return the payload on stdout without creating it.
No network, operator configuration file, or live credential is used. The separate
stdin/environment regression checks that `-q` is exactly the first argument.

`before.stdout` retains the regression failing because observation wrote a file;
`focused.stdout` retains all three observation transport tests passing after the
fix. The PVF manifest classifies the new real-curl fixture as bounded local proof.

## Evidence boundary

The source commit, candidate digest, changed source hashes, full suite and strict
Clippy results, and refreshed 17-attempt corpus are bound in `index.json`. Tests
ran on the working repair subsequently committed unchanged; corpus provenance
records its base HEAD and working diff rather than claiming a clean-HEAD run.
The original proof and CI-repair supplement remain historical evidence.

Published logs normalize host display prefixes and trailing blank lines only;
original hashes are retained. Synthetic clocks, fake remote transport, synthetic
review principals, simulated external merge and the separate clean control retain
the baseline's explicit limits. This local supplement does not assert new-head
CI completion, final independent review, merge, closeout, or live activation.
