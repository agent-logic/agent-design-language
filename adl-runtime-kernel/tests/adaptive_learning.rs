//! PVF: deterministic-core release-gating contract proof with a small resource profile.
use adl_runtime_kernel::*;
use sha2::Digest;
use std::collections::BTreeSet;
use std::{fs, path::Path};

const H: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const R: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
fn graph(version: u64) -> ValidatedReasoningGraph {
    ValidatedReasoningGraph::validate(ReasoningGraphDefinition {
        schema: REASONING_GRAPH_SCHEMA.into(),
        version,
        entry: "a".into(),
        exits: BTreeSet::from(["b".into()]),
        nodes: vec![
            ReasoningNode {
                id: "a".into(),
                score_delta: 1,
            },
            ReasoningNode {
                id: "b".into(),
                score_delta: 1,
            },
        ],
        edges: vec![ReasoningEdge {
            from: "a".into(),
            to: "b".into(),
        }],
    })
    .unwrap()
}
fn profile() -> CognitiveProfile {
    let mut p = CognitiveProfile {
        schema: COGNITIVE_PROFILE_SCHEMA.into(),
        profile_id: "p".into(),
        revision: 1,
        previous_profile_sha256: None,
        identity_root: H.into(),
        continuity_head: H.into(),
        birthday_candidate_sha256: H.into(),
        identity_record_sha256: H.into(),
        continuity_record_sha256: H.into(),
        capability_envelope_sha256: R.into(),
        update_actor: "runtime".into(),
        update_reason: "governed profile".into(),
        added_fields: vec![],
        removed_fields: vec![],
        evidence: vec![],
        fields: vec![],
        nonclaims: vec![],
        redaction_policy_sha256: H.into(),
        policy_sha256: H.into(),
        canonical_input_sha256: H.into(),
        profile_sha256: String::new(),
        public_projection: PublicCognitiveProfile {
            schema: COGNITIVE_PROFILE_PUBLIC_SCHEMA.into(),
            profile_id: "p".into(),
            revision: 1,
            identity_root: H.into(),
            fields: vec![],
            nonclaims: vec![],
            source_profile_sha256: String::new(),
            projection_sha256: H.into(),
        },
    };
    p.profile_sha256 = profile_digest(&p).unwrap();
    p
}
fn policy(p: &CognitiveProfile) -> AdaptiveLearningPolicy {
    AdaptiveLearningPolicy {
        schema: ADAPTIVE_LEARNING_POLICY_SCHEMA.into(),
        profile_sha256: p.profile_sha256.clone(),
        capability_envelope_sha256: R.into(),
        authority_sha256: H.into(),
        evidence: vec![LearningEvidence {
            id: "feedback".into(),
            path: "evidence/feedback.json".into(),
            sha256: H.into(),
            revision_sha256: R.into(),
        }],
        max_recurrence: 4,
        allowed_feedback_sources: vec!["review".into()],
    }
}
fn input(
    g: &ValidatedReasoningGraph,
    p: &CognitiveProfile,
    policy: &AdaptiveLearningPolicy,
    d: LearningDisposition,
) -> AdaptiveLearningInput {
    let mut proposed = graph(2).definition().clone();
    proposed.nodes[0].score_delta = 2;
    let mut decision = LearningDecision {
        disposition: d,
        authority_sha256: H.into(),
        policy_sha256: String::new(),
        reason_code: "reviewed".into(),
    };
    let mut cp = policy.clone();
    cp.evidence.sort();
    cp.allowed_feedback_sources.sort();
    decision.policy_sha256 = {
        let bytes = serde_jcs::to_vec(&cp).unwrap();
        format!("{:x}", sha2::Sha256::digest(bytes))
    };
    AdaptiveLearningInput {
        schema: ADAPTIVE_LEARNING_INPUT_SCHEMA.into(),
        history_id: "history".into(),
        sequence: 1,
        previous_history_sha256: None,
        profile_sha256: p.profile_sha256.clone(),
        capability_envelope_sha256: R.into(),
        recurrence: 1,
        evaluation: LearningEvaluation {
            loop_event_sha256: H.into(),
            feedback_source: "review".into(),
            confidence_bps: 9000,
            evidence_ids: vec!["feedback".into()],
        },
        adaptation: AdaptationDelta {
            before_state_sha256: H.into(),
            after_state_sha256: R.into(),
            rationale: "evidence reviewed".into(),
            rollback_state_sha256: H.into(),
        },
        proposal: GraphProposal {
            proposal_id: "proposal".into(),
            before_graph_sha256: g.hash().into(),
            proposed_graph: proposed,
            evidence_ids: vec!["feedback".into()],
        },
        decision,
    }
}

#[test]
fn accepted_and_rejected_paths_are_deterministic() {
    let g = graph(1);
    let p = profile();
    let pol = policy(&p);
    for d in [LearningDisposition::Accepted, LearningDisposition::Rejected] {
        let i = input(&g, &p, &pol, d);
        let a = build_adaptive_learning_history(&g, &p, &i, &pol, None).unwrap();
        let b = build_adaptive_learning_history(&g, &p, &i, &pol, None).unwrap();
        assert_eq!(a, b);
        validate_adaptive_learning_history(&a, &g, &p, &pol, None).unwrap();
        if d == LearningDisposition::Accepted {
            if let Ok(relative) = std::env::var("ADL_NATIVE_SEMANTIC_OUTPUT") {
                let path = Path::new(&relative);
                assert!(
                    !path.is_absolute()
                        && path
                            .components()
                            .all(|part| matches!(part, std::path::Component::Normal(_)))
                );
                let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
                let output = root.join(path);
                fs::create_dir_all(output.parent().unwrap()).unwrap();
                fs::write(output, serde_jcs::to_vec(&a).unwrap()).unwrap();
            }
        }
        if d == LearningDisposition::Rejected {
            assert_eq!(a.resulting_graph_sha256, g.hash());
            assert_eq!(a.resulting_state_sha256, H)
        }
    }
}
#[test]
fn replay_prefix_and_substitution_fail_closed() {
    let g = graph(1);
    let p = profile();
    let pol = policy(&p);
    let first = build_adaptive_learning_history(
        &g,
        &p,
        &input(&g, &p, &pol, LearningDisposition::Rejected),
        &pol,
        None,
    )
    .unwrap();
    let mut i = input(&g, &p, &pol, LearningDisposition::Rejected);
    i.sequence = 2;
    i.recurrence = 2;
    i.previous_history_sha256 = Some(first.history_sha256.clone());
    assert!(build_adaptive_learning_history(&g, &p, &i, &pol, Some(&first)).is_ok());
    i.previous_history_sha256 = Some(H.into());
    assert!(build_adaptive_learning_history(&g, &p, &i, &pol, Some(&first)).is_err())
}
#[test]
fn missing_forged_and_private_evidence_fail_closed() {
    let g = graph(1);
    let p = profile();
    let pol = policy(&p);
    for case in 0..3 {
        let mut i = input(&g, &p, &pol, LearningDisposition::Accepted);
        let mut q = pol.clone();
        match case {
            0 => i.evaluation.evidence_ids.clear(),
            1 => q.evidence[0].revision_sha256 = H.into(),
            _ => q.evidence[0].path = "/Users/private/key".into(),
        }
        assert!(build_adaptive_learning_history(&g, &p, &i, &q, None).is_err())
    }
}
#[test]
fn invalid_graph_bounds_authority_and_rejected_mutation_fail_closed() {
    let g = graph(1);
    let p = profile();
    let pol = policy(&p);
    for case in 0..4 {
        let mut i = input(&g, &p, &pol, LearningDisposition::Rejected);
        match case {
            0 => i.proposal.before_graph_sha256 = H.into(),
            1 => i.recurrence = 5,
            2 => i.decision.authority_sha256 = R.into(),
            _ => i.evaluation.confidence_bps = 0,
        }
        assert!(build_adaptive_learning_history(&g, &p, &i, &pol, None).is_err())
    }
}
#[test]
fn rollback_requires_exact_accepted_heads() {
    let g = graph(1);
    let p = profile();
    let pol = policy(&p);
    let h = build_adaptive_learning_history(
        &g,
        &p,
        &input(&g, &p, &pol, LearningDisposition::Accepted),
        &pol,
        None,
    )
    .unwrap();
    assert_eq!(
        rollback_adaptive_learning(&h, &h.resulting_graph_sha256, &h.resulting_state_sha256)
            .unwrap(),
        (g.hash().into(), H.into())
    );
    assert!(rollback_adaptive_learning(&h, H, H).is_err())
}
#[test]
fn unknown_fields_and_fixture_matrix_are_enforced() {
    let matrix: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/adaptive_learning/matrix.json")).unwrap();
    assert!(matrix["negative_cases"].as_array().unwrap().len() >= 12);
    let g = graph(1);
    let p = profile();
    let pol = policy(&p);
    let mut value =
        serde_json::to_value(input(&g, &p, &pol, LearningDisposition::Accepted)).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("private_state".into(), true.into());
    assert!(serde_json::from_value::<AdaptiveLearningInput>(value).is_err())
}
