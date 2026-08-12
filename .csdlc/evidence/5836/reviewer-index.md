# Issue #5836 first-birthday reviewer index

Implementation evidence revision: `18fa6a7dcf60b1e81d6d90b0855deb06d82ee78a`.

| Surface | Command or artifact | Current evidence |
| --- | --- | --- |
| Runtime orchestration | `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --locked --test birthday_demo` | 4 tests passed locally after integration with #237. |
| Positive packet | `bash adl/tools/test_v092_first_birthday_demo.sh --positive` | `demos/v0.92/first-birthday/positive.json` (`sha256:5aa560af783b1673a1b18a92a259672f9a15319b666ffbaf0531956c309ecc10`). |
| Negative matrix | `bash adl/tools/test_v092_first_birthday_demo.sh --negative` | 16 rejected packets plus one interrupted packet under `demos/v0.92/first-birthday/`. |
| Strict lint | `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --locked --test birthday_demo --bin adl-runtime-birthday-demo -- -D warnings` | Passed locally. |
| Native macOS | `bash adl/tools/test_v092_first_birthday_demo.sh --native-platform macos` | Passed; `.csdlc/evidence/5836/native-macos-receipt.json`. |
| Publication gate | `ruby .csdlc/evidence/5836/validate-publication-gate.rb --check-only` | Must remain blocked until current exact-head review and operator authorization exist. |

The native Linux receipt remains a separate AC-3 gate. This index does not
claim Linux proof, exact-head review, publication authorization, or public launch.
