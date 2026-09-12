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
        if success {
            signals.provider_reachable = Some(true);
            signals.model_available = Some(true);
        }
    }

    /// The outer execution deadline can cancel a provider future before it
    /// reports its own timeout. Preserve that failure without counting a second
    /// provider attempt. Operator cancellation must not call this method.
    pub(crate) fn observe_execution_timeout(&self, agent: &str, provider: &str, model: &str) {
        let agent = self.canonical_agent(agent);
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
}
impl ProviderCallContext<'_> {
    pub fn begin(&self, provider: &str, model: &str, prompt: &str) -> ProviderUsageRequest {
        self.usage
            .begin(self.agent, provider, model, self.reason, prompt)
    }
}

pub struct ProviderUsageRequest {
    usage: ProviderUsage,
    key: Key,
    completed: bool,
}
impl ProviderUsageRequest {
    pub fn failure(&self, error: &str) {
        if error != "operation cancelled" {
            self.usage.observe_inference(&self.key, false);
        }
        if error != "operation cancelled"
            && matches!(
                self.key.3,
                ProviderRequestReason::OperatorConversation | ProviderRequestReason::AgentToAgent
            )
        {
            self.usage.2.mark_unready(&self.key.0);
        }
    }

    pub fn success(mut self, response: &str) {
        let mut counters = self.usage.0.lock().expect("provider usage lock poisoned");
        let counter = counters
            .get_mut(&self.key)
            .expect("request counted before completion");
        counter.succeeded = counter.succeeded.saturating_add(1);
        counter.estimated_output_tokens = counter
            .estimated_output_tokens
            .saturating_add(estimate(response));
        self.completed = true;
        self.usage.observe_inference(&self.key, true);
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
