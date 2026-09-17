# Installed HTML renderer proof

Issue: #897  
Product revision: `f03425f527bfea1d76ee66a57e169ae893439f24`

## Commands and result

- `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_html` — 4 passed.
- `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_md` — 5 passed.
- `cargo clippy --manifest-path adl/Cargo.toml --lib --bin adl -- -D warnings` — passed.
- `cargo fmt --manifest-path adl/Cargo.toml -- --check` — passed.
- Installed CLI retention scenario with `CODEFRIEND_HTML_RETAIN_DIR` — 1 passed and retained `report.html`, `manifest.json`, and `render-result.json` in this directory.

## Browser observation

The retained `report.html` was served from an ephemeral loopback-only HTTP server and opened in a browser. The page rendered successfully with the `CodeFriend Review Report` heading, contents navigation, finding index, source identity, revision, and scope digest visible. The server returned HTTP 200 for the report. The proof server was stopped immediately after inspection.

The output is static HTML: it contains no scripts, forms, or remote resources. This observation proves local readability and navigation of the retained artifact; it does not claim network publication or external hosting.
