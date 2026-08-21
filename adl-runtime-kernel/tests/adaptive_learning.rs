//! PVF: deterministic-core release-gating contract proof with a small resource profile.
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
};

use adl_runtime_kernel::*;
use ed25519_dalek::SigningKey;
use serde_json::json;
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
    let authority_key = SigningKey::from_bytes(&[7; 32]);
    let mut authority_context = CognitiveAuthorityContext {
        authority_id: "cognitive-board".into(),
        key_id: "cognitive-key-1".into(),
        epoch: 1,
        context_sha256: String::new(),
        verifying_key_hex: hex::encode(authority_key.verifying_key().as_bytes()),
    };
    authority_context.context_sha256 =
        authority_context_payload_digest(&authority_context).unwrap();
    let authority_statement = CognitiveAuthorityStatement {
        schema: COGNITIVE_AUTHORITY_STATEMENT_SCHEMA.into(),
        authority_context_sha256: authority_context.context_sha256.clone(),
        profile_id: "profile".into(),
        revision: 1,
        previous_profile_sha256: None,
        canonical_input_sha256: H.into(),
        policy_sha256: H.into(),
        evidence_sha256: H.into(),
        signature: String::new(),
    }
    .sign(&authority_key)
    .unwrap();
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
        authority: CognitiveAuthorityProof {
            context: authority_context,
            statement: authority_statement,
            rotation: None,
        },
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
fn loop_outcome(graph: &ValidatedReasoningGraph, policy_sha256: &str) -> LoopOutcome {
    loop_outcome_from(graph, AdaptationState::new(0, graph.hash(), policy_sha256))
}
fn loop_outcome_from(graph: &ValidatedReasoningGraph, state: AdaptationState) -> LoopOutcome {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    runtime
        .block_on(execute_loop(
            graph,
            &loop_definition(),
            &observation(),
            state,
            CancellationToken::new(),
        ))
        .unwrap()
}
fn loop_definition() -> LoopDefinition {
    LoopDefinition {
        target_score: 100,
        max_iterations: 1,
        deadline_millis: 5_000,
    }
}
fn observation() -> RecordedObservation {
    RecordedObservation {
        observation_id: "observation".into(),
        score: 0,
        evidence_hash: H.into(),
    }
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
    patches_with_score(2)
}
fn patches_with_score(score_delta: i64) -> Vec<GraphPatch> {
    vec![GraphPatch::SetScoreDelta {
        node: "a".into(),
        score_delta,
    }]
}
fn grant(
    graph: &ValidatedReasoningGraph,
    key: &SigningKey,
    patches: &[GraphPatch],
    policy_sha256: &str,
) -> MutationGrant {
    MutationGrant {
        schema: MUTATION_GRANT_SCHEMA.into(),
        grant_id: "grant".into(),
        principal: "review-board".into(),
        signing_key_id: "review-key".into(),
        graph_hash: graph.hash().into(),
        policy_hash: policy_sha256.into(),
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
    policy_sha256: &str,
) -> MutationGate {
    MutationGate::new(
        graph.clone(),
        authority,
        Arc::new(FixedTime),
        policy_sha256,
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
    proposed.version = proposed.version.checked_add(1).unwrap();
    proposed
        .nodes
        .iter_mut()
        .find(|node| node.id == "a")
        .unwrap()
        .score_delta += 1;
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
    let policy_sha256 = policy_sha(&policy);
    let outcome = loop_outcome(&graph, &policy_sha256);
    let key = SigningKey::from_bytes(&[7; 32]);
    let authority = authority(&key);
    let gate = gate(&graph, &outcome, authority.clone(), &policy_sha256);
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
    let grant = grant(&h.graph, &h.key, &patches, &policy_sha(&h.policy));
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
    assert_eq!(h.durable.governed_lifelog_len().unwrap(), 1);
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
        let grant = grant(&h.graph, &h.key, &patches, &policy_sha(&h.policy));
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
        assert_eq!(h.durable.governed_lifelog_len().unwrap(), 1);
    }
}

#[test]
fn forged_grant_and_authority_fail_before_history_acceptance() {
    let h = harness();
    let patches = patches();
    let mut grant = grant(&h.graph, &h.key, &patches, &policy_sha(&h.policy));
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
    let grant = grant(&h.graph, &h.key, &patches, &policy_sha(&h.policy));
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
fn policy_digest_mismatch_fails_before_mutation_or_persistence() {
    let h = harness();
    let patches = patches();
    let mismatched_grant = grant(&h.graph, &h.key, &patches, H);
    let input = input(&h.graph, &h.outcome, &h.profile, &h.policy);
    let before_state = h.gate.adaptation().state();
    let error = execute_governed_adaptive_learning(
        &h.gate,
        &h.durable,
        &h.profile,
        &input,
        &h.policy,
        None,
        &h.outcome,
        &CancellationToken::new(),
        Some((&mismatched_grant, &patches)),
    )
    .unwrap_err();
    assert!(error.contains(&AdaptiveLearningRejection::InvalidAuthority));
    assert_eq!(h.gate.graph().hash(), h.graph.hash());
    assert_eq!(h.gate.adaptation().state(), before_state);
    assert!(h.gate.evidence().is_empty());
    assert!(h
        .durable
        .load_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN)
        .unwrap()
        .is_none());
}

#[test]
fn proposal_patch_mismatch_and_durable_collision_are_nonmutating() {
    for durable_collision in [false, true] {
        let h = harness();
        let patches = if durable_collision {
            patches()
        } else {
            patches_with_score(3)
        };
        let grant = grant(&h.graph, &h.key, &patches, &policy_sha(&h.policy));
        let input = input(&h.graph, &h.outcome, &h.profile, &h.policy);
        if durable_collision {
            h.durable
                .store_governed_state(
                    &adaptive_learning_history_domain("history", 1),
                    b"collision",
                )
                .unwrap();
        }
        let before_graph = h.gate.graph();
        let before_state = h.gate.adaptation().state();
        let error = execute_governed_adaptive_learning(
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
        .unwrap_err();
        assert_eq!(h.gate.graph().hash(), before_graph.hash());
        assert_eq!(h.gate.adaptation().state(), before_state);
        assert!(h.gate.evidence().is_empty());
        assert!(h
            .durable
            .load_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN)
            .unwrap()
            .is_none());
        if durable_collision {
            assert!(error.contains(&AdaptiveLearningRejection::DurableWriteFailed));
            assert!(load_adaptive_learning_history(&h.durable, "history", 1).is_err());
        } else {
            assert!(error.contains(&AdaptiveLearningRejection::InvalidGraph));
            assert!(load_adaptive_learning_history(&h.durable, "history", 1)
                .unwrap()
                .is_none());
        }
    }
}

fn pending_value(durable: &KernelDurableState) -> serde_json::Value {
    serde_json::from_slice(
        &durable
            .load_governed_state(&adaptive_learning_pending_domain("ignored", 0))
            .unwrap()
            .unwrap(),
    )
    .unwrap()
}

fn store_pending(durable: &KernelDurableState, value: &serde_json::Value) {
    durable
        .store_governed_state(
            &adaptive_learning_pending_domain("ignored", 0),
            &serde_jcs::to_vec(value).unwrap(),
        )
        .unwrap();
}

#[test]
fn startup_discovers_reserved_intent_and_rejects_tampering() {
    let h = harness();
    let before_snapshot = h.gate.snapshot_bytes().unwrap();
    let patch_set = patches();
    let grant = grant(&h.graph, &h.key, &patch_set, &policy_sha(&h.policy));
    execute_governed_adaptive_learning(
        &h.gate,
        &h.durable,
        &h.profile,
        &input(&h.graph, &h.outcome, &h.profile, &h.policy),
        &h.policy,
        None,
        &h.outcome,
        &CancellationToken::new(),
        Some((&grant, &patch_set)),
    )
    .unwrap();
    let mut reserved = pending_value(&h.durable);
    reserved["status"] = json!("reserved");

    let directory = tempfile::tempdir().unwrap();
    let durable = KernelDurableState::open(directory.path()).unwrap();
    store_pending(&durable, &reserved);
    let mut restarted = h.gate.restore_from_snapshot(&before_snapshot).unwrap();
    assert_eq!(
        reconcile_adaptive_learning_startup(
            &durable,
            &mut restarted,
            &h.profile,
            &h.policy,
            &h.authority,
        )
        .unwrap(),
        None
    );
    assert_eq!(pending_value(&durable)["status"], json!("aborted"));
    assert_eq!(restarted.snapshot_bytes().unwrap(), before_snapshot);

    let directory = tempfile::tempdir().unwrap();
    let durable = KernelDurableState::open(directory.path()).unwrap();
    let mut tampered: AdaptiveLearningHistory =
        serde_json::from_value(reserved["history"].clone()).unwrap();
    tampered.adaptation.rationale = "attacker rehashed replacement rationale".into();
    tampered.history_sha256 = history_digest(&tampered).unwrap();
    reserved["history"] = serde_json::to_value(tampered).unwrap();
    store_pending(&durable, &reserved);
    let mut restarted = h.gate.restore_from_snapshot(&before_snapshot).unwrap();
    assert_eq!(
        reconcile_adaptive_learning_startup(
            &durable,
            &mut restarted,
            &h.profile,
            &h.policy,
            &h.authority,
        ),
        Err(AdaptiveLearningRejection::NonCanonicalHistory)
    );
    assert_eq!(restarted.snapshot_bytes().unwrap(), before_snapshot);
}

#[test]
fn startup_completes_committed_intent_and_restores_aborted_live_gate() {
    let mut h = harness();
    let before_snapshot = h.gate.snapshot_bytes().unwrap();
    let patch_set = patches();
    let grant = grant(&h.graph, &h.key, &patch_set, &policy_sha(&h.policy));
    let history = execute_governed_adaptive_learning(
        &h.gate,
        &h.durable,
        &h.profile,
        &input(&h.graph, &h.outcome, &h.profile, &h.policy),
        &h.policy,
        None,
        &h.outcome,
        &CancellationToken::new(),
        Some((&grant, &patch_set)),
    )
    .unwrap();
    let committed = pending_value(&h.durable);
    let encoded = serde_jcs::to_vec(&history).unwrap();

    let directory = tempfile::tempdir().unwrap();
    let durable = KernelDurableState::open(directory.path()).unwrap();
    store_pending(&durable, &committed);
    durable
        .store_governed_state(&adaptive_learning_history_domain("history", 1), &encoded)
        .unwrap();
    durable
        .store_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN, &encoded)
        .unwrap();
    let mut restarted = h.gate.restore_from_snapshot(&before_snapshot).unwrap();
    assert_eq!(
        reconcile_adaptive_learning_startup(
            &durable,
            &mut restarted,
            &h.profile,
            &h.policy,
            &h.authority,
        )
        .unwrap(),
        Some(history.clone())
    );
    assert_eq!(restarted.graph().hash(), history.resulting_graph_sha256);

    let directory = tempfile::tempdir().unwrap();
    let durable = KernelDurableState::open(directory.path()).unwrap();
    let mut aborted = committed;
    aborted["status"] = json!("aborted");
    store_pending(&durable, &aborted);
    assert_eq!(
        reconcile_adaptive_learning_startup(
            &durable,
            &mut h.gate,
            &h.profile,
            &h.policy,
            &h.authority,
        )
        .unwrap(),
        None
    );
    assert_eq!(h.gate.snapshot_bytes().unwrap(), before_snapshot);
}

#[test]
fn transactional_completion_and_postcheck_failure_leave_gate_unchanged() {
    let h = harness();
    let patch_set = patches();
    let grant = grant(&h.graph, &h.key, &patch_set, &policy_sha(&h.policy));
    let before = h.gate.snapshot_bytes().unwrap();
    assert!(h
        .gate
        .apply_and_migrate_transactional(&grant, &patch_set, |_, _, _| h
            .durable
            .compare_and_set_governed_state("completion-cas", Some(b"missing"), b"must-not-commit",)
            .unwrap())
        .is_err());
    assert_eq!(h.gate.snapshot_bytes().unwrap(), before);
    assert!(h
        .gate
        .apply_and_migrate_transactional(&grant, &patch_set, |evidence, graph, _| {
            evidence.after_hash == R && graph.hash() == R
        })
        .is_err());
    assert_eq!(h.gate.snapshot_bytes().unwrap(), before);
}

#[test]
fn concurrent_adaptive_executions_have_one_authoritative_winner() {
    use std::sync::Barrier;

    let h = harness();
    let gate = Arc::new(h.gate);
    let durable = Arc::new(h.durable);
    let profile = Arc::new(h.profile);
    let policy = Arc::new(h.policy);
    let outcome = Arc::new(h.outcome);
    let patch_set = patches();
    let grant = grant(&h.graph, &h.key, &patch_set, &policy_sha(&policy));
    let initial = input(&h.graph, &outcome, &profile, &policy);
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let gate = gate.clone();
        let durable = durable.clone();
        let profile = profile.clone();
        let policy = policy.clone();
        let outcome = outcome.clone();
        let patch_set = patch_set.clone();
        let grant = grant.clone();
        let initial = initial.clone();
        let barrier = barrier.clone();
        workers.push(std::thread::spawn(move || {
            barrier.wait();
            execute_governed_adaptive_learning(
                &gate,
                &durable,
                &profile,
                &initial,
                &policy,
                None,
                &outcome,
                &CancellationToken::new(),
                Some((&grant, &patch_set)),
            )
        }));
    }
    barrier.wait();
    let results = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(gate.evidence().len(), 1);
    let retained: AdaptiveLearningHistory = serde_json::from_slice(
        &durable
            .load_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN)
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(retained.resulting_graph_sha256, gate.graph().hash());
}

#[test]
fn resident_cycle_production_path_accepts_and_rejects_without_fixture_entrypoint() {
    let h = harness();
    let patch_set = patches();
    let grant = grant(&h.graph, &h.key, &patch_set, &policy_sha(&h.policy));
    let initial = input(&h.graph, &h.outcome, &h.profile, &h.policy);
    let accepted = execute_resident_adaptive_learning_cycle(
        "resident-agent",
        &h.profile.continuity_head,
        &h.gate,
        &h.durable,
        &h.profile,
        &initial,
        &h.policy,
        None,
        &h.outcome,
        &CancellationToken::new(),
        Some((&grant, &patch_set)),
    )
    .unwrap();
    assert_eq!(accepted.status, ResidentAdaptiveLearningStatus::Accepted);
    assert_eq!(accepted.resident_id, "resident-agent");
    assert_eq!(accepted.profile_sha256, h.profile.profile_sha256);
    assert_eq!(
        accepted.capability_envelope_sha256,
        h.profile.capability_envelope_sha256
    );
    assert_eq!(accepted.continuity_head_sha256, h.profile.continuity_head);
    assert!(accepted.mutation_evidence_retained);
    assert_eq!(h.gate.evidence().len(), 1);

    let mut next_outcome = h.outcome.clone();
    next_outcome.state = h.gate.adaptation().state();
    let current_graph = h.gate.graph();
    let mut rejected_input = input(&current_graph, &next_outcome, &h.profile, &h.policy);
    let previous = load_adaptive_learning_history(&h.durable, "history", 1)
        .unwrap()
        .unwrap();
    rejected_input.sequence = 2;
    rejected_input.previous_history_sha256 = Some(previous.history_sha256.clone());
    rejected_input.adaptation.before_state_sha256 = previous.resulting_state_sha256.clone();
    rejected_input.adaptation.after_state_sha256 = previous.resulting_state_sha256.clone();
    rejected_input.proposal.before_graph_sha256 = current_graph.hash().into();
    rejected_input.decision.disposition = LearningDisposition::Rejected;
    rejected_input.decision.authority_sha256 = h.policy.authority_sha256.clone();
    let before_graph = h.gate.graph().hash().to_owned();
    let before_state = h.gate.adaptation().state().hash().unwrap();
    let rejected = execute_resident_adaptive_learning_cycle(
        "resident-agent",
        &h.profile.continuity_head,
        &h.gate,
        &h.durable,
        &h.profile,
        &rejected_input,
        &h.policy,
        Some(&previous),
        &next_outcome,
        &CancellationToken::new(),
        None,
    )
    .unwrap();
    assert_eq!(rejected.status, ResidentAdaptiveLearningStatus::Rejected);
    assert!(!rejected.mutation_evidence_retained);
    assert_eq!(h.gate.graph().hash(), before_graph);
    assert_eq!(h.gate.adaptation().state().hash().unwrap(), before_state);
}

#[test]
fn resident_cycle_invalid_bindings_fail_before_mutation_or_history() {
    let h = harness();
    let patch_set = patches();
    let grant = grant(&h.graph, &h.key, &patch_set, &policy_sha(&h.policy));
    let input = input(&h.graph, &h.outcome, &h.profile, &h.policy);
    let before_graph = h.gate.graph().hash().to_owned();
    let before_state = h.gate.adaptation().state().hash().unwrap();
    let error = execute_resident_adaptive_learning_cycle(
        "resident-agent",
        R,
        &h.gate,
        &h.durable,
        &h.profile,
        &input,
        &h.policy,
        None,
        &h.outcome,
        &CancellationToken::new(),
        Some((&grant, &patch_set)),
    )
    .unwrap_err();
    assert!(error.contains(&AdaptiveLearningRejection::InvalidAuthority));
    assert_eq!(h.gate.graph().hash(), before_graph);
    assert_eq!(h.gate.adaptation().state().hash().unwrap(), before_state);
    assert!(h.gate.evidence().is_empty());
    assert!(h
        .durable
        .load_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN)
        .unwrap()
        .is_none());
}

#[test]
fn resident_cycle_restart_restores_and_continues_deterministically() {
    let h = harness();
    let patch_set = patches();
    let first_grant = grant(&h.graph, &h.key, &patch_set, &policy_sha(&h.policy));
    let first = execute_resident_adaptive_learning_cycle(
        "resident-agent",
        &h.profile.continuity_head,
        &h.gate,
        &h.durable,
        &h.profile,
        &input(&h.graph, &h.outcome, &h.profile, &h.policy),
        &h.policy,
        None,
        &h.outcome,
        &CancellationToken::new(),
        Some((&first_grant, &patch_set)),
    )
    .unwrap();
    let after_snapshot = h.gate.snapshot_bytes().unwrap();
    drop(h.gate);
    drop(h.durable);

    let durable = KernelDurableState::open(h._durable_dir.path()).unwrap();
    let mut gate = MutationGate::restore(
        &after_snapshot,
        h.authority.clone(),
        Arc::new(FixedTime),
        16,
    )
    .unwrap();
    let restored = reconcile_resident_adaptive_learning_startup(
        "resident-agent",
        &h.profile.continuity_head,
        &durable,
        &mut gate,
        &h.profile,
        &h.policy,
        &h.authority,
    )
    .unwrap()
    .unwrap();
    assert_eq!(restored.status, ResidentAdaptiveLearningStatus::Restored);
    assert_eq!(restored.history_sha256, first.history_sha256);

    let previous = load_adaptive_learning_history(&durable, "history", 1)
        .unwrap()
        .unwrap();
    let second_graph = gate.graph();
    let mut second_outcome = h.outcome.clone();
    second_outcome.state = gate.adaptation().state();
    let second_patches = patches_with_score(3);
    let mut second_grant = grant(
        &second_graph,
        &h.key,
        &second_patches,
        &policy_sha(&h.policy),
    );
    second_grant.grant_id = "resident-grant-2".into();
    second_grant = second_grant.sign(&h.key).unwrap();
    let mut second_input = input(&second_graph, &second_outcome, &h.profile, &h.policy);
    second_input.sequence = 2;
    second_input.previous_history_sha256 = Some(previous.history_sha256.clone());
    let continued = execute_resident_adaptive_learning_cycle(
        "resident-agent",
        &h.profile.continuity_head,
        &gate,
        &durable,
        &h.profile,
        &second_input,
        &h.policy,
        Some(&previous),
        &second_outcome,
        &CancellationToken::new(),
        Some((&second_grant, &second_patches)),
    )
    .unwrap();
    assert_eq!(continued.status, ResidentAdaptiveLearningStatus::Accepted);
    assert_eq!(continued.sequence, 2);
    assert_eq!(
        load_adaptive_learning_history(&durable, "history", 2)
            .unwrap()
            .unwrap()
            .history_sha256,
        continued.history_sha256
    );
}

#[test]
fn two_sequence_history_survives_restart_and_supports_authoritative_rollback() {
    let Harness {
        graph,
        profile,
        policy,
        outcome,
        authority,
        gate,
        key,
        _durable_dir: durable_dir,
        durable,
    } = harness();
    let first_patches = patches();
    let first_grant = grant(&graph, &key, &first_patches, &policy_sha(&policy));
    let first_input = input(&graph, &outcome, &profile, &policy);
    let first = execute_governed_adaptive_learning(
        &gate,
        &durable,
        &profile,
        &first_input,
        &policy,
        None,
        &outcome,
        &CancellationToken::new(),
        Some((&first_grant, &first_patches)),
    )
    .unwrap();
    let snapshot = gate.snapshot_bytes().unwrap();
    drop(gate);
    drop(durable);

    let durable = KernelDurableState::open(durable_dir.path()).unwrap();
    let gate =
        MutationGate::restore(&snapshot, authority.clone(), Arc::new(FixedTime), 16).unwrap();
    assert_eq!(
        load_adaptive_learning_history(&durable, "history", 1)
            .unwrap()
            .as_ref(),
        Some(&first)
    );
    let second_before_graph = gate.graph();
    let mut second_outcome = outcome.clone();
    second_outcome.state = gate.adaptation().state();
    let second_patches = patches_with_score(3);
    let mut second_grant = grant(
        &second_before_graph,
        &key,
        &second_patches,
        &policy_sha(&policy),
    );
    second_grant.grant_id = "grant-2".into();
    second_grant = second_grant.sign(&key).unwrap();
    let mut second_input = input(&second_before_graph, &second_outcome, &profile, &policy);
    second_input.sequence = 2;
    second_input.previous_history_sha256 = Some(first.history_sha256.clone());
    let second = execute_governed_adaptive_learning(
        &gate,
        &durable,
        &profile,
        &second_input,
        &policy,
        Some(&first),
        &second_outcome,
        &CancellationToken::new(),
        Some((&second_grant, &second_patches)),
    )
    .unwrap();
    let snapshot = gate.snapshot_bytes().unwrap();
    drop(gate);
    drop(durable);

    let durable = KernelDurableState::open(durable_dir.path()).unwrap();
    let gate =
        MutationGate::restore(&snapshot, authority.clone(), Arc::new(FixedTime), 16).unwrap();
    assert_eq!(
        load_adaptive_learning_history(&durable, "history", 1).unwrap(),
        Some(first.clone())
    );
    assert_eq!(
        load_adaptive_learning_history(&durable, "history", 2).unwrap(),
        Some(second.clone())
    );
    assert_eq!(
        rollback_governed_adaptive_learning(
            &second,
            &second.resulting_graph_sha256,
            &second.resulting_state_sha256,
            &second_before_graph,
            &profile,
            &policy,
            Some(&first),
            &authority,
            &gate,
            &durable,
        )
        .unwrap(),
        (
            second_before_graph.definition().clone(),
            first.resulting_state_sha256.clone()
        )
    );
}

#[test]
fn fixture_matrix_tracks_the_governed_negative_surface() {
    let matrix: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/adaptive_learning/matrix.json")).unwrap();
    assert!(matrix["negative_cases"].as_array().unwrap().len() >= 18);
    assert_eq!(matrix["schema"], "adl.adaptive_learning.fixture_matrix.v1");
}
