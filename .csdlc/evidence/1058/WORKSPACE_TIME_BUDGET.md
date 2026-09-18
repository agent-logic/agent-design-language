# Hosted workspace time budget evidence

Run35385414984 at8cf88f563859ca80fd64ffc75324f6586e224ba8 exhausted its30-minute job envelope. Retained artifact adl-coverage-workspace-35385414984-1 shows:

- Cold instrumented compilation:19m45s.
- Partition Cargo startup:0.75s and1.02s; redundant compilation is resolved.
- Partition1:1399 executed,1398 passed,1 failed;467.932s.
- Partition2:1370 executed and passed;380.253s.
- Sole assertion failure: Cargo ANSI color hid the literal Fresh provenance-probe. Reviewed successor afb3ce4f8b fixes this with --color never and passed12 server tests with outer CARGO_TERM_COLOR=always.
- Coverage step started19:24:27Z, ended19:52:37Z, elapsed1690s. The job began19:22:37Z; its30-minute envelope left no reporting/cleanup headroom. A report was saved before cancellation.

Bounded repair:35-minute hosted workspace job envelope only. Existing per-test120/240s bounds, test selection, coverage thresholds and fail-closed aggregation are unchanged. Native proof, independent review and fresh hosted CI remain required. This is not product acceptance or sprint completion.

Raw downloaded logs remain in ci-8cf88-workspace-artifacts under this evidence directory; source artifact ID10564289143.
