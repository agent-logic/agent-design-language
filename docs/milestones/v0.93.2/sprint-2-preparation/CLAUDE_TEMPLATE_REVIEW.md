# CodeFriend new-template review: handoff for Claude

Date: 2026-10-20. Prepared for Claude, via Daniel, from Sprint 2 / CodeFriend #121 review of the six unfinished #130 examples.

**Two reproducible issues need attention: SVG browser isolation and clipping of long unbroken text.** Three pagination observations are polish candidates. Other items below are validation gaps, not demonstrated defects. Only the new #130 templates are selected; the old document templates are excluded.

No files in #130 were changed. This packet records observations and recommendations; it does not authorize or claim changes to the designs.

## 1. SVG finishing permits script execution at its browser insertion boundary

**Priority:** High before accepting untrusted diagram content. Production exploitability is not yet established.

**Affected source:** `mermaid-theme/finish.py`, especially the SVG interpolation into `page.set_content` at line 47 of the inspected snapshot. The source launches Chromium with explicit sandbox configuration, but this insertion path lacks the PDF pipeline's CSP and resource allowlist. Chromium process sandboxing does not disable page scripts or resource loading.

**Observed:** A benign inline SVG script set `window.cfProbeExecuted=true`. An SVG image reference attempted a request to `https://probe.invalid/image.png`. The test harness intercepted and aborted that request and used a dead proxy. No successful external connection or data exfiltration was demonstrated. The PDF pipeline blocked the same inline script.

**Reproduction:** The retained [probe](evidence/claude-template-review/probe.py) inserts an SVG containing the marker script and external image through the same `page.set_content` primitive used in `finish.py`. See [results](evidence/claude-template-review/probe-results.json), `mermaid_set_content` and `pdf_isolation`. This is a focused insertion-boundary probe, **not** execution of the complete finishing command and **not** a demonstrated malicious Mermaid-source-to-SVG exploit.

**Recommended correction:** Establish an explicit untrusted-SVG boundary: disable page script execution, deny undeclared resources, and use bounded contexts/processes with cleanup. Sanitization may complement those controls, but must not rely on the browser sandbox alone. Reuse the PDF path's restrictive approach where appropriate, without weakening it.

**Validation after correction:** Run a regression through the actual finishing command using hostile SVG fixtures. Inline scripts and event handlers must not execute; undeclared network/file loads must be refused; expected local diagram resources must still render. Check timeout and failed-render cleanup. Trace whether customer or model-generated diagram content can reach this path in production before assigning a production security severity.

## 2. Long unbroken text is clipped in the report summary

**Priority:** Correctness fix before accepting variable customer content.

**Affected source:** `canvas-source/Main.dc.html`, the existing Summary paragraph, rendered with normal spacing (`compact=False`). The measured baseline Main example uses compact spacing; **compact-mode reproduction is not claimed**.

**Reproduction:** In an isolated copy, replace the first summary sentence with `LONGTOKEN_` repeated 300 times followed by `VARIABLE_CONTENT_END`. Append 100 numbered rows to the existing first table. The retained probe performs those substitutions and calls the copied PDF pipeline. It produced a 17-page PDF.

**Observed:** The summary runs past the right content edge instead of wrapping. Extracted text contains only 10 of the 300 token repetitions; the end marker remains. All 100 table row identifiers remain in the PDF. The visible clipping is independently confirmed on page 1.

- [Rendered stress PDF](evidence/claude-template-review/variable-report.pdf)
- [Page 1 screenshot](evidence/claude-template-review/clipped-summary-page1.png)
- [Probe and exact substitutions](evidence/claude-template-review/probe.py)

**Recommended correction:** Ensure long variable text wraps within its content box, or reject unsupported input explicitly before rendering. Evaluate wrapping rules for prose, long paths, URLs, identifiers and code separately so the fix does not damage normal formatting or meaningful code layout.

**Validation after correction:** Repeat in normal and compact modes. Confirm the full token sequence survives without horizontal clipping, overlapping adjacent content or losing pagination. Exercise realistic long paths/URLs as well as the synthetic token. Verify actual production field binding separately: this probe substitutes text in the copied sample HTML and does not prove production field handling.

## 3. Pagination polish observations

These occur in the six standard generated examples. They are not confirmed data-loss defects or automatic launch blockers.

| Sample | Observation | Suggested design check |
| --- | --- | --- |
| Product report, `CF-RPT-0142`, page 3 | The F7 row continues alone at the top after the findings table starts on page 2. | Consider avoiding a one-row table continuation while retaining repeated headers and reasonable page density. |
| Test plan, `CF-TPL-0027`, page 2 | T07 and T08 continue from the preceding page's test matrix. | Check whether this split is intentional; test minimum continuation sizes with variable row heights. |
| Security review, `CF-SEC-0031`, page 4 | The final coverage-gaps section leaves most of the page empty. | Review keep-together and closing-section rules for balanced pagination without forcing overflow. |

The standard six-example, 24-page overview showed no obvious clipping or overlap. That observation does not contradict the variable-content failure above. The stress PDF retained every added row and repeated table headers, but all 17 stress pages have not received a complete visual review.

## 4. Remaining validation gaps

These are open acceptance questions, not additional reproduced bugs:

- **Repeated PDF bytes differ in some runs.** Follow-up at CodeFriend #121 commit `eed35b42a63a7ee611e35f75afe86bc7eeae4b5a`: in-memory removal of CreationDate/ModDate values makes five of six three-render groups byte-identical. Documentation retains two variants; the execution owner reports these become identical after also normalizing tagged-PDF node identifiers. See [byte comparison](evidence/claude-template-review/pdf-repeat-comparison.json) and retained source baseline. Original PDFs were not modified. This is diagnostic normalization, not accessibility-equivalence proof; cross-run and cross-machine determinism remain unqualified.
- **Final field binding and content range:** Validate real generated content, empty/optional sections, long paths/URLs/code, wide tables, large findings sets, Unicode and supported fonts. The six fixtures cannot establish all 25 document types and their declared formats.
- **Accessibility and navigation:** All six sampled PDFs report tagged Letter output. Tag presence does not prove correct reading order, headings, table semantics, image descriptions, links or bookmarks. Validate those against the final templates.
- **HTML:** Six builds completed; browser visual and interaction acceptance is not established by build success.
- **Markdown:** Samples are authored sources. No Markdown generator timing or production generation acceptance is established.
- **Other formats:** Record and test each declared output format, including diagram source and image exports where applicable. Do not equate 25 types with 25 PDFs. The planned Journal type remains separate from these six non-Journal examples.
- **Production boundary:** Validate the actual service invocation, input/process limits, concurrency, timeout behavior, cancellation and cleanup, resource isolation and failure reporting. Local sample timing is not end-to-end service latency.
- **Final templates:** Repeat affected checks against the delivered template revisions. Sprint 2 may retain this evidence and route remaining #121/final-template work; final-template readiness is still required for the applicable launch acceptance.

## 5. Performance evidence

Chrome 154.0.8037.98; Python Playwright 1.55.0; explicit Chromium sandbox enabled. One reused browser, fresh context for each document; OS caches not flushed. Browser startup was **524.867 ms**, measured separately. Each PDF duration includes context creation, rendering, writing and closing.

| Sample | Median of three PDF runs | Pages |
| --- | ---: | ---: |
| Product report | 205.455 ms | 4 |
| Security review | 200.422 ms | 4 |
| Repository documentation | 193.492 ms | 3 |
| ADR | 190.578 ms | 3 |
| Diagram packet | 441.086 ms | 7 |
| Test plan | 183.593 ms | 3 |

All 18 PDF runs completed, as did six HTML builds. These measurements do not establish a service SLO, cold-machine performance, production throughput or final-template acceptance. Full run and input identities are retained in [measurements](evidence/new130-sample-measurements.json).

## 6. Earlier concerns that should not be reopened as current defects

- Current PDF source renders once by default. A second spacing render is optional under `--explore`; double rendering is not the default benchmark cost.
- Current PDF and Mermaid finishing source explicitly configure Chromium sandboxing. The SVG finding concerns page-level execution/resource isolation, not absence of process sandboxing.
- The old renderer's Greek Delta/font failure and untagged PDF observations belong to superseded diagnostic work. They are not findings against the selected #130 templates.

## Source evidence and reproduction custody

Input inventory SHA-256: `141056c10179b9ef50d4542ca1c52b7ae9532f5549fe7e4d5d403366fbe88f6c`.

- [Retained source baseline](evidence/claude-template-review/source-baseline.md) includes the execution owner's latest qualifications.
- [Full measurements and per-input hashes](evidence/new130-sample-measurements.json).
- [Structural and visual overview](evidence/new130-sample-rendering-observation.json).
- [Exact probe](evidence/claude-template-review/probe.py), [probe results](evidence/claude-template-review/probe-results.json), and stress output linked above.

The probe is retained as evidence, not a portable command: it references the isolated snapshot at `/private/tmp/worker30-new130-baseline/source-snapshot`, local Chrome and a scratch output directory. To rerun, copy the selected source into a fresh scratch directory, verify its inventory, and adapt those paths in the copied harness. Preserve request interception/dead-proxy controls. Never run the probe with #130 as its output directory. Changed source requires a new recorded inventory; current findings apply to the inspected snapshot.

## Documentation scope, claim ledger and validation

Mode: handoff packet. Audience: Claude via Daniel. Edit scope: this handoff and retained evidence in the Sprint 2 worktree. No design files changed.

Source-backed facts are labeled as observations above; corrections are recommendations; production exploitability, determinism, accessibility and final acceptance are explicitly unproven. No fixes in this packet are claimed implemented. Local paths are included intentionally for this private worktree handoff, not public documentation.

Validation performed: source/probe/result inspection, page-1 screenshot inspection, retained evidence existence and Markdown link checks. No product tests were rerun merely to write this document. Full stress-document visual review and production validation remain outstanding.

Publication attempted: false. Release approval claimed: false. Review approval claimed: false. ADR accepted: false. Broad rewrite performed: false.
