# Ceremony repair verification

Inspected candidate: `2674f00c1417dacb437a1f813a134291f6e9c14d`.
Repair: #856 / PR #858, merged as `c58001391924622b5fbb0417441bb45a03f0cb59`.

This addendum preserves all original findings and prior status records unchanged.

The repair is integrated into #526. `RELEASE_ARTIFACTS.json` declares coordinated release packages and independent/historical exceptions. The shell wrapper routes exclusively to the installed native v3 preflight; its former v2 fallback is removed.

Executed here:

- `bash adl/tools/test_release_ceremony.sh`: PASS routing contract.
- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test release_preflight`: PASS, one matrix test, zero failures, 40.42 seconds. This covers positive fixture consistency and negative identity/version/gate cases, not real release approval.
- Installed reviewed native owner through `install_owner_binaries.sh --bin csdlc`.
- Exact-candidate preflight in a clean shared validation clone: rejected with `native_v3_gate_missing`, as expected. A preliminary attempt in the issue worktree rejected `candidate_checkout_dirty` because operational receipts are untracked; those files were preserved. Request/results are retained in resolved Git metadata under `csdlc-v3/issue526-preflight/`.
- Canonical release projection check: PASS structural check, 393 rows, 143 approved removals, 14 negative cases; `release_decision` remains `blocked` at its retained candidate `64a99fd71b9770e15cb0dc393d669450d3a5f5b4`.

Remaining release prerequisites: reviewed final #522 finding disposition and final quality proof; a current release projection; an explicit gate binding the final candidate, notes, inventory and reviewed #522/#525/#526 evidence; and operator authorization for release operations. #525 is now CLOSED. Issue closure alone does not establish final-candidate semantic acceptance.

No actual candidate-ready gate was fabricated. No tag or release was created. No #833 source or review record was edited.
