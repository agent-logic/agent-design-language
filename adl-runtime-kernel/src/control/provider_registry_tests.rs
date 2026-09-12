//! PVF provider lane: deterministic local lifecycle proof, no network or paid
//! inference, release-required for issue855. Real vendor transports are a
//! separate provider-core matrix, and hosted demonstration is separate again.
use super::*;
use adl_provider_core::registry::{
    AdapterCapabilities, PreparedProvider, ProviderBinding, ProviderFailure, ProviderProjection,
    RuntimeProviderAdapter,
};
use std::sync::atomic::{AtomicUsize, Ordering};
struct Lifecycle;
#[async_trait]
impl LifecycleControl for Lifecycle {
    async fn shutdown(&self, _: Duration) -> Result<KernelExit, ()> {
        Ok(KernelExit::Clean)
    }
}
struct Fixture(Arc<AtomicUsize>);
impl adl_provider_core::Provider for Fixture {
    fn complete(&self, prompt: &str) -> anyhow::Result<String> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(format!("generated sixth-provider response: {prompt}"))
    }
}
impl RuntimeProviderAdapter for Fixture {
    fn capabilities(&self) -> AdapterCapabilities {
        AdapterCapabilities::text(false)
    }
    fn prepare(
        &self,
        id: &str,
        _: &adl_provider_core::ProviderSpec,
        binding: &ProviderBinding,
    ) -> Result<PreparedProvider, ProviderFailure> {
        Ok(PreparedProvider {
            executor: Box::new(Fixture(self.0.clone())),
            projection: ProviderProjection {
                provider: id.into(),
                adapter: "sixth".into(),
                model_ref: binding.model.clone(),
                provider_model_id: binding.model.clone(),
                endpoint_class: "fixture".into(),
                capabilities: self.capabilities(),
                definition_generation: 0,
                definition_digest: String::new(),
                health: "fixture".into(),
            },
        })
    }
}
fn service(path: PathBuf, calls: Arc<AtomicUsize>) -> ControlService<Lifecycle> {
    let recorder = RuntimeRecorder::new(16);
    recorder
        .providers
        .register("sixth", Arc::new(Fixture(calls)))
        .unwrap();
    let service = ControlService::new_with_observatory_config_and_agents(
        "registry-runtime",
        recorder,
        Lifecycle,
        ControlAuthority::new(BTreeMap::new()),
        16,
        std::iter::empty(),
        AgentPopulationFeed::resident_shepherd(),
    );
    service.configure_dynamic_agent_store(path).unwrap();
    service
}
fn binding() -> AgentAdmissionRequest {
    AgentAdmissionRequest {
        schema: AGENT_ADMISSION_SCHEMA.into(),
        id: "fixture-agent".into(),
        name: "ember.fixture".into(),
        display_name: "Ember Fixture".into(),
        office: "assistant".into(),
        role: String::new(),
        provider: "sixth".into(),
        model: "fixture-model".into(),
        endpoint: String::new(),
        credential_ref: None,
        required_capabilities: vec!["conversation".into(), "agent_to_agent".into()],
    }
}
#[tokio::test]
async fn sixth_adapter_lifecycle_no_vendor_dispatch_or_restart() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(".adl/issue855");
    fs::create_dir_all(&root).unwrap();
    let temp = tempfile::tempdir_in(&root).unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let svc = service(temp.path().join("admissions.json"), calls.clone());
    let incarnation = svc.runtime_incarnation_id.clone();
    let mut request = binding();
    assert_eq!(
        svc.admit_agent(request.clone()).await.unwrap().status,
        "admitted"
    );
    for _ in 0..3 {
        svc.refresh_dynamic_agent_health().await;
    }
    assert_eq!(
        calls.load(Ordering::SeqCst),
        0,
        "admission/health never completes"
    );
    for prompt in [
        "operator to ember.fixture",
        "ember.fixture to beacon.fixture",
    ] {
        let reply = crate::provider_registry::complete(
            svc.recorder.providers.clone(),
            provider_binding(&request),
            prompt.into(),
            &CancellationToken::new(),
        )
        .await
        .unwrap();
        assert!(reply.contains(prompt));
    }
    let checkpoint = svc.checkpoint_agent(&request.id).unwrap();
    assert_eq!(checkpoint.declaration.name, "ember.fixture");
    assert_eq!(
        checkpoint.checkpoint_digest,
        agent_checkpoint_digest(&checkpoint).unwrap()
    );
    let detail = svc.agent_roster_detail(&request.id).unwrap();
    assert_eq!(detail.provider_binding.as_ref().unwrap().adapter, "sixth");
    assert!(detail.capabilities.contains(&"agent_to_agent".to_owned()));
    request.model = "replacement-model".into();
    assert_eq!(
        svc.admit_agent(request.clone()).await.unwrap().status,
        "replaced"
    );
    let bundle = svc.dehydrate_agent(&request.id).unwrap();
    assert!(svc.commit_agent_migration(&request.id, "tampered").is_err());
    assert_eq!(
        svc.commit_agent_migration(&request.id, &bundle.bundle_digest)
            .unwrap(),
        "removed"
    );
    let mut tampered = bundle.clone();
    tampered.declaration.model = "tampered".into();
    assert!(svc.rehydrate_agent(tampered).await.is_err());
    assert_eq!(
        svc.rehydrate_agent(bundle).await.unwrap().status,
        "admitted"
    );
    let reply = crate::provider_registry::complete(
        svc.recorder.providers.clone(),
        provider_binding(&request),
        "after restore".into(),
        &CancellationToken::new(),
    )
    .await
    .unwrap();
    assert!(reply.contains("after restore"));
    assert_eq!(calls.load(Ordering::SeqCst), 3);
    assert_eq!(svc.runtime_incarnation_id, incarnation);
    assert_eq!(svc.remove_agent(&request.id).unwrap(), "removed");
}
#[tokio::test]
async fn registry_admission_negatives_have_no_state_or_completion_side_effects() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(".adl/issue855");
    fs::create_dir_all(&root).unwrap();
    let temp = tempfile::tempdir_in(root).unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let svc = service(temp.path().join("admissions.json"), calls.clone());
    let mut request = binding();
    request.provider = "not-registered".into();
    assert_eq!(
        agent_admission_failure_reason(&svc.admit_agent(request).await.unwrap_err()),
        "provider_unknown"
    );
    let mut request = binding();
    request.required_capabilities.push("streaming".into());
    assert_eq!(
        agent_admission_failure_reason(&svc.admit_agent(request).await.unwrap_err()),
        "provider_unsupported_capability"
    );
    let mut request = binding();
    request.credential_ref = Some("secret-value".into());
    assert!(svc.admit_agent(request).await.is_err());
    assert!(svc.dynamic_agents.lock().unwrap().is_empty());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn named_definition_removal_and_fixed_model_mismatch_fail_closed() {
    let registry = adl_provider_core::registry::ProviderRegistry::empty();
    registry
        .register("sixth", Arc::new(Fixture(Arc::new(AtomicUsize::new(0)))))
        .unwrap();
    let spec = adl_provider_core::ProviderSpec {
        id: Some("sixth".into()),
        profile: None,
        kind: "sixth".into(),
        base_url: None,
        default_model: Some("stable-model".into()),
        config: std::collections::HashMap::from([(
            "provider_model_id".into(),
            "native-model".into(),
        )]),
    };
    registry
        .replace_definitions(
            std::collections::HashMap::from([("sixth".into(), spec)]),
            "generation-one".into(),
        )
        .unwrap();
    let mut b = provider_binding(&binding());
    b.model = "wrong-model".into();
    assert!(matches!(
        registry.prepare(&b),
        Err(ProviderFailure::ModelUnavailable)
    ));
    b.model = "stable-model".into();
    let pinned = registry.prepare(&b).unwrap();
    registry
        .replace_definitions(std::collections::HashMap::new(), "generation-two".into())
        .unwrap();
    assert!(matches!(
        registry.prepare(&b),
        Err(ProviderFailure::UnknownProvider)
    ));
    assert!(matches!(
        registry.project(&b),
        Err(ProviderFailure::UnknownProvider)
    ));
    assert_eq!(pinned.projection.definition_digest, "generation-one");
    assert!(pinned
        .executor
        .complete("old pinned call")
        .unwrap()
        .contains("old pinned call"));
}
#[tokio::test]
async fn metadata_refresh_does_not_erase_failed_inference() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(".adl/issue855");
    fs::create_dir_all(&root).unwrap();
    let temp = tempfile::tempdir_in(root).unwrap();
    let svc = service(
        temp.path().join("admissions.json"),
        Arc::new(AtomicUsize::new(0)),
    );
    let request = binding();
    svc.admit_agent(request.clone()).await.unwrap();
    let usage = svc.recorder.provider_usage.begin(
        &request.id,
        &request.provider,
        &request.model,
        crate::provider_usage::ProviderRequestReason::OperatorConversation,
        "test",
    );
    usage.failure("provider_credentials");
    svc.refresh_dynamic_agent_health().await;
    let population = svc.agent_population.read().unwrap();
    let sample = population
        .sample
        .iter()
        .find(|a| a.id == request.id)
        .unwrap();
    assert_eq!(sample.inference_readiness, InferenceReadinessState::Failed);
    assert!(sample
        .detail
        .contains("inference_failure_requires_generated_recovery"));
}
#[tokio::test]
async fn registry_cancelled_call_does_not_execute_or_invalidate_success() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(".adl/issue855");
    fs::create_dir_all(&root).unwrap();
    let temp = tempfile::tempdir_in(root).unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let svc = service(temp.path().join("admissions.json"), calls.clone());
    let request = binding();
    let prior = svc.recorder.provider_usage.begin(
        &request.id,
        &request.provider,
        &request.model,
        crate::provider_usage::ProviderRequestReason::OperatorConversation,
        "test",
    );
    prior.success("ready");
    let cancel = CancellationToken::new();
    cancel.cancel();
    let err = crate::provider_registry::complete(
        svc.recorder.providers.clone(),
        provider_binding(&request),
        "must not dispatch".into(),
        &cancel,
    )
    .await
    .unwrap_err();
    assert_eq!(err, ProviderFailure::Cancelled);
    let usage = svc.recorder.provider_usage.begin(
        &request.id,
        &request.provider,
        &request.model,
        crate::provider_usage::ProviderRequestReason::OperatorConversation,
        "cancelled",
    );
    usage.failure(err.code());
    assert_eq!(
        svc.recorder.provider_usage.health_snapshot()[0].inference_ready,
        Some(true)
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[test]
fn provider_demo_budget_caps_full_prompt_calls_and_failure_retries() {
    struct FailingFixture(Arc<AtomicUsize>);
    impl adl_provider_core::Provider for FailingFixture {
        fn complete(&self, prompt: &str) -> anyhow::Result<String> {
            self.0.fetch_add(1, Ordering::SeqCst);
            if prompt == "fail" {
                return Err(ProviderFailure::Credentials.into());
            }
            if prompt == "empty" {
                return Ok(String::new());
            }
            Ok("generated".into())
        }
    }
    struct Adapter(Arc<AtomicUsize>);
    impl RuntimeProviderAdapter for Adapter {
        fn capabilities(&self) -> AdapterCapabilities {
            AdapterCapabilities::text(false)
        }
        fn prepare(
            &self,
            id: &str,
            spec: &adl_provider_core::ProviderSpec,
            b: &ProviderBinding,
        ) -> Result<PreparedProvider, ProviderFailure> {
            let mut p = Fixture(self.0.clone()).prepare(id, spec, b)?;
            p.executor = Box::new(FailingFixture(self.0.clone()));
            Ok(p)
        }
    }
    let calls = Arc::new(AtomicUsize::new(0));
    let registry = adl_provider_core::registry::ProviderRegistry::empty();
    registry
        .register("fixture", Arc::new(Adapter(calls.clone())))
        .unwrap();
    let make = |max_calls| adl_provider_core::ProviderSpec {
        id: None,
        profile: None,
        kind: "fixture".into(),
        base_url: None,
        default_model: None,
        config: std::collections::HashMap::from([
            ("runtime_max_calls".into(), serde_json::json!(max_calls)),
            ("runtime_max_input_bytes".into(), 10.into()),
            ("runtime_stop_after_failure".into(), true.into()),
        ]),
    };
    for malformed in [
        serde_json::json!({"runtime_max_calls":"six", "runtime_max_input_bytes":10}),
        serde_json::json!({"runtime_max_calls":6, "runtime_max_input_bytes":10, "runtime_stop_after_failure":"true"}),
        serde_json::json!({"runtime_max_input_bytes":10}),
        serde_json::json!({"runtime_stop_after_failure":true}),
        serde_json::json!({"runtime_max_calls":6}),
    ] {
        let mut spec = make(6);
        spec.config = serde_json::from_value(malformed).unwrap();
        registry
            .replace_definitions(
                std::collections::HashMap::from([("malformed".into(), spec)]),
                "malformed-budget".into(),
            )
            .unwrap();
        let mut b = provider_binding(&binding());
        b.provider = "malformed".into();
        assert!(
            matches!(
                registry.prepare(&b),
                Err(ProviderFailure::InvalidConfiguration)
            ),
            "present malformed or incomplete budget must fail closed"
        );
    }
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    registry
        .replace_definitions(
            std::collections::HashMap::from([
                ("bounded".into(), make(2)),
                ("stopped".into(), make(6)),
                ("invalid-output".into(), make(6)),
            ]),
            "bounded-fixture".into(),
        )
        .unwrap();
    let mut b = provider_binding(&binding());
    b.provider = "bounded".into();
    assert!(registry
        .prepare(&b)
        .unwrap()
        .executor
        .complete("this full prompt exceeds ten bytes")
        .is_err());
    assert_eq!(
        calls.load(Ordering::SeqCst),
        0,
        "over-limit full prompt rejected before dispatch"
    );
    for _ in 0..2 {
        assert_eq!(
            registry
                .prepare(&b)
                .unwrap()
                .executor
                .complete("short")
                .unwrap(),
            "generated"
        );
    }
    assert!(registry
        .prepare(&b)
        .unwrap()
        .executor
        .complete("short")
        .is_err());
    assert_eq!(
        calls.load(Ordering::SeqCst),
        2,
        "fresh bindings cannot reset process-lifetime call cap"
    );
    b.provider = "stopped".into();
    assert!(registry
        .prepare(&b)
        .unwrap()
        .executor
        .complete("fail")
        .is_err());
    for _ in 0..3 {
        assert!(registry
            .prepare(&b)
            .unwrap()
            .executor
            .complete("retry")
            .is_err());
    }
    assert_eq!(
        calls.load(Ordering::SeqCst),
        3,
        "scheduler retries cannot dispatch after first failed demo call"
    );
    b.provider = "invalid-output".into();
    assert!(registry
        .prepare(&b)
        .unwrap()
        .executor
        .complete("empty")
        .is_err());
    assert!(registry
        .prepare(&b)
        .unwrap()
        .executor
        .complete("retry")
        .is_err());
    assert_eq!(
        calls.load(Ordering::SeqCst),
        4,
        "invalid output stops subsequent dispatch too"
    );
}

// PVF: deterministic definition/admission contract, no completion or network;
// release-required preservation of accepted #876 trusted-local HTTP definitions.
#[tokio::test]
async fn named_local_http_definition_admits_without_hosted_vendor_policy() {
    let candidate = adl_provider_core::candidate::parse_validated_provider_sidecar(
        r#"
version: 1
providers:
  local-chat:
    type: http
    base_url: http://127.0.0.1:11434/v1/chat/completions
    default_model: local-model
    config:
      api_format: openai_chat_completions
"#,
    )
    .unwrap();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(".adl/issue855");
    fs::create_dir_all(&root).unwrap();
    let temp = tempfile::tempdir_in(root).unwrap();
    let svc = service(
        temp.path().join("admissions.json"),
        Arc::new(AtomicUsize::new(0)),
    );
    // Use the production standard adapters, then the same validated sidecar map.
    svc.recorder
        .providers
        .replace_definitions(candidate.providers, candidate.digest)
        .unwrap();
    let mut request = binding();
    request.provider = "local-chat".into();
    request.model = "local-model".into();
    assert_eq!(
        svc.admit_agent(request.clone()).await.unwrap().status,
        "admitted"
    );
    let projection = svc
        .recorder
        .providers
        .project(&provider_binding(&request))
        .unwrap();
    assert_eq!(projection.adapter, "http");
    assert_eq!(projection.endpoint_class, "private_http");
    assert!(projection.capabilities.conversation);
    assert!(projection.capabilities.credentials);
    drop(svc);
}
