# Hosted CI retry receipt

- Initial exact publication head: `13bcc065cc444bacf7592e10c776e054a819b930`
- Initial workflow run: `33607832729`
- Unrelated failing case: `parity_c_provider_scheduler_tools::scheduler_dispatch_is_deterministic_and_bounded`
- Hosted failure: the test exceeded its existing four-second wall-clock assertion.
- Local focused reproduction: passed unchanged in 1.60 seconds.
- Issue #617 canonical-name tests were not the failing surface.

This metadata-only receipt triggers a fresh exact-head CI observation without
changing Runtime behavior or weakening the unrelated timing assertion.
