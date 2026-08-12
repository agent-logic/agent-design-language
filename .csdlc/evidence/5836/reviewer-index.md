# Issue #5836 first-birthday reviewer index

Implementation evidence revision: `7615b9089993799c25de28c45868078b99b7b9a0`.

| Surface | Command or artifact | Current evidence |
| --- | --- | --- |
| Runtime orchestration | `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --locked --test birthday_demo` | 4 tests passed locally after integration with #237. |
| Positive packet | `bash adl/tools/test_v092_first_birthday_demo.sh --positive` | `demos/v0.92/first-birthday/positive.json` (`sha256:99abf4a9d60347372e0f3b3666010bcfbd00a8f1c9722ab8e993908bc1f2a3cf`). |
| Negative matrix | `bash adl/tools/test_v092_first_birthday_demo.sh --negative` | 16 rejected packets plus one interrupted packet under `demos/v0.92/first-birthday/`. |
| Strict lint | `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --locked --test birthday_demo --bin adl-runtime-birthday-demo -- -D warnings` | Passed locally. |
| Native macOS | `bash adl/tools/test_v092_first_birthday_demo.sh --native-platform macos` | Passed; `.csdlc/evidence/5836/native-macos-receipt.json`. |
| Publication gate | `ruby .csdlc/evidence/5836/validate-publication-gate.rb --check-only` | Must remain blocked until current exact-head review and operator authorization exist. |

The native Linux receipt remains a separate AC-3 gate. This index does not
claim Linux proof, exact-head review, publication authorization, or public launch.
