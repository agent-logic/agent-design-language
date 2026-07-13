use std::collections::BTreeSet;

use csdlc_v2::{select_generation, Generation, GenerationSelector, ProofManifest, ProofStep};

fn manifest() -> ProofManifest {
    serde_json::from_slice(include_bytes!("../operator/pre-switch-proof.json")).unwrap()
}

#[test]
fn manifest_requires_v1_default_opt_in_and_exact_proofs() {
    let mut value = manifest();
    assert!(value.validate().is_ok());
    value.default_generation = Generation::V2;
    assert!(value.validate().is_err());
    value.default_generation = Generation::V1;
    value.steps.pop();
    assert!(value.validate().is_err());
}

#[test]
fn relabeled_arbitrary_commands_are_rejected() {
    let mut value = manifest();
    value.steps[0] = ProofStep {
        id: "full_suite".into(),
        executable: "sh".into(),
        args: vec!["-c".into(), "true".into()],
    };
    assert!(value.validate().is_err());
}

#[test]
fn explicit_v2_rehearsal_rolls_back_to_unchanged_v1_default() {
    let selector = GenerationSelector {
        schema: "csdlc.generation_selector.v1".into(),
        default_generation: Generation::V1,
        opted_in_issues: BTreeSet::from([5293]),
    };
    assert_eq!(
        select_generation(&selector, 5293, None).unwrap(),
        Generation::V1
    );
    assert_eq!(
        select_generation(&selector, 5293, Some(Generation::V2)).unwrap(),
        Generation::V2
    );
    assert_eq!(
        select_generation(&selector, 5293, None).unwrap(),
        Generation::V1
    );
}
