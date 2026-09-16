# #896 Installed Markdown Renderer Remediation Proof

- Installed command: `adl codefriend export markdown`
- Input review revision: `410da89a0ed42c523143da89fffeb7f6402833e0`
- Input scope: pinned ten-file Vector review scope
- Current decision: explicitly approved before renderer execution
- Renderer: `v1`
- Findings rendered: `1`
- Report digest: `b9af969342e2108d3d827e69c68e73a5b3f07d6fca9d8c862c3efc3b5541da88`
- Manifest digest: `13c3ab6a7c2aa7dafa66698d0089eb4a74ad2bad9ca0038da56ed60a6e2cbc02`
- Approval decision digest: `61eee0193e185d5022d2e613c00cd6943002a16877ac9a365119a57e39a5c4b1`

The installed binary generated `report.md` and `manifest.json` into a fresh
local target using directory-handle-anchored writes and a platform no-replace
rename. It reopened both committed artifacts through the anchored directory
handle and verified their exact bytes and digests before returning the retained
result. The report includes every governed synthesis-source, remediation-action,
and test-case identity and evidence field.

Focused remediation proof:

```text
cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md
5 passed; 0 failed

cargo test --manifest-path adl/Cargo.toml --lib codefriend::publication::markdown::tests
3 passed; 0 failed
```

The unit regressions prove variable-length code spans do not activate links or
HTML from backtick-bearing paths, a competing target cannot be replaced, and a
parent-path swap cannot redirect writes outside the already opened directory.
No provider, network, browser, HTML/PDF, remote publication, or paid resource
was used or claimed.
