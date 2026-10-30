# New template sample baseline — issue 121

Daniel selected #130's new templates; the existing document templates will not be used. #130 remains read-only. Measurements and probes use a copied input snapshot, inventory SHA-256 `141056c10179b9ef50d4542ca1c52b7ae9532f5549fe7e4d5d403366fbe88f6c`. These six non-Journal examples are work in progress; Journal remains #129. This is sample-pipeline evidence, not final catalog, accessibility, service or launch acceptance.

## Measurements

Chrome 154.0.8037.98, Python Playwright 1.55.0, sandbox enabled, outbound networking blocked. One browser launch took 524.867 ms. Each document uses a fresh context. OS caches were not flushed; these are local observations, not a service SLO. Each PDF measured context creation, render, write and close; script-level stages and exact binaries/fonts/assets are retained in `.csdlc/evidence/121/new130/measurements.json`.

| Sample | Three PDF observations (ms) | Pages |
| --- | --- | --- |
| Product report | 306.661, 205.455, 197.913 | 4 |
| Security | 206.290, 199.495, 200.422 | 4 |
| Documentation | 182.537, 194.787, 193.492 | 3 |
| ADR | 186.660, 190.578, 192.035 | 3 |
| Diagram | 441.086, 449.144, 431.506 | 7 |
| Test plan | 182.821, 183.593, 183.895 | 3 |

All 18 PDF renders and six HTML builds completed. Markdown is authored sample material; no generator timing is claimed. PDF metadata reports Letter and tagged output; tagging alone does not establish accessibility. Original sample overview found no obvious clipping or overlap. F7 on report p3, T07/T08 on testplan p2, and sparse security p4 are pagination polish observations, not confirmed functional defects. Repeated PDF bytes sometimes differ. For five of six three-render groups, removing only CreationDate/ModDate values in memory makes each group byte-identical (`pdf-repeat-comparison.json`); Documentation still has two normalized byte variants; those become identical after also normalizing tagged-PDF node identifiers. This is a diagnostic byte comparison, not proof of equivalent accessibility structure. The original PDFs remain untouched. Cross-run and cross-machine determinism remain unqualified.

## Reproduced findings for the #130 owner

No #130 files were modified. Reproduction harness and results are retained under `.csdlc/evidence/121/new130/`; paths in the harness identify the isolated snapshot and local probe output.

1. **Mermaid finishing isolation gap.** `mermaid-theme/finish.py` inserts SVG through `page.set_content` without the PDF pipeline's CSP or request allowlist. The probe exercises the same insertion primitive, not the complete finishing command or a Mermaid-source-to-SVG exploit. A benign inline SVG script set `window.cfProbeExecuted=true`; an SVG external image attempted a request to reserved `https://probe.invalid/image.png`. The test harness aborted that request and also used a dead proxy. This proves script execution and attempted outbound loading, not successful egress. Expected for untrusted diagram content: prohibit page scripts and undeclared resources, with process/context bounds and cleanup. The PDF pipeline blocked the same inline script. Trusted static sample inputs limit immediate exposure; production integration must establish the trust boundary.

2. **Long unbroken text clips in an existing summary slot.** In the copied report HTML, replace the first summary sentence with `LONGTOKEN_` repeated 300 times plus `VARIABLE_CONTENT_END`. Also add 100 numbered rows to the existing first table. The probe selects normal spacing (`compact=False`), while the measured Main sample uses compact spacing; compact-mode reproduction is not claimed. The generated 17-page PDF retains all 100 row identifiers and the end marker, but only 10 of 300 token repetitions appear in extracted PDF text. Visual inspection of page 1 confirms horizontal clipping at the right content edge. Expected: preserve and wrap variable text, or reject it explicitly before rendering. Repro artifact: `/private/tmp/worker30-new130-slot-probes/variable-report.pdf`, page 1; screenshot beside it. This is a synthetic existing-slot stress case, not a production field-binding acceptance test. Table headers repeat; complete visual inspection of all 17 stress pages remains outstanding.

## Remaining acceptance

Retain completed measurements while final #130 designs continue. Verify final field binding, long paths/URLs/code and wide tables, navigation/accessibility, input/process limits, cleanup on failures, and real service invocation overhead against the final selected templates. Broader catalog acceptance and launch validation remain downstream. No old renderer repair or qualification follows from this record.
