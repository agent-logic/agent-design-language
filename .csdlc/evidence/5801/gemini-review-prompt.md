# Gemini 3.1 Pro Review Prompt

Review the supplied exact ADL revision for issue #5801, WP-02A CI and coverage
reliability. Treat the repository diff and `ci-topology.md` as the bounded
review surface.

Prioritize correctness defects that could:

- classify docs, lifecycle metadata, workflow/tooling, ordinary source,
  runtime-critical source, unknown, or mixed changes incorrectly;
- allow a cancelled, failed, or missing child result to satisfy a stable
  required-check aggregator;
- duplicate coverage authority or weaken exact-head source proof;
- change required-check names, coverage thresholds, AWS, runner sizing, or
  migration settings outside scope.

Return strict JSON with:

- `summary`: concise assessment;
- `findings`: array of objects containing `id`, `severity`, `actionable`,
  `path`, and `summary`;
- `recommendation`: `pass` or `changes_required`.

Do not recommend broad refactors. A finding is actionable only when it names a
specific defect in the supplied revision.
