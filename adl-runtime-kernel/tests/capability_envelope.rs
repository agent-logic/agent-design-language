//! Public-boundary proof: raw capability construction is not exported.
//!
//! ```compile_fail
//! use adl_runtime_kernel::build_capability_envelope;
//! ```
//!
//! ```compile_fail
//! use adl_runtime_kernel::validate_capability_envelope;
//! ```

#[test]
fn verified_continuity_entrypoints_are_public() {
    let _ = adl_runtime_kernel::build_capability_envelope_with_continuity;
    let _ = adl_runtime_kernel::validate_capability_envelope_with_continuity;
    let _ = adl_runtime_kernel::build_governed_cognitive_profile_with_continuity;
    let _ = adl_runtime_kernel::validate_governed_cognitive_profile_with_continuity;
}
