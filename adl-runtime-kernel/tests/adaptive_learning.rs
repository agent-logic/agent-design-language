//! PVF: deterministic-core release-gating contract proof with a small resource profile.
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
};

use adl_runtime_kernel::*;
use ed25519_dalek::SigningKey;
use sha2::Digest;
use tokio_util::sync::CancellationToken;

const H: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const R: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn graph() -> ValidatedReasoningGraph {
    ValidatedReasoningGraph::validate(ReasoningGraphDefinition {
        schema: REASONING_GRAPH_SCHEMA.into(),
        version: 1,
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
    let mut value = CognitiveProfile {
        schema: COGNITIVE_PROFILE_SCHEMA.into(),
        profile_id: "profile".into(),
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
            profile_id: "profile".into(),
            revision: 1,
            identity_root: H.into(),
            fields: vec![],
            nonclaims: vec![],
            source_profile_sha256: String::new(),
            projection_sha256: H.into(),
        },
    };
    value.profile_sha256 = profile_digest(&value).unwrap();
    value
}
fn policy(profile: &CognitiveProfile) -> AdaptiveLearningPolicy {
    AdaptiveLearningPolicy {
        schema: ADAPTIVE_LEARNING_POLICY_SCHEMA.into(),
        profile_sha256: profile.profile_sha256.clone(),
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
fn policy_sha(policy: &AdaptiveLearningPolicy) -> String {
    let mut value = policy.clone();
    value.evidence.sort();
    value.allowed_feedback_sources.sort();
    value.allowed_feedback_sources.dedup();
    format!(
        "{:x}",
        sha2::Sha256::digest(serde_jcs::to_vec(&value).unwrap())
    )
}
fn loop_outcome(graph: &ValidatedReasoningGraph) -> LoopOutcome {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    runtime
        .block_on(execute_loop(
            graph,
            &LoopDefinition {
                target_score: 100,
                max_iterations: 1,
                deadline_millis: 5_000,
            },
            &RecordedObservation {
                observation_id: "observation".into(),
                score: 0,
                evidence_hash: H.into(),
            },
            AdaptationState::new(0, graph.hash(), H),
            CancellationToken::new(),
        ))
        .unwrap()
}
struct FixedTime;
impl TrustedTime for FixedTime {
    fn now_unix_millis(&self) -> u64 {
        100
    }
}
fn authority(key: &SigningKey) -> MutationAuthority {
    MutationAuthority::new(BTreeMap::from([(
        "review-key".into(),
        TrustedMutationKey {
            principal: "review-board".into(),
            verifying_key: key.verifying_key(),
        },
    )]))
}
fn patches() -> Vec<GraphPatch> {
    vec![GraphPatch::SetScoreDelta {
        node: "a".into(),
        score_delta: 2,
    }]
}
fn grant(
    graph: &ValidatedReasoningGraph,
    key: &SigningKey,
    patches: &[GraphPatch],
) -> MutationGrant {
    MutationGrant {
        schema: MUTATION_GRANT_SCHEMA.into(),
        grant_id: "grant".into(),
        principal: "review-board".into(),
        signing_key_id: "review-key".into(),
        graph_hash: graph.hash().into(),
        policy_hash: H.into(),
        provenance: "review-5831".into(),
        patch_hash: graph_patch_hash(patches).unwrap(),
        allowed_operations: BTreeSet::from([PatchKind::SetScoreDelta]),
        max_patches: 1,
        max_nodes: 8,
        max_edges: 8,
        expires_unix_millis: 1_000,
        signature: String::new(),
    }
    .sign(key)
    .unwrap()
}
fn gate(
    graph: &ValidatedReasoningGraph,
    outcome: &LoopOutcome,
    authority: MutationAuthority,
) -> MutationGate {
    MutationGate::new(
        graph.clone(),
        authority,
        Arc::new(FixedTime),
        H,
        16,
        Arc::new(AdaptationStore::new(outcome.state.clone())),
    )
    .unwrap()
}
fn input(
    graph: &ValidatedReasoningGraph,
    outcome: &LoopOutcome,
    profile: &CognitiveProfile,
    policy: &AdaptiveLearningPolicy,
) -> AdaptiveLearningInput {
    let mut proposed = graph.definition().clone();
    proposed.version = 2;
    proposed
        .nodes
        .iter_mut()
        .find(|node| node.id == "a")
        .unwrap()
        .score_delta = 2;
    let state = outcome.state.hash().unwrap();
    AdaptiveLearningInput {
        schema: ADAPTIVE_LEARNING_INPUT_SCHEMA.into(),
        history_id: "history".into(),
        sequence: 1,
        previous_history_sha256: None,
        profile_sha256: profile.profile_sha256.clone(),
        capability_envelope_sha256: R.into(),
        recurrence: 4,
        evaluation: LearningEvaluation {
            loop_event_sha256: outcome.replay.last().unwrap().hash.clone(),
            feedback_source: "review".into(),
            confidence_bps: 9_000,
            evidence_ids: vec!["feedback".into()],
        },
        adaptation: AdaptationDelta {
            before_state_sha256: state.clone(),
            after_state_sha256: state,
            rationale: "reviewed evidence supports bounded graph delta".into(),
            rollback_state_sha256: outcome.state.hash().unwrap(),
        },
        proposal: GraphProposal {
            proposal_id: "proposal".into(),
            before_graph_sha256: graph.hash().into(),
            proposed_graph: proposed,
            evidence_ids: vec!["feedback".into()],
        },
        decision: LearningDecision {
            disposition: LearningDisposition::Rejected,
            authority_sha256: H.into(),
            policy_sha256: policy_sha(policy),
            reason_code: "reviewed".into(),
        },
    }
}
struct Harness {
    graph: ValidatedReasoningGraph,
    profile: CognitiveProfile,
    policy: AdaptiveLearningPolicy,
    outcome: LoopOutcome,
    authority: MutationAuthority,
    gate: MutationGate,
    key: SigningKey,
    _durable_dir: tempfile::TempDir,
    durable: KernelDurableState,
}
fn harness() -> Harness {
    let graph = graph();
    let profile = profile();
    let policy = policy(&profile);
    let outcome = loop_outcome(&graph);
    let key = SigningKey::from_bytes(&[7; 32]);
    let authority = authority(&key);
    let gate = gate(&graph, &outcome, authority.clone());
    let durable_dir = tempfile::tempdir().unwrap();
    let durable = KernelDurableState::open(durable_dir.path()).unwrap();
    Harness {
        graph,
        profile,
        policy,
        outcome,
        authority,
        gate,
        key,
        _durable_dir: durable_dir,
        durable,
    }
}

#[test]
fn governed_acceptance_mutates_and_persists_exact_history() {
    let h = harness();
    let patches = patches();
    let grant = grant(&h.graph, &h.key, &patches);
    let initial_input = input(&h.graph, &h.outcome, &h.profile, &h.policy);
    let history = execute_governed_adaptive_learning(
        &h.gate,
        &h.durable,
        &h.profile,
        &initial_input,
        &h.policy,
        None,
        &h.outcome,
        &CancellationToken::new(),
        Some((&grant, &patches)),
    )
    .unwrap();
    assert_eq!(history.decision.disposition, LearningDisposition::Accepted);
    assert_eq!(history.recurrence, 4);
    assert_eq!(h.gate.evidence().len(), 1);
    let retained: AdaptiveLearningHistory = serde_json::from_slice(
        &h.durable
            .load_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN)
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(retained, history);
    assert_eq!(h.durable.governed_lifelog_len().unwrap(), 2);
    validate_governed_adaptive_learning_history(
        &history,
        &h.graph,
        &h.profile,
        &h.policy,
        None,
        &h.authority,
        &h.gate,
    )
    .unwrap();
    let rollback = rollback_governed_adaptive_learning(
        &history,
        &history.resulting_graph_sha256,
        &history.resulting_state_sha256,
        &h.graph,
        &h.profile,
        &h.policy,
        None,
        &h.authority,
        &h.gate,
        &h.durable,
    )
    .unwrap();
    assert_eq!(
        rollback,
        (
            h.graph.definition().clone(),
            h.outcome.state.hash().unwrap()
        )
    );
    if let Ok(relative) = std::env::var("ADL_NATIVE_SEMANTIC_OUTPUT") {
        let path = Path::new(&relative);
        assert!(
            !path.is_absolute()
                && path
                    .components()
                    .all(|c| matches!(c, std::path::Component::Normal(_)))
        );
        let out = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(path);
        std::fs::create_dir_all(out.parent().unwrap()).unwrap();
        std::fs::write(out, serde_jcs::to_vec(&history).unwrap()).unwrap();
    }
}

#[test]
fn rejected_and_cancelled_paths_are_durable_and_nonmutating() {
    for cancelled in [false, true] {
        let h = harness();
        let patches = patches();
        let grant = grant(&h.graph, &h.key, &patches);
        let input = input(&h.graph, &h.outcome, &h.profile, &h.policy);
        let token = CancellationToken::new();
        if cancelled {
            token.cancel();
        }
        let mutation = cancelled.then_some((&grant, patches.as_slice()));
        let history = execute_governed_adaptive_learning(
            &h.gate, &h.durable, &h.profile, &input, &h.policy, None, &h.outcome, &token, mutation,
        )
        .unwrap();
        assert_eq!(history.decision.disposition, LearningDisposition::Rejected);
        assert_eq!(history.resulting_graph_sha256, h.graph.hash());
        assert!(h.gate.evidence().is_empty());
        assert_eq!(h.durable.governed_lifelog_len().unwrap(), 2);
    }
}

#[test]
fn forged_grant_and_authority_fail_before_history_acceptance() {
    let h = harness();
    let patches = patches();
    let mut grant = grant(&h.graph, &h.key, &patches);
    grant.signature.replace_range(..2, "00");
    let input = input(&h.graph, &h.outcome, &h.profile, &h.policy);
    assert!(execute_governed_adaptive_learning(
        &h.gate,
        &h.durable,
        &h.profile,
        &input,
        &h.policy,
        None,
        &h.outcome,
        &CancellationToken::new(),
        Some((&grant, &patches)),
    )
    .is_err());
    assert!(h.gate.evidence().is_empty());
}

#[test]
fn predecessor_splice_and_sequence_overflow_fail_closed() {
    let h = harness();
    let input = input(&h.graph, &h.outcome, &h.profile, &h.policy);
    let first = execute_governed_adaptive_learning(
        &h.gate,
        &h.durable,
        &h.profile,
        &input,
        &h.policy,
        None,
        &h.outcome,
        &CancellationToken::new(),
        None,
    )
    .unwrap();
    let mut forged = first.clone();
    forged.history_id = "other".into();
    forged.history_sha256 = history_digest(&forged).unwrap();
    let mut next = input.clone();
    next.sequence = 2;
    next.previous_history_sha256 = Some(forged.history_sha256.clone());
    next.adaptation.before_state_sha256 = forged.resulting_state_sha256.clone();
    assert!(execute_governed_adaptive_learning(
        &h.gate,
        &h.durable,
        &h.profile,
        &next,
        &h.policy,
        Some(&forged),
        &h.outcome,
        &CancellationToken::new(),
        None,
    )
    .is_err());
    let mut overflow = first;
    overflow.sequence = u64::MAX;
    overflow.history_sha256 = history_digest(&overflow).unwrap();
    next.previous_history_sha256 = Some(overflow.history_sha256.clone());
    assert!(build_adaptive_learning_history(
        &h.graph,
        &h.profile,
        &next,
        &h.policy,
        Some(&overflow)
    )
    .is_err());
}

#[test]
fn tampered_history_and_rollback_never_return_attacker_hashes() {
    let h = harness();
    let patches = patches();
    let grant = grant(&h.graph, &h.key, &patches);
    let input = input(&h.graph, &h.outcome, &h.profile, &h.policy);
    let mut history = execute_governed_adaptive_learning(
        &h.gate,
        &h.durable,
        &h.profile,
        &input,
        &h.policy,
        None,
        &h.outcome,
        &CancellationToken::new(),
        Some((&grant, &patches)),
    )
    .unwrap();
    history.adaptation.rollback_state_sha256 = R.into();
    history.history_sha256 = history_digest(&history).unwrap();
    assert!(rollback_governed_adaptive_learning(
        &history,
        &history.resulting_graph_sha256,
        &history.resulting_state_sha256,
        &h.graph,
        &h.profile,
        &h.policy,
        None,
        &h.authority,
        &h.gate,
        &h.durable,
    )
    .is_err());
}

#[test]
fn recurrence_roundtrip_and_capacity_bounds_fail_closed() {
    let h = harness();
    let initial_input = input(&h.graph, &h.outcome, &h.profile, &h.policy);
    let history = execute_governed_adaptive_learning(
        &h.gate,
        &h.durable,
        &h.profile,
        &initial_input,
        &h.policy,
        None,
        &h.outcome,
        &CancellationToken::new(),
        None,
    )
    .unwrap();
    assert_eq!(history.recurrence, initial_input.recurrence);
    validate_adaptive_learning_history(&history, &h.graph, &h.profile, &h.policy, None).unwrap();
    for case in 0..4 {
        let h = harness();
        let mut input = input(&h.graph, &h.outcome, &h.profile, &h.policy);
        let mut policy = h.policy.clone();
        match case {
            0 => input.recurrence = 0,
            1 => policy.max_recurrence = 10_001,
            2 => policy.allowed_feedback_sources = (0..65).map(|n| format!("source-{n}")).collect(),
            _ => {
                policy.evidence = (0..257)
                    .map(|n| LearningEvidence {
                        id: format!("evidence-{n}"),
                        path: format!("evidence/{n}.json"),
                        sha256: H.into(),
                        revision_sha256: R.into(),
                    })
                    .collect()
            }
        }
        input.decision.policy_sha256 = policy_sha(&policy);
        assert!(execute_governed_adaptive_learning(
            &h.gate,
            &h.durable,
            &h.profile,
            &input,
            &policy,
            None,
            &h.outcome,
            &CancellationToken::new(),
            None,
        )
        .is_err());
    }
}

#[test]
fn unsafe_ids_rationale_paths_and_unknown_fields_fail_closed() {
    for case in 0..4 {
        let h = harness();
        let mut input = input(&h.graph, &h.outcome, &h.profile, &h.policy);
        let mut policy = h.policy.clone();
        match case {
            0 => input.history_id = "gho_secret".into(),
            1 => input.adaptation.rationale = "private_state leaked in rationale".into(),
            2 => policy.evidence[0].path = "/Users/operator/key".into(),
            _ => input.adaptation.rationale = "x".repeat(513),
        }
        input.decision.policy_sha256 = policy_sha(&policy);
        assert!(execute_governed_adaptive_learning(
            &h.gate,
            &h.durable,
            &h.profile,
            &input,
            &policy,
            None,
            &h.outcome,
            &CancellationToken::new(),
            None,
        )
        .is_err());
    }
    let h = harness();
    let mut value =
        serde_json::to_value(input(&h.graph, &h.outcome, &h.profile, &h.policy)).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("private_state".into(), true.into());
    assert!(serde_json::from_value::<AdaptiveLearningInput>(value).is_err());
}

#[test]
fn fixture_matrix_tracks_the_governed_negative_surface() {
    let matrix: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/adaptive_learning/matrix.json")).unwrap();
    assert!(matrix["negative_cases"].as_array().unwrap().len() >= 18);
    assert_eq!(matrix["schema"], "adl.adaptive_learning.fixture_matrix.v1");
}
