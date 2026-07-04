# Runtime Soak 2 Attempt Status (#4682)

This packet records the current #4682 Soak 2 execution attempt against
v0.91.7 evidence. It does not claim that Runtime Soak 2 completed.

The current result is `blocked_before_full_soak`: the #4843 matrix PR #4870,
the #4784 resilience failure-injection PR #4871, and the #4683 diet-map PR
#4872 are ready and green, but they are not merged to `main`; the canonical
runtime path PR #4868 is ready at `0b8821ec1de112d3b84fa64e87d2a6fb9fb63a02`
with `adl-ci` and `adl-coverage` green but is not merged to `main`; and the
#4783 scheduler/watcher/AEE resilience middleware PR #4869 is ready at
`d0e17ba2e06689d38d32cfb09704e541257665fb` after a janitor retrigger with
`adl-ci` green and `adl-coverage` still in progress.

The status artifact is:

- `soak2_execution_status_4682.json`

The blocker register is:

- `blocker_register.json`

Validate this packet with:

```bash
bash adl/tools/validate_v0917_soak2_4682_status.sh
```

## Non-Claims

- This packet does not claim v0.92 runtime coherence.
- This packet does not claim the full Soak 2 feature-list matrix has run.
- This packet does not replace the #4843 matrix, the #4683 diet map, or the #4783/#4681/#4784
  implementation PRs.
