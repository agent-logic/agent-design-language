use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};

use adl_runtime_kernel::{
    bootstrap_reasoning_services, build_live_assembly, feature_dispositions, graph_patch_hash,
    rollback_candidate, AdaptationState, AdaptationStore, AdapterKind, AdvisorySignals,
    CognitionGates, DegradedOperationExecutor, DomainWork, FeatureDispositionKind, GraphPatch,
    LiveBindings, LoopDefinition, LoopStatus, MutationAuthority, MutationGate, MutationGrant,
    OperationExecutor, ParityBCognitionDisposition, ParityBError, ParityBExecutor, ParityBRequest,
    PatchKind, ReasoningEdge, ReasoningGraphDefinition, ReasoningNode, RecordedObservation,
    RuntimeRecorder, TimeQualificationBounds, TimeSample, TimeSampleError, TimeSampleSource,
    TrustedMutationKey, TrustedTime, PARITY_B_REQUEST_SCHEMA, REASONING_GRAPH_SCHEMA,
    REQUIRED_OPERATIONAL_ADAPTERS,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;

struct FixedSample;

#[async_trait]
impl TimeSampleSource for FixedSample {
    async fn sample(&self) -> Result<TimeSample, TimeSampleError> {
        Ok(TimeSample {
            source: "parity-b-test-time".to_owned(),
            unix_millis: 1_720_000_000_000,
            offset_millis: 0,
            round_trip: Duration::from_millis(1),
        })
    }
}

fn hash(value: &[u8]) -> String {
    blake3::hash(value).to_hex().to_string()
}

fn graph() -> ReasoningGraphDefinition {
    ReasoningGraphDefinition {
        schema: REASONING_GRAPH_SCHEMA.to_owned(),
        version: 1,
        entry: "observe".to_owned(),
        exits: BTreeSet::from(["decide".to_owned()]),
        nodes: vec![
            ReasoningNode {
                id: "observe".to_owned(),
                score_delta: 1,
            },
            ReasoningNode {
                id: "evaluate".to_owned(),
                score_delta: 1,
            },
            ReasoningNode {
                id: "decide".to_owned(),
                score_delta: 1,
            },
        ],
        edges: vec![
            ReasoningEdge {
                from: "observe".to_owned(),
                to: "evaluate".to_owned(),
            },
            ReasoningEdge {
                from: "evaluate".to_owned(),
                to: "decide".to_owned(),
            },
        ],
    }
}

fn policy_key() -> SigningKey {
    SigningKey::from_bytes(&[51; 32])
}

fn checkpoint_key() -> SigningKey {
    SigningKey::from_bytes(&[52; 32])
}

fn executor() -> ParityBExecutor {
    ParityBExecutor::new(
        BTreeMap::from([("policy-review".to_owned(), policy_key().verifying_key())]),
        "checkpoint",
        checkpoint_key(),
        None,
    )
    .unwrap()
}

fn restore_executor(bytes: &[u8]) -> Result<ParityBExecutor, ParityBError> {
    ParityBExecutor::restore(
        bytes,
        BTreeMap::from([("policy-review".to_owned(), policy_key().verifying_key())]),
        "checkpoint",
        checkpoint_key(),
        None,
    )
}

fn request() -> ParityBRequest {
    let evidence_hash = hash(b"authenticated-observation");
    let mut request = ParityBRequest {
        schema: PARITY_B_REQUEST_SCHEMA.to_owned(),
        graph: graph(),
        policy_hash: hash(b"parity-b-policy"),
        observation: RecordedObservation {
            observation_id: "observation-1".to_owned(),
            score: 0,
            evidence_hash: evidence_hash.clone(),
        },
        loop_definition: LoopDefinition {
            target_score: 7,
            max_iterations: 4,
            deadline_millis: 1_000,
        },
        signals: AdvisorySignals {
            provenance: adl_runtime_kernel::SignalProvenance::Policy,
            evidence_hash,
            risk: 10,
            uncertainty: 20,
            conflict: 5,
            affect_adjustment: 15,
            curiosity_steps: 1,
            theory_of_mind_confidence: 60,
            observable_interaction_only: true,
            asserted_claims: BTreeSet::new(),
        },
        gates: CognitionGates {
            freedom_allowed: true,
            shutdown_requested: false,
            review_required: false,
            constructability_satisfied: true,
            mutation_allowed: true,
        },
        resume: None,
        execution_slice_iterations: 4,
        mutation: None,
        policy_key_id: String::new(),
        policy_signature: String::new(),
    };
    adl_runtime_kernel::sign_policy_request(&mut request, "policy-review", &policy_key()).unwrap();
    request
}

fn live_bindings(recorder: RuntimeRecorder, executor: Arc<ParityBExecutor>) -> LiveBindings {
    let operation_executors = REQUIRED_OPERATIONAL_ADAPTERS
        .into_iter()
        .map(|kind| {
            let value: Arc<dyn OperationExecutor> = if kind == AdapterKind::Agent {
                executor.clone()
            } else {
                Arc::new(DegradedOperationExecutor::new("not configured"))
            };
            (kind, value)
        })
        .collect();
    LiveBindings {
        recorder: recorder.clone(),
        operation_executors,
        permit_keys: BTreeMap::from([(
            "test-operations".to_owned(),
            SigningKey::from_bytes(&[7; 32]).verifying_key(),
        )]),
        reasoning: bootstrap_reasoning_services(recorder).unwrap(),
        time_source: Arc::new(FixedSample),
        time_bounds: TimeQualificationBounds {
            timeout: Duration::from_secs(1),
            max_offset: Duration::from_millis(10),
            max_round_trip: Duration::from_millis(10),
        },
    }
}

#[tokio::test]
async fn live_graph_executes_through_guardian_canonical_ingress() {
    let recorder = RuntimeRecorder::new(256);
    let executor = Arc::new(executor());
    let assembly = build_live_assembly(live_bindings(recorder.clone(), executor.clone())).unwrap();
    let ingress = assembly.canonical_ingress.clone();
    let kernel = adl_runtime_kernel::Kernel::new(assembly.topology, recorder)
        .start()
        .await
        .unwrap();
    let work = DomainWork {
        schema: adl_runtime_kernel::DOMAIN_WORK_SCHEMA.to_owned(),
        work_id: "parity-b-live-graph".to_owned(),
        kind: "parity-a".to_owned(),
        payload: serde_json::to_vec(&request()).unwrap(),
    };
    let result = ingress.submit(work, hash(b"correlation")).await.unwrap();
    assert_eq!(result.accepted_sequence, 1);
    let receipt = executor.receipt("parity-b-live-graph").unwrap().unwrap();
    assert_eq!(receipt.disposition, ParityBCognitionDisposition::Execute);
    assert_eq!(receipt.loop_status, LoopStatus::Converged);
    assert_eq!(receipt.final_score, 9);
    assert_eq!(
        kernel.shutdown(Duration::from_secs(1)).await.unwrap(),
        adl_runtime_kernel::KernelExit::Clean
    );
}

#[tokio::test]
async fn bounded_loop_resume_preserves_budgets_and_effect_identity() {
    let executor = executor();
    let mut body = request();
    body.execution_slice_iterations = 1;
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    let initial_operation = operation("bounded-loop", &body);
    let first = executor.execute(&initial_operation).await.unwrap();
    let replay = executor.execute(&initial_operation).await.unwrap();
    assert_eq!(first, replay);
    let first_receipt: adl_runtime_kernel::ParityBReceipt = serde_json::from_slice(&first).unwrap();
    let first_resume = first_receipt
        .resume
        .clone()
        .expect("one-iteration slice resumes");
    assert_eq!(first_resume.completed_iterations, 1);
    assert!(first_resume.remaining_deadline_millis < body.loop_definition.deadline_millis);
    let checkpoint = executor.snapshot().unwrap();
    let restored = restore_executor(&checkpoint).unwrap();
    assert_eq!(restored.execute(&initial_operation).await.unwrap(), first);
    body.resume = Some(first_resume);
    body.execution_slice_iterations = 3;
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    let resumed: adl_runtime_kernel::ParityBReceipt = serde_json::from_slice(
        &restored
            .execute(&operation("bounded-loop-resume", &body))
            .await
            .unwrap(),
    )
    .unwrap();
    assert!(resumed.iterations <= 3);
    assert!(resumed.resume.is_none());
    assert!(resumed.accepted_sequence > first_receipt.accepted_sequence);
    let mut tampered = checkpoint.clone();
    *tampered.last_mut().unwrap() ^= 1;
    assert!(restore_executor(&tampered).is_err());
}

struct FixedTime(u64);
impl TrustedTime for FixedTime {
    fn now_unix_millis(&self) -> u64 {
        self.0
    }
}

#[tokio::test]
async fn adaptive_learning_consumes_exact_one_shot_mutation_authority() {
    let validated = adl_runtime_kernel::ValidatedReasoningGraph::validate(graph()).unwrap();
    let policy = hash(b"parity-b-policy");
    let key = SigningKey::from_bytes(&[42; 32]);
    let authority = || {
        MutationAuthority::new(BTreeMap::from([(
            "review-key".to_owned(),
            TrustedMutationKey {
                principal: "review-board".to_owned(),
                verifying_key: key.verifying_key(),
            },
        )]))
    };
    let patches = vec![GraphPatch::SetScoreDelta {
        node: "evaluate".to_owned(),
        score_delta: 2,
    }];
    let grant = MutationGrant {
        schema: adl_runtime_kernel::MUTATION_GRANT_SCHEMA.to_owned(),
        grant_id: "one-shot".to_owned(),
        principal: "review-board".to_owned(),
        signing_key_id: "review-key".to_owned(),
        graph_hash: validated.hash().to_owned(),
        policy_hash: policy.clone(),
        provenance: "review-5592".to_owned(),
        patch_hash: graph_patch_hash(&patches).unwrap(),
        allowed_operations: BTreeSet::from([PatchKind::SetScoreDelta]),
        max_patches: 1,
        max_nodes: 8,
        max_edges: 8,
        expires_unix_millis: 1_000,
        signature: String::new(),
    }
    .sign(&key)
    .unwrap();
    let gate = Arc::new(
        MutationGate::new(
            validated.clone(),
            authority(),
            Arc::new(FixedTime(500)),
            policy.clone(),
            4,
            Arc::new(AdaptationStore::new(AdaptationState::new(
                0,
                validated.hash(),
                policy,
            ))),
        )
        .unwrap(),
    );
    let executor = ParityBExecutor::new(
        BTreeMap::from([("policy-review".to_owned(), policy_key().verifying_key())]),
        "checkpoint",
        checkpoint_key(),
        Some(gate.clone()),
    )
    .unwrap();
    let mut body = request();
    body.mutation = Some(adl_runtime_kernel::ParityBMutation {
        grant: grant.clone(),
        patches: patches.clone(),
    });
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    let receipt: adl_runtime_kernel::ParityBReceipt = serde_json::from_slice(
        &executor
            .execute(&operation("mutate-live", &body))
            .await
            .unwrap(),
    )
    .unwrap();
    let evidence = receipt.mutation_evidence.expect("mutation evidence");
    assert!(executor
        .execute(&operation("mutate-replay-new-id", &body))
        .await
        .is_err());
    assert_eq!(
        rollback_candidate(&gate.graph(), &evidence, &authority())
            .unwrap()
            .hash(),
        validated.hash()
    );
}

#[tokio::test]
async fn affect_control_rejects_adversarial_signal_authority() {
    let executor = executor();
    let mut forged_policy = request();
    forged_policy.signals.risk = 100;
    assert!(executor
        .execute(&operation("forged-policy", &forged_policy))
        .await
        .unwrap_err()
        .message
        .contains("authority"));
    let mut body = request();
    body.signals.provenance = adl_runtime_kernel::SignalProvenance::TaskContent;
    body.signals.affect_adjustment = 100;
    let adversarial_operation = operation("affect-adversarial", &body);
    assert!(executor
        .execute(&adversarial_operation)
        .await
        .unwrap_err()
        .message
        .contains("task content"));
    body.signals.provenance = adl_runtime_kernel::SignalProvenance::Policy;
    body.signals
        .asserted_claims
        .insert("consciousness".to_owned());
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    assert!(executor
        .execute(&operation("affect-claim", &body))
        .await
        .unwrap_err()
        .message
        .contains("unsupported"));
}

#[tokio::test]
async fn curiosity_and_theory_of_mind_remain_non_authoritative() {
    let executor = executor();
    let mut body = request();
    body.signals.observable_interaction_only = false;
    assert!(executor
        .execute(&operation("private-state", &body))
        .await
        .unwrap_err()
        .message
        .contains("private state"));
    body.signals.observable_interaction_only = true;
    body.signals.curiosity_steps = 65;
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    assert!(executor
        .execute(&operation("unbounded-curiosity", &body))
        .await
        .is_err());
}

#[tokio::test]
async fn governed_cognition_cannot_bypass_shutdown_or_freedom_gate() {
    let review_executor = executor();
    let mut review = request();
    review.gates.review_required = true;
    adl_runtime_kernel::sign_policy_request(&mut review, "policy-review", &policy_key()).unwrap();
    assert!(review_executor
        .execute(&operation("review-required", &review))
        .await
        .unwrap_err()
        .message
        .contains("human review"));
    assert!(review_executor
        .receipt("review-required")
        .unwrap()
        .is_none());

    let executor = executor();
    let mut body = request();
    body.gates.shutdown_requested = true;
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    assert!(executor
        .execute(&operation("shutdown", &body))
        .await
        .unwrap_err()
        .message
        .contains("shutdown"));
    let shutdown_checkpoint = executor.snapshot().unwrap();
    let restored_shutdown = restore_executor(&shutdown_checkpoint).unwrap();
    assert!(restored_shutdown
        .execute(&operation("restart-bypass", &request()))
        .await
        .unwrap_err()
        .message
        .contains("shutdown"));
    body.gates.shutdown_requested = false;
    body.gates.freedom_allowed = false;
    adl_runtime_kernel::sign_policy_request(&mut body, "policy-review", &policy_key()).unwrap();
    assert!(executor
        .execute(&operation("freedom-denied", &body))
        .await
        .unwrap_err()
        .message
        .contains("shutdown"));
}

#[test]
fn feature_dispositions_require_live_kernel_or_accepted_boundary() {
    let rows = feature_dispositions();
    assert_eq!(rows.len(), 12);
    assert_eq!(
        rows.iter()
            .map(|row| row.feature.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        12
    );
    for row in rows {
        assert!(!row.evidence.trim().is_empty());
        match row.disposition {
            FeatureDispositionKind::LiveRuntimeV3 => {
                assert!([
                    "reasoning_graph",
                    "bounded_loop",
                    "adaptive_learning",
                    "affect_reasoning_control",
                    "governed_cognition",
                    "constructability",
                ]
                .contains(&row.feature.as_str()))
            }
            FeatureDispositionKind::AcceptedBoundary => {
                assert!([
                    "curiosity_discovery",
                    "theory_of_mind",
                    "godel_mechanics",
                    "guild",
                    "economics_context",
                    "adl.skill.v1",
                ]
                .contains(&row.feature.as_str()))
            }
        }
    }
}

fn operation(id: &str, request: &ParityBRequest) -> adl_runtime_kernel::OperationRequest {
    adl_runtime_kernel::OperationRequest {
        schema: adl_runtime_kernel::OPERATION_REQUEST_SCHEMA.to_owned(),
        request_id: id.to_owned(),
        idempotency_key: id.to_owned(),
        principal: "canonical-ingress".to_owned(),
        payload: serde_json::to_vec(request).unwrap(),
        permit: None,
    }
}

#[test]
fn checkpoint_rejects_semantic_tampering_after_valid_encoding() {
    let executor = executor();
    let bytes = executor.snapshot().unwrap();
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    value["state"]["accepted_sequence"] = serde_json::json!(1);
    assert!(matches!(
        restore_executor(&serde_json::to_vec(&value).unwrap()),
        Err(ParityBError::CheckpointIntegrity)
    ));
}
