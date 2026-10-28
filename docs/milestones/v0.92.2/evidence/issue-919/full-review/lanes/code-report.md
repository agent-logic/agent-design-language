# CODE lane — candidate 5c4a6149771c637f3c805985b86231077965eab4

## Findings

**CODE-001 — P2: Preserve source whitespace in exact citation exports.** `adl/src/codefriend/publication/markdown.rs:978` and `publication/html.rs:613` replace newlines, carriage returns and tabs with spaces in strings displayed as “Exact source excerpt”. Verified citation byte ranges do not prevent the resulting exported excerpt from changing indentation-sensitive code or multiline string contents. PDF inherits the same Markdown text and further normalizes whitespace. Render source excerpts through a dedicated literal code representation. Acceptance: CF-RENDER-MD-A03/A04, CF-RENDER-HTML-A02/A04, CF-RENDER-PDF-A02/A04. Owner: #921 renderer surface.

**CODE-002 — P2: Reject retry while the original shell attempt is still running.** `adl/src/codefriend/operator/mod.rs:235` considers an Incomplete attempt settled. Direct retry from a second terminal can advance active_attempt while the first provider execution still holds the evidence Store lock. The second attempt then fails store_busy, subsequent cancel targets the second attempt, and the first attempt's completion does not update the shell because active_attempt has changed. Require terminal settlement and serialize state checks before advancing the active attempt. Acceptance: CF-SHELL-A02/A04/A07. Owner: #921 operator shell.


Detailed triggers, paths, acceptance IDs, repro limitations and remediation are in `code-findings.json`.

## Evidence and limitations

Two bounded standalone Rust helper harnesses executed successfully: `code-excerpt-repro.rs` demonstrates exact production escape helpers flatten source whitespace; `code-retry-repro.rs` demonstrates the exact settlement predicate admits Incomplete without a retained settlement receipt. Output is retained in matching `.log` files. The whitespace probe stubs the unrelated privacy predicate false for a benign fixture; the retry probe uses lightweight data types around the original predicate. These are helper-level reproductions. Full approved renderer and concurrent CLI end-to-end reproduction were not executed. The retry downstream effect is established by source tracing through store ownership, run completion, and cancellation routing.

This is an independent source review subagent, not an implementation author. No product source, runtime state, provider services, GitHub state or cloud state was changed. No paid or network tests were performed. Prior product reviews and CI success were not treated as substitutes for source inspection.

## Coverage

All **77 changed CodeFriend production source paths** have now been inspected. `code-coverage.json` enumerates all 230 changed Rust src paths, with reviewed ranges, observations and explicitly unresolved complementary coverage. Embedded tests were inspected selectively; production coverage is not an assertion that every test body ran or was reviewed. Additional completed support inspection includes two runtime modules. All 12 changed CodeFriend CLI and binary entrypoint paths (2652 lines) were additionally reviewed in full, including embedded tests; no additional finding. C-SDLC is recorded separately in architecture-coverage.json. Other unresolved paths are enumerated in `code-complementary-followup-paths.txt`; no pass is implied for them. The CodeFriend followup path list is empty.

Completed surfaces include all ingestion adapters, parsing/import resolution, language governance, evidence admission/storage/contracts, four-perspective execution and synthesis, both architecture graph generations, 4+1 generation/rendering, baseline/Palace memory, activity proposals, shell lifecycle, paired agent transport/journaling/publication, journey attachments, server control/drain/publication and Markdown/HTML/PDF publication/approval/relay. An additional drain-gate/render-admission contract concern was referred to the lead for cross-lane assessment and is not counted as a confirmed finding here.

Test context inspected (not executed): shell cancel/retry, PDF semantic comparison and Unicode/wrapping, import/resolver grammar, Rust parser resource admission, unique JSON keys, Markdown anchored exports, agent journal locking, journey delivery acknowledgment/no-replay, owned baseline reference liveness, unknown-effect precedence, manifest growth and resident health. Existing retry tests cancel first and PDF comparisons normalize whitespace, so neither proves absence of the corresponding finding.

This completed assigned CodeFriend slice does not establish milestone acceptance or a complete repository-wide CODE PASS: complementary C-SDLC, provider/runtime and website reviews retain their own coverage and findings. No full build or broad test suite was run in this lane.


CODE-003 withdrawn after cross-lane challenge: documented #1102 policy deliberately removes total request/capture deadlines. No distinct required inactivity/cancellation contract was established. This is not an actionable defect.
