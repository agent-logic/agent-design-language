# PR 1063 runtime coverage repair

At head 0686e60a0bf29b595f266c476b2ae0996cf6fdd7, runtime coverage job 105192059737 in run 35218300153 passed 692 of 693 tests. The existing `file_events_are_debounced` failed its parser-count assertion (3 rather than 2). Its real 5 ms sleeps did not constrain scheduler delay relative to the 40 ms debounce window.

The test now controls Tokio time and waits for each file candidate to be observed. Three candidates arrive 30 ms apart, requiring deadline reset across a burst longer than one debounce interval. Before expiry, only the initial snapshot is parsed; after expiry, only the final candidate is applied. Complete synchronous writes on the test's current-thread runtime avoid unrelated partial-write observations. Production code and debounce policy are unchanged.

PVF: existing runtime integration test; deterministic virtual-clock regression proof with real local filesystem I/O; bounded local CPU/filesystem resources; no network/provider effects. Retains runtime coverage CI gate membership. This is test reliability evidence, not hosted CodeFriend acceptance.

Validation: all eight config_reload integration tests passed; the repaired debounce test passed 100 executions with up to 16 processes concurrently. Independent review by review_1056_server found no actionable issues in the two-file repair. Native proof and CI outcomes are recorded separately; no claim of CI success is made here.
