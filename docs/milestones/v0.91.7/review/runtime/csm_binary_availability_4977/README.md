# CSM Binary Availability Proof (#4977)

This packet records the WP-07 proof that runtime work no longer depends on
incidental `target/` cache state for the standalone `csm` owner binary.

## Proof

- Guard: `bash adl/tools/ensure_csm_binary.sh --json`
- Owner lane: `bash adl/tools/run_owner_validation_lane.sh runtime`
  runs the CSM binary availability contract and guard before the runtime
  compatibility boundary.
- Owner-lane build mode: `bash adl/tools/run_owner_validation_lane.sh runtime
  --build` includes `--bin csm` and exports `ADL_CSM_BIN` to the repo-owned
  `adl/target/debug/csm` path.
- Strict missing-binary mode:
  `ADL_CSM_BINARY_STRICT_REQUEST=1 CARGO_TARGET_DIR=.tmp/csm-binary-availability-4977-target`
- Retained output: `restoration.json`
- Result: `status=restored`, `action=rebuilt`, `provenance=cargo_build`
- Warm-cache result: `status=ok`, `linked_files=8159`,
  `linked_fingerprint_files=3136`
- Build result: `cargo build --manifest-path adl/Cargo.toml --bin csm`
  completed successfully and produced the requested fresh-target `csm` binary.

## Boundary

The strict restoration proof uses a disposable `.tmp` target so it does not
move, hide, or disturb the live primary-checkout `csm` binary used by the main
runtime watch. The durable invariant is enforced by the runtime owner lane:
runtime validation now fails if the standalone CSM owner binary cannot be
resolved or restored from the current repo source. Runtime proof wrappers retain
their own `csm_binary_availability.json` packet before invoking `csm`.
