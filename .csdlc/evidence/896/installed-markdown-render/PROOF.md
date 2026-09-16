# #896 Installed Markdown Renderer Proof

- Installed command: `adl codefriend export markdown`
- Input review revision: `410da89a0ed42c523143da89fffeb7f6402833e0`
- Input scope: pinned ten-file Vector review scope
- Current decision: explicitly approved before renderer execution
- Renderer: `v1`
- Findings rendered: `1`
- Report digest: `38abef828a33b37f5b89ed257953e824fcbde46d3ff72377933339f6e96a7b15`
- Manifest digest: `f5cd4e16c6d332574fc4c1952b541afebe4c8dd36e78a76a97292d56bcce4d32`
- Approval decision digest: `7eed74c63b82748b3f70cde1d7e65f38a6a65a2c1c9f1d52f1d9fa90805f7c26`

The installed binary generated `report.md` and `manifest.json` into a new
local target, then the renderer re-opened both artifacts and verified their
bytes and digests before returning `render-result.json`. The retained report
was opened and inspected for source scope, revision, finding identity,
non-clickable citation, attribution, severity, uncertainty, remediation plan,
test plan, approval identity, claims, and nonclaims.

Focused proof:

```text
cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md
5 passed; 0 failed
```

Adjacent compatibility proof:

```text
codefriend_synthesis: 5 passed
codefriend_remediate: 9 passed
codefriend_testplan: 8 passed
codefriend_ux: 7 passed
```

Strict Clippy for `codefriend_render_md`, formatting, JSON parsing, and exact
diff hygiene passed. No provider, network, browser, HTML/PDF, remote
publication, or paid resource was used or claimed.
