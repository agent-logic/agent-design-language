# Current V3-F proof — issue843

Source: `a32cc903e12375690613b8a4bf003e985390a221`.

Independent reviewer `codex:review_835` reviewed all11 changed or added files against the archived baseline;153 files are unchanged, yielding164 current scope files. All four source-review criteria pass. The legacy-ready recovery finding is fixed. Assignment, review and mapping records bind the complete blob manifest.

The separate locked C-SDLC suite passed225 tests from a clean detached checkout with an external build directory. `suite.json` binds the exact source, command, checkout state and redacted log digest. This is component execution proof, not live GitHub mutation, a new cloud canary or release authorization.

The previous accepted packet is retained under `archive/c0a9edef5b/`; older packets and component reviews remain historical evidence. `historical-fixture.diff` is retained historical repair evidence, not the current fixture identity. Each current mapping row binds the fixture blob at the source above. Historical census and exception bytes remain unchanged.

Run `python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/validate.py` and repeat with `--negative`. The validator requires exact source/scope, independent semantic review, matching suite bytes, clean execution and immutable historical records. Evidence-only descendants are allowed; changes under the coupled source prefixes invalidate the proof.

PVF: required deterministic local mapping checks and detached component execution.15 substitution cases reject stale source, partial scope, missing review, altered suite and historical evidence. No release ceremony or approval is inferred.
