//! Runtime adapter registration and immutable provider definition bindings.
//! No lifecycle policy or provider-name dispatch belongs in Runtime callers.
use crate::{build_provider_for_id, Provider, ProviderSpec};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    net::{IpAddr, ToSocketAddrs},
    sync::{Arc, RwLock},
};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderBinding {
    pub provider: String,
    pub model: String,
    #[serde(default)]
    pub endpoint: String,
    #[serde(default)]
    pub credential_ref: Option<String>,
    #[serde(default)]
    pub required_capabilities: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdapterCapabilities {
    pub conversation: bool,
    pub agent_to_agent: bool,
    pub tools: bool,
    pub streaming: bool,
    pub credentials: bool,
    pub model_discovery: bool,
    pub model_validation: bool,
    pub token_accounting: bool,
    pub health_checks: bool,
}
impl AdapterCapabilities {
    pub fn names(&self) -> Vec<String> {
        [
            ("conversation", self.conversation),
            ("agent_to_agent", self.agent_to_agent),
            ("tools", self.tools),
            ("streaming", self.streaming),
            ("credentials", self.credentials),
            ("model_discovery", self.model_discovery),
            ("model_validation", self.model_validation),
            ("token_accounting", self.token_accounting),
            ("health_checks", self.health_checks),
        ]
        .into_iter()
        .filter(|(_, on)| *on)
        .map(|(s, _)| s.into())
        .collect()
    }
    pub fn text(credentials: bool) -> Self {
        Self {
            conversation: true,
            agent_to_agent: true,
            tools: false,
            streaming: false,
            credentials,
            model_discovery: false,
            model_validation: false,
            token_accounting: false,
            health_checks: false,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFailure {
    UnknownProvider,
    Credentials,
    Quota,
    UnsupportedCapability,
    ModelUnavailable,
    Transport,
    Timeout,
    InvalidResponse,
    InvalidConfiguration,
    Cancelled,
}
impl ProviderFailure {
    pub fn code(self) -> &'static str {
        match self {
            Self::UnknownProvider => "provider_unknown",
            Self::Credentials => "provider_credentials",
            Self::Quota => "provider_quota",
            Self::UnsupportedCapability => "provider_unsupported_capability",
            Self::ModelUnavailable => "provider_model_unavailable",
            Self::Transport => "provider_transport",
            Self::Timeout => "provider_timeout",
            Self::InvalidResponse => "provider_invalid_response",
            Self::InvalidConfiguration => "provider_invalid_configuration",
            Self::Cancelled => "provider_cancelled",
        }
    }
}
impl fmt::Display for ProviderFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
impl std::error::Error for ProviderFailure {}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderProjection {
    pub provider: String,
    pub adapter: String,
    pub model_ref: String,
    pub provider_model_id: String,
    pub endpoint_class: String,
    pub capabilities: AdapterCapabilities,
    pub definition_generation: u64,
    pub definition_digest: String,
    pub health: String,
}
pub struct PreparedProvider {
    pub projection: ProviderProjection,
    pub executor: Box<dyn Provider>,
}
/// Implementations own authentication, transport and metering capabilities.
/// prepare and verify must never issue a completion request.
pub trait RuntimeProviderAdapter: Send + Sync {
    fn capabilities(&self) -> AdapterCapabilities;
    fn prepare(
        &self,
        id: &str,
        spec: &ProviderSpec,
        binding: &ProviderBinding,
    ) -> Result<PreparedProvider, ProviderFailure>;
    fn verify(&self, _prepared: &PreparedProvider) -> Result<(), ProviderFailure> {
        Ok(())
    }
}
#[derive(Clone)]
struct DefinitionSnapshot {
    generation: u64,
    digest: String,
    providers: HashMap<String, ProviderSpec>,
}
pub struct ProviderRegistry {
    adapters: RwLock<BTreeMap<String, Arc<dyn RuntimeProviderAdapter>>>,
    definitions: RwLock<Arc<DefinitionSnapshot>>,
    projections: RwLock<HashMap<String, ProviderProjection>>,
    budgets: std::sync::Mutex<HashMap<String, Arc<RuntimeBudget>>>,
}
impl fmt::Debug for ProviderRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProviderRegistry").finish_non_exhaustive()
    }
}
impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::standard()
    }
}
impl ProviderRegistry {
    pub fn empty() -> Self {
        Self {
            budgets: std::sync::Mutex::new(HashMap::new()),
            projections: RwLock::new(HashMap::new()),
            adapters: RwLock::new(BTreeMap::new()),
            definitions: RwLock::new(Arc::new(DefinitionSnapshot {
                generation: 0,
                digest: "builtin".into(),
                providers: HashMap::new(),
            })),
        }
    }
    pub fn standard() -> Self {
        let r = Self::empty();
        for (id, kind, hosted) in [
            ("ollama", "ollama", false),
            ("openai-compatible", "http", false),
            ("openai", "openai", true),
            ("anthropic", "anthropic", true),
            ("vertex_ai", "vertex_ai_gemini", true),
            ("vertex_ai_gemini", "vertex_ai_gemini", true),
            ("deepseek", "deepseek", true),
            ("kimi", "kimi", true),
            ("moonshot", "kimi", true),
            ("openrouter", "openrouter", true),
            ("z_ai", "z_ai", true),
            ("zai", "z_ai", true),
            ("zhipu", "z_ai", true),
            ("vertex", "vertex_ai_gemini", true),
            ("bedrock", "bedrock", true),
            ("aws_bedrock", "bedrock", true),
            ("http", "http", false),
            ("http_remote", "http_remote", false),
            ("local_ollama", "local_ollama", false),
            ("mock", "mock", false),
        ] {
            r.register(
                id,
                Arc::new(NativeAdapter {
                    kind: kind.into(),
                    hosted,
                    chat_compatible: id == "openai-compatible",
                }),
            )
            .expect("unique builtins");
        }
        r
    }
    pub fn register(
        &self,
        id: &str,
        adapter: Arc<dyn RuntimeProviderAdapter>,
    ) -> Result<(), ProviderFailure> {
        if id.is_empty()
            || !id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"_-".contains(&c))
        {
            return Err(ProviderFailure::InvalidConfiguration);
        }
        let mut entries = self
            .adapters
            .write()
            .map_err(|_| ProviderFailure::InvalidConfiguration)?;
        if entries.contains_key(id) {
            return Err(ProviderFailure::InvalidConfiguration);
        }
        entries.insert(id.into(), adapter);
        Ok(())
    }
    /// Report a generated response rejected by the Runtime action protocol.
    pub fn record_response_failure(&self, provider: &str) {
        if let Ok(budgets) = self.budgets.lock() {
            if let Some(budget) = budgets.get(provider) {
                budget
                    .failed
                    .store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    pub fn definition_generation(&self) -> u64 {
        self.definitions
            .read()
            .expect("provider snapshot")
            .generation
    }

    pub fn catalog(&self) -> Vec<serde_json::Value> {
        let snapshot = self.definitions.read().expect("provider snapshot").clone();
        let adapters = self.adapters.read().expect("provider adapters");
        if snapshot.generation == 0 {
            return adapters.iter().map(|(id,a)|serde_json::json!({"provider":id,"capabilities":a.capabilities(),"definition_generation":0,"definition_digest":snapshot.digest})).collect();
        }
        let mut ids = snapshot.providers.keys().collect::<Vec<_>>();
        ids.sort();
        ids.into_iter().filter_map(|id|{let spec=&snapshot.providers[id];let adapter=adapters.get(&spec.kind)?;Some(serde_json::json!({"provider":id,"adapter":spec.kind,"model_ref":spec.default_model,"provider_model_id":spec.config.get("provider_model_id"),"capabilities":adapter.capabilities(),"definition_generation":snapshot.generation,"definition_digest":snapshot.digest}))}).collect()
    }
    pub fn replace_definitions(
        &self,
        providers: HashMap<String, ProviderSpec>,
        digest: String,
    ) -> Result<(), ProviderFailure> {
        let adapters = self
            .adapters
            .read()
            .map_err(|_| ProviderFailure::InvalidConfiguration)?;
        for spec in providers.values() {
            if !adapters.contains_key(&spec.kind) {
                return Err(ProviderFailure::UnknownProvider);
            }
        }
        let mut current = self
            .definitions
            .write()
            .map_err(|_| ProviderFailure::InvalidConfiguration)?;
        *current = Arc::new(DefinitionSnapshot {
            generation: current.generation + 1,
            digest,
            providers,
        });
        Ok(())
    }
    pub fn prepare(&self, binding: &ProviderBinding) -> Result<PreparedProvider, ProviderFailure> {
        self.prepare_inner(binding, true)
    }
    pub fn project(
        &self,
        binding: &ProviderBinding,
    ) -> Result<ProviderProjection, ProviderFailure> {
        self.validate_reference(binding)?;
        let generation = self
            .definitions
            .read()
            .map_err(|_| ProviderFailure::InvalidConfiguration)?
            .generation;
        let key =
            serde_json::to_string(binding).map_err(|_| ProviderFailure::InvalidConfiguration)?;
        self.projections
            .read()
            .map_err(|_| ProviderFailure::InvalidConfiguration)?
            .get(&key)
            .filter(|p| p.definition_generation == generation)
            .cloned()
            .ok_or(ProviderFailure::InvalidConfiguration)
    }
    pub fn validate_reference(&self, binding: &ProviderBinding) -> Result<(), ProviderFailure> {
        let snapshot = self
            .definitions
            .read()
            .map_err(|_| ProviderFailure::InvalidConfiguration)?;
        let kind = match snapshot.providers.get(&binding.provider) {
            Some(spec) => spec.kind.as_str(),
            None if snapshot.generation == 0 => binding.provider.as_str(),
            None => return Err(ProviderFailure::UnknownProvider),
        };
        let adapters = self
            .adapters
            .read()
            .map_err(|_| ProviderFailure::InvalidConfiguration)?;
        let caps = adapters
            .get(kind)
            .ok_or(ProviderFailure::UnknownProvider)?
            .capabilities();
        if !caps.conversation
            || binding
                .required_capabilities
                .iter()
                .any(|c| !caps.names().contains(c))
        {
            return Err(ProviderFailure::UnsupportedCapability);
        }
        Ok(())
    }
    fn prepare_inner(
        &self,
        binding: &ProviderBinding,
        verify: bool,
    ) -> Result<PreparedProvider, ProviderFailure> {
        let snapshot = self
            .definitions
            .read()
            .map_err(|_| ProviderFailure::InvalidConfiguration)?
            .clone();
        if snapshot.generation > 0 && !snapshot.providers.contains_key(&binding.provider) {
            return Err(ProviderFailure::UnknownProvider);
        }
        let spec = snapshot
            .providers
            .get(&binding.provider)
            .cloned()
            .unwrap_or_else(|| ProviderSpec {
                id: Some(binding.provider.clone()),
                profile: None,
                kind: binding.provider.clone(),
                base_url: None,
                default_model: None,
                config: HashMap::new(),
            });
        if let Some(native) = spec
            .config
            .get("provider_model_id")
            .or_else(|| spec.config.get("model"))
            .and_then(|v| v.as_str())
        {
            let stable = spec
                .config
                .get("model_ref")
                .and_then(|v| v.as_str())
                .or(spec.default_model.as_deref());
            if binding.model != native && stable != Some(binding.model.as_str()) {
                return Err(ProviderFailure::ModelUnavailable);
            }
        }
        let adapter = self
            .adapters
            .read()
            .map_err(|_| ProviderFailure::InvalidConfiguration)?
            .get(&spec.kind)
            .cloned()
            .ok_or(ProviderFailure::UnknownProvider)?;
        let capabilities = adapter.capabilities();
        let supported = capabilities.names();
        if !capabilities.conversation
            || binding
                .required_capabilities
                .iter()
                .any(|c| !supported.contains(c))
        {
            return Err(ProviderFailure::UnsupportedCapability);
        }
        let mut prepared = adapter.prepare(&binding.provider, &spec, binding)?;
        if let Some(RuntimeBudgetLimits {
            max_calls,
            input_bytes,
            stop_after_failure,
        }) = runtime_budget_limits(&spec.config)?
        {
            let budget = self
                .budgets
                .lock()
                .map_err(|_| ProviderFailure::InvalidConfiguration)?
                .entry(binding.provider.clone())
                .or_default()
                .clone();
            prepared.executor = Box::new(BudgetedProvider {
                inner: prepared.executor,
                budget,
                max_calls,
                input_bytes,
                stop_after_failure,
            });
        }
        prepared.projection.definition_generation = snapshot.generation;
        prepared.projection.definition_digest = snapshot.digest.clone();
        prepared.projection.capabilities = capabilities;
        if verify {
            adapter.verify(&prepared)?;
            if prepared.projection.capabilities.model_validation {
                prepared.projection.health = "metadata_verified_inference_unverified".into();
            }
        }
        let key =
            serde_json::to_string(binding).map_err(|_| ProviderFailure::InvalidConfiguration)?;
        let mut projections = self
            .projections
            .write()
            .map_err(|_| ProviderFailure::InvalidConfiguration)?;
        if projections.len() >= 10_000 {
            projections.clear();
        }
        projections.insert(key, prepared.projection.clone());
        Ok(prepared)
    }
}

pub(crate) struct RuntimeBudgetLimits {
    pub(crate) max_calls: u64,
    pub(crate) input_bytes: u64,
    pub(crate) stop_after_failure: bool,
}

pub(crate) fn runtime_budget_limits(
    config: &HashMap<String, serde_json::Value>,
) -> Result<Option<RuntimeBudgetLimits>, ProviderFailure> {
    if ![
        "runtime_max_calls",
        "runtime_max_input_bytes",
        "runtime_stop_after_failure",
    ]
    .iter()
    .any(|key| config.contains_key(*key))
    {
        return Ok(None);
    }
    let max_calls = config
        .get("runtime_max_calls")
        .and_then(|v| v.as_u64())
        .filter(|v| (1..=1024).contains(v))
        .ok_or(ProviderFailure::InvalidConfiguration)?;
    let input_bytes = config
        .get("runtime_max_input_bytes")
        .and_then(|v| v.as_u64())
        .filter(|v| (1..=1_000_000).contains(v))
        .ok_or(ProviderFailure::InvalidConfiguration)?;
    let stop_after_failure = match config.get("runtime_stop_after_failure") {
        None => false,
        Some(v) => v.as_bool().ok_or(ProviderFailure::InvalidConfiguration)?,
    };
    Ok(Some(RuntimeBudgetLimits {
        max_calls,
        input_bytes,
        stop_after_failure,
    }))
}

#[derive(Default)]
struct RuntimeBudget {
    dispatch: std::sync::Mutex<()>,
    calls: std::sync::atomic::AtomicU64,
    failed: std::sync::atomic::AtomicBool,
}
struct BudgetedProvider {
    inner: Box<dyn Provider>,
    budget: Arc<RuntimeBudget>,
    max_calls: u64,
    input_bytes: u64,
    stop_after_failure: bool,
}
impl Provider for BudgetedProvider {
    fn verify_model_metadata(&self) -> anyhow::Result<bool> {
        self.inner.verify_model_metadata()
    }
    fn complete(&self, prompt: &str) -> anyhow::Result<String> {
        use std::sync::atomic::Ordering;
        // The optional demo envelope allows one active call; never queue a new
        // dispatch inside blocking work after its caller may have cancelled.
        let _dispatch = self
            .budget
            .dispatch
            .try_lock()
            .map_err(|_| ProviderFailure::Quota)?;
        if prompt.len() as u64 > self.input_bytes {
            return Err(ProviderFailure::InvalidConfiguration.into());
        }
        if self.stop_after_failure && self.budget.failed.load(Ordering::SeqCst) {
            return Err(ProviderFailure::Quota.into());
        }
        self.budget
            .calls
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| {
                (n < self.max_calls).then_some(n + 1)
            })
            .map_err(|_| ProviderFailure::Quota)?;
        let result = self.inner.complete(prompt).and_then(|output| {
            if output.trim().is_empty() || output.len() > 4_194_304 {
                Err(ProviderFailure::InvalidResponse.into())
            } else {
                Ok(output)
            }
        });
        if result.is_err() {
            self.budget.failed.store(true, Ordering::SeqCst);
        }
        result
    }
}

pub fn credential_env(reference: &str) -> Result<&str, ProviderFailure> {
    let name = reference
        .strip_prefix("env:")
        .ok_or(ProviderFailure::Credentials)?;
    if name.is_empty()
        || name.len() > 128
        || !name
            .bytes()
            .enumerate()
            .all(|(i, c)| c == b'_' || c.is_ascii_uppercase() || (i > 0 && c.is_ascii_digit()))
    {
        return Err(ProviderFailure::Credentials);
    }
    Ok(name)
}
fn private(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => ip.is_loopback() || ip.is_private(),
        IpAddr::V6(ip) => ip.is_loopback() || ip.is_unique_local(),
    }
}
// A fixed two-worker DNS pool bounds resolver work even if the platform DNS
// resolver stalls. A timed-out request does not create replacement workers.
fn bounded_resolve(host: &str, port: u16) -> Result<Vec<std::net::SocketAddr>, ProviderFailure> {
    if let Ok(ip) = host.parse::<IpAddr>() {
        return Ok(vec![std::net::SocketAddr::new(ip, port)]);
    }
    if host == "localhost" {
        return Ok(vec![std::net::SocketAddr::new(
            IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
            port,
        )]);
    }
    type Query = (
        String,
        u16,
        std::sync::mpsc::SyncSender<Vec<std::net::SocketAddr>>,
    );
    static DNS: std::sync::OnceLock<std::sync::mpsc::SyncSender<Query>> =
        std::sync::OnceLock::new();
    let queue = DNS.get_or_init(|| {
        let (tx, rx) = std::sync::mpsc::sync_channel::<Query>(8);
        let rx = Arc::new(std::sync::Mutex::new(rx));
        for _ in 0..2 {
            let rx = rx.clone();
            std::thread::spawn(move || loop {
                let query = rx.lock().expect("DNS queue").recv();
                let Ok((host, port, result)) = query else {
                    break;
                };
                let addresses = (host.as_str(), port)
                    .to_socket_addrs()
                    .map(|a| a.collect())
                    .unwrap_or_default();
                let _ = result.try_send(addresses);
            });
        }
        tx
    });
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    queue
        .try_send((host.into(), port, tx))
        .map_err(|_| ProviderFailure::Transport)?;
    rx.recv_timeout(std::time::Duration::from_secs(2))
        .map_err(|_| ProviderFailure::Timeout)
}
/// Local DNS is pinned to a checked numeric address before client construction.
fn safe_endpoint(endpoint: &str, hosted: bool) -> Result<(String, String), ProviderFailure> {
    let mut url =
        reqwest::Url::parse(endpoint).map_err(|_| ProviderFailure::InvalidConfiguration)?;
    if !url.username().is_empty() || url.password().is_some() || url.fragment().is_some() {
        return Err(ProviderFailure::InvalidConfiguration);
    }
    if url.scheme() == "https" {
        return Ok((endpoint.into(), "https".into()));
    }
    if hosted || url.scheme() != "http" {
        return Err(ProviderFailure::InvalidConfiguration);
    }
    let host = url
        .host_str()
        .ok_or(ProviderFailure::InvalidConfiguration)?
        .trim_matches(['[', ']']);
    let port = url
        .port_or_known_default()
        .ok_or(ProviderFailure::InvalidConfiguration)?;
    let addresses = bounded_resolve(host, port)?;
    if addresses.is_empty() || addresses.iter().any(|a| !private(a.ip())) {
        return Err(ProviderFailure::InvalidConfiguration);
    }
    url.set_ip_host(addresses[0].ip())
        .map_err(|_| ProviderFailure::InvalidConfiguration)?;
    Ok((url.into(), "private_http".into()))
}
struct NativeAdapter {
    kind: String,
    hosted: bool,
    chat_compatible: bool,
}
impl RuntimeProviderAdapter for NativeAdapter {
    fn capabilities(&self) -> AdapterCapabilities {
        let mut capabilities = AdapterCapabilities::text(
            self.hosted
                || self.chat_compatible
                || matches!(self.kind.as_str(), "http" | "http_remote"),
        );
        capabilities.model_validation = self.kind == "ollama";
        // Metadata validates one binding; enumeration is not a registry API yet.
        capabilities.model_discovery = false;
        capabilities
    }
    fn verify(&self, prepared: &PreparedProvider) -> Result<(), ProviderFailure> {
        if self.kind == "ollama"
            && !prepared
                .executor
                .verify_model_metadata()
                .map_err(map_adapter_failure)?
        {
            return Err(ProviderFailure::UnsupportedCapability);
        }
        Ok(())
    }
    fn prepare(
        &self,
        id: &str,
        spec: &ProviderSpec,
        binding: &ProviderBinding,
    ) -> Result<PreparedProvider, ProviderFailure> {
        let mut spec = spec.clone();
        spec.kind = self.kind.clone();
        if self.chat_compatible {
            spec.config
                .insert("api_format".into(), "openai_chat_completions".into());
        }
        // Bound transport even if the caller stops awaiting the blocking call.
        spec.config.insert("timeout_secs".into(), 30.into());
        spec.config.insert("runtime_max_attempts".into(), 1.into());
        if let Some(reference) = &binding.credential_ref {
            let name = credential_env(reference)?;
            spec.config.insert("auth_env".into(), name.into());
            spec.config.insert(
                "auth".into(),
                serde_json::json!({"type":"bearer","env":name}),
            );
        }
        let endpoint = if binding.endpoint.is_empty() {
            spec.config
                .get("endpoint")
                .and_then(|v| v.as_str())
                .map(str::to_owned)
                .or_else(|| spec.base_url.clone())
        } else {
            Some(binding.endpoint.clone())
        };
        let endpoint_class = if let Some(endpoint) = endpoint {
            let (mut safe, class) = safe_endpoint(&endpoint, self.hosted)?;
            if self.chat_compatible {
                let mut url = reqwest::Url::parse(&safe)
                    .map_err(|_| ProviderFailure::InvalidConfiguration)?;
                match url.path().trim_end_matches('/') {
                    "" => url.set_path("/v1/chat/completions"),
                    "/v1" => url.set_path("/v1/chat/completions"),
                    _ => {}
                }
                safe = url.into();
            }
            spec.base_url = Some(safe.clone());
            spec.config.insert("endpoint".into(), safe.into());
            class
        } else if self.kind == "ollama" || self.chat_compatible {
            // Runtime Ollama is the HTTP adapter; absence must not silently launch a CLI.
            return Err(ProviderFailure::InvalidConfiguration);
        } else if self.hosted {
            "https".into()
        } else {
            "local_process".into()
        };
        if self.hosted && self.kind != "bedrock" && self.kind != "vertex_ai_gemini" {
            let name = spec
                .config
                .get("auth")
                .and_then(|a| a.get("env"))
                .and_then(|v| v.as_str())
                .ok_or(ProviderFailure::Credentials)?;
            credential_env(&format!("env:{name}"))?;
        }
        if binding.model.is_empty()
            || binding.model.len() > 256
            || binding.model.chars().any(char::is_control)
        {
            return Err(ProviderFailure::ModelUnavailable);
        }
        crate::candidate::reject_credential_values(&HashMap::from([(id.to_owned(), spec.clone())]))
            .map_err(|_| ProviderFailure::InvalidConfiguration)?;
        let target = crate::provider_substrate::provider_invocation_target_v1(
            id,
            &spec,
            Some(&binding.model),
        )
        .map_err(|_| ProviderFailure::InvalidConfiguration)?;
        let executor =
            build_provider_for_id(id, &spec, Some(&binding.model)).map_err(map_adapter_failure)?;
        Ok(PreparedProvider {
            projection: ProviderProjection {
                provider: id.into(),
                adapter: self.kind.clone(),
                model_ref: binding.model.clone(),
                provider_model_id: target.provider_model_id,
                endpoint_class,
                capabilities: self.capabilities(),
                definition_generation: 0,
                definition_digest: String::new(),
                health: "configuration_validated_inference_unverified".into(),
            },
            executor,
        })
    }
}
fn map_adapter_failure(error: anyhow::Error) -> ProviderFailure {
    match crate::failure_category(&error) {
        "credentials" => ProviderFailure::Credentials,
        "quota" => ProviderFailure::Quota,
        "unsupported_capability" => ProviderFailure::UnsupportedCapability,
        "model_unavailable" => ProviderFailure::ModelUnavailable,
        "timeout" => ProviderFailure::Timeout,
        "invalid_response" => ProviderFailure::InvalidResponse,
        "invalid_configuration" => ProviderFailure::InvalidConfiguration,
        _ => ProviderFailure::Transport,
    }
}
