# Issue 5780 deletion metrics

Baseline: `a5a16509af05af441369e78eeacfd74017bcf63b`.

| Surface | Before | After | Net deleted | Reduction |
| --- | ---: | ---: | ---: | ---: |
| Production Rust | 28,722 | 18,516 | 10,206 | 35.53% |
| Test Rust | 15,092 | 9,247 | 5,845 | 38.73% |
| Combined Rust | 43,814 | 27,763 | 16,051 | 36.64% |
| Source binaries | 23 | 21 | 2 | 8.70% |

Across every changed tracked file, including lifecycle cards and proof logs,
the implementation adds 4,379 lines and deletes 18,174, for a net deletion of
13,795 lines.

The removed surface includes the `csdlc-closeout` binary and skill, terminal
repair and reconciliation request schemas, readiness and merged-publication
writers, legacy receipt writers, and their writer-specific tests. The retained
compatibility boundary is read-only: all 114 tracked v0.91.8 terminal records
pass the census, historical terminal phases and receipts remain deserializable,
and the 314 existing Git-common receipt files are left immutable. New terminal
authority comes only from the minimal derived envelope written by
`csdlc-finish`.
