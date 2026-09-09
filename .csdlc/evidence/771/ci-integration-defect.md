# PR #801 CI integration repair

Run 34387376726 tested merge 2d57b17cf of head 8599ff8f7409204f1b4bfff2025bd66f5625edf8 into main 693189d1e99301467b862a53f3ea808edfa5178d. Two new terminal receipt regressions failed before receipt preflight because main moved primary-repository terminal outputs into `.git/csdlc-v3/local`, while the new fixtures requested `.csdlc`. This was a base integration mismatch, not a demonstrated Linux or temporary-directory defect.

Integrated current main b631e8ddf3bc924cd46aed48be8aa65f232dd508. The same focused command reproduced 3 passed / 2 failed locally before fixing both new tests' request and readback paths. After correction all 5 focused receipt tests passed; the terminal conflict-before-state-write repair is preserved. Added assertion findings for future diagnosis.

PVF: deterministic offline local Git fixtures, small CPU/disk, required terminal receipt regression proof. Logs redact machine-local roots; no credentials or live write transport are involved. Full exact-source validation and independent review must be refreshed before publication. Prior 7fecd63398 review/suite/mapping are archived unchanged.
