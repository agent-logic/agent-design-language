# Installed original transpiler scaffold

Build `demos/transpiler_demo` with its locked dependencies, then package the executable:

```
python3 tools/transpiler_distribution/package.py --binary demos/transpiler_demo/target/debug/transpiler_demo --output transpiler-demo.tar
```

The archive contains the executable, exact original YAML and Rust skeleton, repository license and member hash manifest. Data and license bytes are read from the recorded Git commit, regardless of working-tree edits. The supplied executable is hash-bound; its source/build provenance requires the corresponding CI build receipt. Authenticate the archive and source revision before extraction. The executable is platform-specific; use the matching CI job artifact. It is a bounded mapping scaffold, not a code generator or adaptive repair engine.

Invoke the installed executable with `--input-root <absolute installed directory> --output-root <absolute fresh output directory>`. Both are explicit; inputs and roots reject symlink traversal. The output parent must already exist; the output root must not exist. The original no-argument source mode remains unchanged. The verification JSON retains its historical schema, relative artifact labels and command field; actual installed argv/platform belong in the delivery receipt.

Tests: deterministic offline product tests; release gate for this distribution only. No network/provider/runtime lifecycle, credentials or shared binary activation. Positive original scenario and negative missing/mismatched inputs, symlink confinement and existing output cover installed behavior.
