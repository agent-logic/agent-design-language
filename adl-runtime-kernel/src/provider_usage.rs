//! Runtime-owned, process-lifetime provider accounting. Token values are byte
//! estimates, not billing receipts. No prompt, response, endpoint or credential
//! is retained. Snapshot reads never contact a provider.
use serde::Serialize;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderRequestReason {
    OperatorConversation,
    AgentToAgent,
    StartupProbe,
    RecoveryProbe,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProviderUsageCounter {
    pub agent: String,
    pub provider: String,
    pub model: String,
    pub reason: ProviderRequestReason,
    pub requests: u64,
    pub succeeded: u64,
    pub failed: u64,
    pub estimated_input_tokens: u64,
    pub estimated_output_tokens: u64,
    pub provider_reported_input_tokens: u64,
    pub provider_reported_output_tokens: u64,
    pub provider_reported_total_tokens: u64,
    pub provider_reported_responses: u64,
    pub last_finish_reason: Option<String>,
    pub token_accounting: &'static str,
}

type Key = (String, String, String, ProviderRequestReason);
type HealthKey = (String, String, String);
#[derive(Clone, Debug, Default)]
pub struct ProviderUsage(
    Arc<Mutex<BTreeMap<Key, ProviderUsageCounter>>>,
    Arc<Mutex<BTreeMap<HealthKey, ProviderHealthSignals>>>,
    pub(crate) crate::ResidentShepherdReadiness,
    Arc<Mutex<BTreeMap<String, String>>>,
    Arc<Mutex<BTreeMap<String, u64>>>,
);

/// Last observed evidence, not an active liveness check. Unknown stays null.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ProviderHealthSignals {
    pub agent: String,
    pub provider: String,
    pub model: String,
    pub provider_reachable: Option<bool>,
    pub model_available: Option<bool>,
    pub inference_ready: Option<bool>,
    pub inference_observed_at_unix_millis: Option<u64>,
    pub last_successful_inference_at_unix_millis: Option<u64>,
}

impl ProviderUsage {
    fn canonical_agent(&self, agent: &str) -> String {
        self.3
            .lock()
            .expect("provider alias lock poisoned")
            .get(agent)
            .cloned()
            .unwrap_or_else(|| agent.to_owned())
    }

    pub fn register_resident_alias(&self, alias: &str, name: &str) {
        self.2.register_alias(alias, name);
        self.3
            .lock()
            .expect("provider alias lock poisoned")
            .insert(alias.to_owned(), name.to_owned());
    }

    pub(crate) fn readiness(&self) -> crate::ResidentShepherdReadiness {
        self.2.clone()
    }

    /// Fence observations from requests that predate a live binding change.
    pub fn binding_epoch(&self, agent: &str) -> u64 {
        let agent = self.canonical_agent(agent);
        self.4
            .lock()
            .expect("provider epochs poisoned")
            .get(&agent)
            .copied()
            .unwrap_or(0)
    }

    pub fn invalidate_binding(&self, agent: &str) {
        let agent = self.canonical_agent(agent);
        let mut epochs = self.4.lock().expect("provider epochs poisoned");
        let epoch = epochs.entry(agent.clone()).or_default();
        *epoch = epoch.saturating_add(1);
        self.1
            .lock()
            .expect("provider health poisoned")
            .retain(|(name, _, _), _| name != &agent);
        self.2.mark_unready(&agent);
    }

    pub fn health_snapshot(&self) -> Vec<ProviderHealthSignals> {
        self.1
            .lock()
            .expect("provider health lock poisoned")
            .values()
            .cloned()
            .collect()
    }

    pub fn observe_metadata(
        &self,
        agent: &str,
        provider: &str,
        model: &str,
        result: Result<(), &'static str>,
    ) {
        let agent = self.canonical_agent(agent);
        let mut health = self.1.lock().expect("provider health lock poisoned");
        let signals = health
            .entry((agent.clone(), provider.to_owned(), model.to_owned()))
            .or_insert_with(|| ProviderHealthSignals {
                agent: agent.to_owned(),
                provider: provider.to_owned(),
                model: model.to_owned(),
                ..Default::default()
            });
        match result {
            Ok(()) if provider == "ollama" => {
                signals.provider_reachable = Some(true);
                signals.model_available = Some(true);
            }
            Err("model_not_installed") => {
                signals.provider_reachable = Some(true);
                signals.model_available = Some(false);
                signals.inference_ready = Some(false);
            }
            Err("provider_unreachable") => {
                signals.provider_reachable = Some(false);
                signals.model_available = None;
                signals.inference_ready = Some(false);
            }
            Err(_) => {
                signals.inference_ready = Some(false);
            }
            // OpenAI-compatible has no universally supported model-list API.
            // Its no-op metadata phase establishes neither reachability nor model presence.
            Ok(()) => {}
        }
    }

    fn observe_inference(&self, key: &Key, success: bool) {
        let mut health = self.1.lock().expect("provider health lock poisoned");
        let signals = health
            .entry((key.0.clone(), key.1.clone(), key.2.clone()))
            .or_insert_with(|| ProviderHealthSignals {
                agent: key.0.clone(),
                provider: key.1.clone(),
                model: key.2.clone(),
                ..Default::default()
            });
        signals.inference_ready = Some(success);
        signals.inference_observed_at_unix_millis = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        );
        if success {
            signals.last_successful_inference_at_unix_millis =
                signals.inference_observed_at_unix_millis;
            signals.provider_reachable = Some(true);
            signals.model_available = Some(true);
        }
    }

    /// The outer execution deadline can cancel a provider future before it
    /// reports its own timeout. Preserve that failure without counting a second
    /// provider attempt. Operator cancellation must not call this method.
    pub(crate) fn observe_execution_timeout(
        &self,
        agent: &str,
        provider: &str,
        model: &str,
        epoch: u64,
    ) {
        let agent = self.canonical_agent(agent);
        let epochs = self.4.lock().expect("provider epochs poisoned");
        if epochs.get(&agent).copied().unwrap_or(0) != epoch {
            return;
        }
        self.observe_inference(
            &(
                agent.clone(),
                provider.to_owned(),
                model.to_owned(),
                ProviderRequestReason::OperatorConversation,
            ),
            false,
        );
        self.2.mark_unready(&agent);
    }

    pub fn snapshot(&self) -> Vec<ProviderUsageCounter> {
        self.0
            .lock()
            .expect("provider usage lock poisoned")
            .values()
            .cloned()
            .collect()
    }

    /// Count before dispatch so failed or cancelled requests remain visible.
    pub fn begin(
        &self,
        agent: &str,
        provider: &str,
        model: &str,
        reason: ProviderRequestReason,
        prompt: &str,
    ) -> ProviderUsageRequest {
        let epoch = self.binding_epoch(agent);
        self.begin_recorded(agent, provider, model, reason, prompt, epoch)
    }

    /// Preserve the admission epoch across queueing and provider execution.
    pub fn begin_at_epoch(
        &self,
        agent: &str,
        provider: &str,
        model: &str,
        reason: ProviderRequestReason,
        prompt: &str,
        epoch: u64,
    ) -> Result<ProviderUsageRequest, &'static str> {
        if self.binding_epoch(agent) != epoch {
            return Err("agent_binding_replaced");
        }
        Ok(self.begin_recorded(agent, provider, model, reason, prompt, epoch))
    }

    fn begin_recorded(
        &self,
        agent: &str,
        provider: &str,
        model: &str,
        reason: ProviderRequestReason,
        prompt: &str,
        epoch: u64,
    ) -> ProviderUsageRequest {
        let canonical = self.canonical_agent(agent);
        let agent = canonical.as_str();
        let key = (
            agent.to_owned(),
            provider.to_owned(),
            model.to_owned(),
            reason,
        );
        let mut counters = self.0.lock().expect("provider usage lock poisoned");
        let counter = counters
            .entry(key.clone())
            .or_insert_with(|| ProviderUsageCounter {
                agent: agent.to_owned(),
                provider: provider.to_owned(),
                model: model.to_owned(),
                reason,
                requests: 0,
                succeeded: 0,
                failed: 0,
                estimated_input_tokens: 0,
                estimated_output_tokens: 0,
                provider_reported_input_tokens: 0,
                provider_reported_output_tokens: 0,
                provider_reported_total_tokens: 0,
                provider_reported_responses: 0,
                last_finish_reason: None,
                token_accounting: "estimate_utf8_bytes_div_4_rounded_up_not_billing",
            });
        counter.requests = counter.requests.saturating_add(1);
        counter.estimated_input_tokens = counter
            .estimated_input_tokens
            .saturating_add(estimate(prompt));
        eprintln!(
            "adl_event {}",
            serde_json::json!({"event":"provider_request", "agent":agent, "provider":provider, "model":model, "reason":reason, "request_count":counter.requests, "estimated_input_tokens":counter.estimated_input_tokens, "token_accounting":counter.token_accounting})
        );
        ProviderUsageRequest {
            epoch,
            usage: self.clone(),
            key,
            completed: false,
        }
    }
}

/// Identity and Runtime recorder supplied by the admitted execution route.
#[derive(Clone, Copy)]
pub(crate) struct ProviderCallContext<'a> {
    pub usage: &'a ProviderUsage,
    pub agent: &'a str,
    pub reason: ProviderRequestReason,
    pub binding_epoch: u64,
}
impl ProviderCallContext<'_> {
    pub fn begin(
        &self,
        provider: &str,
        model: &str,
        prompt: &str,
    ) -> Result<ProviderUsageRequest, &'static str> {
        self.usage.begin_at_epoch(
            self.agent,
            provider,
            model,
            self.reason,
            prompt,
            self.binding_epoch,
        )
    }
}

pub struct ProviderUsageRequest {
    epoch: u64,
    usage: ProviderUsage,
    key: Key,
    completed: bool,
}
impl ProviderUsageRequest {
    pub fn failure(&self, error: &str) {
        let epochs = self.usage.4.lock().expect("provider epochs poisoned");
        if epochs.get(&self.key.0).copied().unwrap_or(0) != self.epoch {
            return;
        }
        if !matches!(error, "operation cancelled" | "provider_cancelled") {
            self.usage.observe_inference(&self.key, false);
        }
        if !matches!(error, "operation cancelled" | "provider_cancelled")
            && matches!(
                self.key.3,
                ProviderRequestReason::OperatorConversation | ProviderRequestReason::AgentToAgent
            )
        {
            self.usage.2.mark_unready(&self.key.0);
        }
    }

    pub fn success(self, response: &str) {
        self.success_with_metadata(response, None);
    }

    pub fn success_with_metadata(
        mut self,
        response: &str,
        metadata: Option<&adl_provider_core::provider::ProviderCompletionMetadata>,
    ) {
        let mut counters = self.usage.0.lock().expect("provider usage lock poisoned");
        let counter = counters
            .get_mut(&self.key)
            .expect("request counted before completion");
        counter.succeeded = counter.succeeded.saturating_add(1);
        if let Some(metadata) = metadata {
            counter.last_finish_reason = metadata.finish_reason.clone();
            let has_reported_usage = metadata.input_tokens.is_some()
                || metadata.output_tokens.is_some()
                || metadata.total_tokens.is_some();
            counter.provider_reported_input_tokens = counter
                .provider_reported_input_tokens
                .saturating_add(metadata.input_tokens.unwrap_or(0));
            counter.provider_reported_output_tokens = counter
                .provider_reported_output_tokens
                .saturating_add(metadata.output_tokens.unwrap_or(0));
            counter.provider_reported_total_tokens = counter
                .provider_reported_total_tokens
                .saturating_add(metadata.total_tokens.unwrap_or(0));
            if has_reported_usage {
                counter.provider_reported_responses =
                    counter.provider_reported_responses.saturating_add(1);
                counter.token_accounting = "provider_reported_exact_with_estimate_fallback";
            }
            if metadata.output_tokens.is_none() {
                counter.estimated_output_tokens = counter
                    .estimated_output_tokens
                    .saturating_add(estimate(response));
            }
        } else {
            counter.estimated_output_tokens = counter
                .estimated_output_tokens
                .saturating_add(estimate(response));
        }
        self.completed = true;
        let epochs = self.usage.4.lock().expect("provider epochs poisoned");
        if epochs.get(&self.key.0).copied().unwrap_or(0) == self.epoch {
            self.usage.observe_inference(&self.key, true);
        }
    }
}
impl Drop for ProviderUsageRequest {
    fn drop(&mut self) {
        if !self.completed {
            let mut counters = self.usage.0.lock().expect("provider usage lock poisoned");
            let counter = counters
                .get_mut(&self.key)
                .expect("request counted before completion");
            counter.failed = counter.failed.saturating_add(1);
        }
    }
}
fn estimate(text: &str) -> u64 {
    u64::try_from(text.len().div_ceil(4)).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    // PVF runtime: deterministic lifecycle epoch regression, no network, required.
    #[test]
    fn retired_binding_completion_cannot_change_successor_health() {
        let usage = ProviderUsage::default();
        let old = usage.begin(
            "resident",
            "ollama",
            "model",
            ProviderRequestReason::OperatorConversation,
            "fixture",
        );
        usage.invalidate_binding("resident");
        let next = usage.begin(
            "resident",
            "ollama",
            "model",
            ProviderRequestReason::OperatorConversation,
            "fixture",
        );
        next.success("ok");
        old.failure("provider_timeout");
        assert_eq!(usage.health_snapshot()[0].inference_ready, Some(true));
        usage.invalidate_binding("resident");
        old.success("stale success");
        assert!(usage.health_snapshot().is_empty());
    }

    // PVF runtime: queued retired work must not begin using successor credentials.
    #[test]
    fn queued_binding_preserves_its_admitted_epoch() {
        let usage = ProviderUsage::default();
        let epoch = usage.binding_epoch("resident");
        usage.invalidate_binding("resident");
        assert!(matches!(
            usage.begin_at_epoch(
                "resident",
                "ollama",
                "model",
                ProviderRequestReason::OperatorConversation,
                "old",
                epoch
            ),
            Err("agent_binding_replaced")
        ));
        assert!(usage.snapshot().is_empty());
        let current = usage.binding_epoch("resident");
        let request = usage
            .begin_at_epoch(
                "resident",
                "ollama",
                "model",
                ProviderRequestReason::OperatorConversation,
                "new",
                current,
            )
            .unwrap();
        usage.invalidate_binding("resident");
        request.success("retired while executing");
        assert!(usage.health_snapshot().is_empty());
    }

    #[test]
    fn provider_reported_usage_and_finish_reason_are_preserved() {
        let usage = ProviderUsage::default();
        usage
            .begin(
                "harbor.axioma",
                "bedrock_kimi_k25",
                "hosted:adl-bedrock:moonshotai.kimi-k2.5",
                ProviderRequestReason::OperatorConversation,
                "hello",
            )
            .success_with_metadata(
                "hello back",
                Some(&adl_provider_core::provider::ProviderCompletionMetadata {
                    finish_reason: Some("end_turn".to_owned()),
                    input_tokens: Some(11),
                    output_tokens: Some(7),
                    total_tokens: Some(18),
                }),
            );

        let rows = usage.snapshot();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].provider_reported_input_tokens, 11);
        assert_eq!(rows[0].provider_reported_output_tokens, 7);
        assert_eq!(rows[0].provider_reported_total_tokens, 18);
        assert_eq!(rows[0].provider_reported_responses, 1);
        assert_eq!(rows[0].last_finish_reason.as_deref(), Some("end_turn"));
        assert_eq!(
            rows[0].token_accounting,
            "provider_reported_exact_with_estimate_fallback"
        );
        assert_eq!(rows[0].estimated_output_tokens, 0);
    }

    #[test]
    fn finish_reason_without_usage_keeps_estimated_token_accounting() {
        let usage = ProviderUsage::default();
        usage
            .begin(
                "harbor.axioma",
                "bedrock_kimi_k25",
                "hosted:adl-bedrock:moonshotai.kimi-k2.5",
                ProviderRequestReason::OperatorConversation,
                "hello",
            )
            .success_with_metadata(
                "hello back",
                Some(&adl_provider_core::provider::ProviderCompletionMetadata {
                    finish_reason: Some("end_turn".to_owned()),
                    ..Default::default()
                }),
            );

        let rows = usage.snapshot();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].provider_reported_responses, 0);
        assert_eq!(rows[0].provider_reported_input_tokens, 0);
        assert_eq!(rows[0].provider_reported_output_tokens, 0);
        assert_eq!(rows[0].provider_reported_total_tokens, 0);
        assert_eq!(rows[0].estimated_input_tokens, 2);
        assert_eq!(rows[0].estimated_output_tokens, 3);
        assert_eq!(rows[0].last_finish_reason.as_deref(), Some("end_turn"));
        assert_eq!(
            rows[0].token_accounting,
            "estimate_utf8_bytes_div_4_rounded_up_not_billing"
        );
    }

    // PVF: deterministic local Runtime health-identity contract; no network;
    // required #854 regression for reusing an admitted resident ID.
    #[test]
    fn reused_resident_identity_does_not_inherit_or_overwrite_other_model_health() {
        for (new_provider, new_model) in
            [("ollama", "new-model"), ("openai-compatible", "old-model")]
        {
            let usage = ProviderUsage::default();
            usage.register_resident_alias("resident-id", "resident");
            let old_request = usage.begin(
                "resident-id",
                "ollama",
                "old-model",
                ProviderRequestReason::OperatorConversation,
                "old input",
            );
            usage.observe_metadata("resident", "ollama", "old-model", Ok(()));
            usage.observe_metadata("resident-id", new_provider, new_model, Ok(()));
            let new_request = usage.begin(
                "resident-id",
                new_provider,
                new_model,
                ProviderRequestReason::StartupProbe,
                "new input",
            );
            new_request.failure("provider request failed");
            drop(new_request);

            // An old in-flight completion after re-admission belongs only to
            // its original provider/model, regardless of completion ordering.
            old_request.success("old reply");
            let rows = usage.health_snapshot();
            assert_eq!(rows.len(), 2);
            assert!(rows.iter().all(|row| row.agent == "resident"));
            let old = rows
                .iter()
                .find(|row| row.provider == "ollama" && row.model == "old-model")
                .unwrap();
            assert_eq!(old.inference_ready, Some(true));
            let new = rows
                .iter()
                .find(|row| row.provider == new_provider && row.model == new_model)
                .unwrap();
            assert_eq!(new.inference_ready, Some(false));
            if new_provider == "openai-compatible" {
                assert_eq!(new.provider_reachable, None);
                assert_eq!(new.model_available, None);
            }
        }
    }

    // PVF: deterministic local Runtime contract; no provider/network; required
    // issue854 accounting proof. Exercises counters, cancellation and redaction.
    #[test]
    fn counts_success_failure_and_dropped_requests_without_retaining_content() {
        let usage = ProviderUsage::default();
        usage
            .begin(
                "beacon",
                "ollama",
                "cloud-model",
                ProviderRequestReason::StartupProbe,
                "secret prompt",
            )
            .success("secret reply");
        drop(usage.begin(
            "beacon",
            "ollama",
            "cloud-model",
            ProviderRequestReason::RecoveryProbe,
            "retry",
        ));
        let cancelled = usage.begin(
            "ember",
            "openai-compatible",
            "model-b",
            ProviderRequestReason::AgentToAgent,
            "agent text",
        );
        drop(cancelled);
        let initial = serde_json::to_value(usage.snapshot()).unwrap();
        for _ in 0..100 {
            assert_eq!(serde_json::to_value(usage.snapshot()).unwrap(), initial);
        }
        let rows = usage.snapshot();
        assert_eq!(rows.iter().map(|r| r.requests).sum::<u64>(), 3);
        assert_eq!(rows.iter().map(|r| r.succeeded).sum::<u64>(), 1);
        assert_eq!(rows.iter().map(|r| r.failed).sum::<u64>(), 2);
        let startup = rows
            .iter()
            .find(|r| r.reason == ProviderRequestReason::StartupProbe)
            .unwrap();
        assert_eq!(startup.estimated_input_tokens, 4);
        assert_eq!(startup.estimated_output_tokens, 3);
        assert!(initial.to_string().contains("startup_probe"));
        assert!(!initial.to_string().contains("secret"));
    }
}
