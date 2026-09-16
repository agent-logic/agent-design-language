# #896 Installed Markdown Renderer Snapshot Remediation Proof

- Installed command: `adl codefriend export markdown`
- Input review revision: `410da89a0ed42c523143da89fffeb7f6402833e0`
- Input scope: pinned ten-file Vector review scope
- Current decision: explicitly approved before renderer execution
- Renderer: `v1`
- Findings rendered: `1`
- Report digest: `800ef2bba83879093386f2727a1d23aa5c5ea05c55ccc4c3aa594da322c9cf11`
- Manifest digest: `cfc4d4c6e77840e407035ae9003c8b19ae5e6c4e83ffef63f04c644db30804bb`
- Approval decision digest: `61aa42483ff0940636848c21ca1e5ff7df554f794b484964159efbd9338d01fc`

The current installed binary retained every approved artifact byte after one
verified snapshot and parsed the synthesis, remediation, and test-plan bundles
only from those retained bytes. It canonically revalidated each bundle before
rendering, then generated `report.md` and `manifest.json` into a fresh local
target through directory-handle-anchored writes and a platform no-replace
rename. It reopened both committed outputs through the anchored directory
handle and verified their exact bytes and digests before success.

Focused remediation proof:

```text
cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md
5 passed; 0 failed

cargo test --manifest-path adl/Cargo.toml --lib codefriend::publication::markdown::tests
4 passed; 0 failed

cargo clippy --manifest-path adl/Cargo.toml --lib --test codefriend_render_md -- -D warnings
passed

cargo fmt --manifest-path adl/Cargo.toml --check
passed

git diff --check
passed
```

The snapshot regression deterministically replaces an approved source path
after snapshot admission and proves downstream parsing still consumes the
approved retained bytes. Prior citation, competing-target, and parent-swap
regressions remain passing. No provider, network, browser, HTML/PDF, remote
publication, or paid resource was used or claimed.
