# Native readiness placeholder gap

Observed during Sprint 8 preparation: native `issue` creates rendered cards with unresolved `<field>` values. Native `edit` accepts extra values such as `summary` and `plan_steps` that do not populate the SPP template fields `plan_summary` and `deliverables_inline`. Native `validate` and `doctor` reported `six_card_validation_passed` and a next execution route while those placeholders remained.

This is structural/digest validation evidence, not semantic design readiness. The affected preparation was corrected through editor skills and native field updates; workers must inspect rendered cards before implementation. No validator bypass or product repair is included in #934.

Safe reproduction: initialize an isolated native issue fixture, inspect SPP for `<plan_summary>` and `<step_1_status>`, run native validate/doctor, and compare the successful report with the unresolved rendered values. Preserve isolation and avoid new live issue creation merely to reproduce.

Suggested bounded follow-on: distinguish structural validity from design readiness; reject unresolved required pre-execution fields and unknown edit keys, with positive fully populated and negative placeholder/typo regression cases. Separate product issue admission is required before implementation. Historical preparation receipts remain evidence and are not rewritten.
