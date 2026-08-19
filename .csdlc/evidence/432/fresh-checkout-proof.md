# Fresh-checkout proof

- Candidate: `79c8a749d0811cbfb190710dc07ed0974c9dc0e5`
- Fresh clone: `/Volumes/FastWork/adl-432-final.zfB9Sa/repo`
- Clone source: local repository using `git clone --no-local`

Passing commands:

1. `bash adl/tools/test_check_no_tracked_adl.sh`
   - proved a zero tracked `.adl` denominator
   - proved canonical policy presence
   - proved tracked-path and reconstructed legacy-authority negative cases
2. `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2 actual_binaries_create_validate_doctor_and_bind_without_claims -- --exact`
   - `1 passed; 0 failed`
   - exercised the real create, validate, doctor, and `csdlc-bind` binaries from the fresh checkout
3. `test ! -e .adl`
   - confirmed the fresh clone materialized no `.adl` path

The fresh clone is retained; no cleanup deleted it.
