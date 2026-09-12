//! ADL compatibility facade for the shared provider adapter owner.
pub(crate) use adl_provider_core::profiles::{
    is_allowed_ollama_endpoint, is_allowed_remote_endpoint,
};
pub use adl_provider_core::provider::*;
mod profiles;
pub mod reload;
pub use profiles::{
    activate_provider_profile_candidate, expand_provider_profiles,
    provider_profile_materialization_projection, ProviderProfileActivation,
};
pub use reload::{
    current_provider_reload_document, set_global_provider_reload_handle, ProviderReloadDiagnostic,
    ProviderReloadGlobalGuard, ProviderReloadHandle, ProviderReloadOwner, ProviderReloadSnapshot,
};

/// Preserve remote-execution classification at the ADL boundary.
pub fn is_retryable_error(err: &anyhow::Error) -> bool {
    if adl_provider_core::stable_failure_kind(err).is_some() {
        return adl_provider_core::is_retryable_error(err);
    }
    crate::remote_exec::retryability(err).unwrap_or(true)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::adl;
    use std::collections::HashMap;
    #[test]
    fn provider_mod_remote_retry_classification_distinguishes_deterministic_failures() {
        let schema = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::SchemaViolation,
            "REMOTE_SCHEMA_VIOLATION",
            "missing result on ok response",
        ));
        assert!(!is_retryable_error(&schema));

        let envelope = anyhow::Error::new(crate::remote_exec::SecurityEnvelopeError::MissingKeyId);
        assert!(!is_retryable_error(&envelope));

        let remote_schema = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::RemoteExecution,
            "REMOTE_SCHEMA_VIOLATION",
            "invalid provider config",
        ));
        assert!(!is_retryable_error(&remote_schema));

        let timeout = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::Timeout,
            "REMOTE_TIMEOUT",
            "timed out",
        ));
        assert!(is_retryable_error(&timeout));
    }

    #[test]
    fn provider_mod_profile_expansion_materializes_bounded_inference_defaults() {
        let mut doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:phi4-mini".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::new(),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };

        let expanded = expand_provider_profiles(&doc).expect("profile expansion");
        let local = expanded.providers.get("local").expect("local provider");
        assert_eq!(local.kind, "ollama");
        assert_eq!(local.default_model.as_deref(), Some("phi4-mini"));
        assert_eq!(
            local.config["provider_model_id"],
            serde_json::json!("phi4-mini")
        );
        assert_eq!(local.config["temperature"], serde_json::json!(0.0));
        assert_eq!(local.config["top_p"], serde_json::json!(1.0));
        assert_eq!(local.config["max_output_tokens"], serde_json::json!(512));
        assert_eq!(local.config["timeout_secs"], serde_json::json!(120));
        assert_eq!(local.config["deterministic_seed"], serde_json::json!(0));
        assert_eq!(
            local.config["materialization_policy"],
            serde_json::json!("deterministic_ollama_v1")
        );
        assert_eq!(
            local.config["profile_state"]["retention"],
            serde_json::json!("retain_last_valid_materialization")
        );

        doc.providers
            .get_mut("local")
            .expect("local")
            .config
            .insert("temperature".to_string(), serde_json::json!(3.0));
        let err = expand_provider_profiles(&doc).expect_err("out-of-bounds profile should fail");
        assert!(err.to_string().contains("config.temperature"));
    }

    #[test]
    fn provider_mod_profile_materialization_projection_is_stable_and_redacted() {
        let doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:phi4-mini".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::from([(
                        "metadata".to_string(),
                        serde_json::json!({
                            "safe_count": 2,
                            "api_key": "secret-key",
                            "password": 123456,
                            "pin": 654321,
                            "passphrase": {
                                "hint": "swordfish"
                            },
                            "private_payload": {
                                "prompt": "do not retain"
                            }
                        }),
                    )]),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };

        let projection1 =
            provider_profile_materialization_projection(&doc).expect("projection run 1");
        let projection2 =
            provider_profile_materialization_projection(&doc).expect("projection run 2");
        let json1 = serde_json::to_string(&projection1).expect("serialize projection 1");
        let json2 = serde_json::to_string(&projection2).expect("serialize projection 2");
        assert_eq!(json1, json2, "canonical projection must be byte-stable");
        assert!(json1.contains("adl.provider_profile_materialization_projection.v1"));
        assert!(json1.contains("\"safe_count\":2"));
        assert!(json1.contains("<redacted>"));
        assert!(!json1.contains("secret-key"));
        assert!(!json1.contains("do not retain"));
        assert!(!json1.contains("123456"));
        assert!(!json1.contains("654321"));
        assert!(!json1.contains("swordfish"));

        let mixed = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "hosted".to_string(),
                adl::ProviderSpec {
                    id: Some("hosted".to_string()),
                    profile: None,
                    kind: "http".to_string(),
                    base_url: Some(
                        "https://user:token@example.invalid/v1?api_key=secret".to_string(),
                    ),
                    default_model: Some("gpt-test".to_string()),
                    config: HashMap::new(),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };
        let mixed_projection =
            provider_profile_materialization_projection(&mixed).expect("mixed projection");
        let mixed_json = serde_json::to_string(&mixed_projection).expect("mixed json");
        assert_eq!(
            mixed_projection["providers"]["hosted"]["base_url_present"],
            serde_json::json!(true)
        );
        assert!(!mixed_json.contains("user:token"));
        assert!(!mixed_json.contains("api_key=secret"));
        assert!(!mixed_json.contains("https://user"));
    }

    #[test]
    fn provider_mod_profile_expansion_rejects_non_deterministic_ollama_seed() {
        let doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:phi4-mini".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::from([(
                        "deterministic_seed".to_string(),
                        serde_json::json!(7),
                    )]),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };

        let err = expand_provider_profiles(&doc).expect_err("seed drift should fail");
        assert!(err.to_string().contains("deterministic_seed must remain 0"));
    }

    #[test]
    fn provider_mod_profile_expansion_rejects_malformed_inference_values() {
        for (key, value) in [
            ("temperature", serde_json::json!("hot")),
            ("temperature", serde_json::json!("0.2")),
            ("top_p", serde_json::json!(true)),
            ("timeout_secs", serde_json::json!("later")),
            ("timeout_secs", serde_json::json!("120")),
            ("timeout_secs", serde_json::json!(601)),
            ("max_output_tokens", serde_json::json!(-1)),
            ("max_output_tokens", serde_json::json!(32_769)),
            ("deterministic_seed", serde_json::json!("seed")),
            ("deterministic_seed", serde_json::json!("0")),
        ] {
            let doc = adl::AdlDoc {
                version: "0.92".to_string(),
                providers: HashMap::from([(
                    "local".to_string(),
                    adl::ProviderSpec {
                        id: Some("local".to_string()),
                        profile: Some("ollama:phi4-mini".to_string()),
                        kind: String::new(),
                        base_url: None,
                        default_model: None,
                        config: HashMap::from([(key.to_string(), value)]),
                    },
                )]),
                tools: HashMap::new(),
                agents: HashMap::new(),
                tasks: HashMap::new(),
                workflows: HashMap::new(),
                patterns: Vec::new(),
                signature: None,
                run: adl::RunSpec {
                    id: None,
                    name: None,
                    created_at: None,
                    defaults: Default::default(),
                    workflow_ref: None,
                    workflow: None,
                    pattern_ref: None,
                    inputs: HashMap::new(),
                    placement: None,
                    remote: None,
                    delegation_policy: None,
                },
            };
            let err = expand_provider_profiles(&doc).expect_err("malformed profile should fail");
            assert!(err.to_string().contains(key), "{key}: {err:#}");
        }
    }

    #[test]
    fn provider_mod_profile_expansion_rejects_provider_model_id_conflicts() {
        for value in [
            serde_json::json!("llama3.1:8b"),
            serde_json::json!(123),
            serde_json::json!(true),
            serde_json::json!({ "model": "phi4-mini" }),
        ] {
            let doc = adl::AdlDoc {
                version: "0.92".to_string(),
                providers: HashMap::from([(
                    "local".to_string(),
                    adl::ProviderSpec {
                        id: Some("local".to_string()),
                        profile: Some("ollama:phi4-mini".to_string()),
                        kind: String::new(),
                        base_url: None,
                        default_model: None,
                        config: HashMap::from([("provider_model_id".to_string(), value)]),
                    },
                )]),
                tools: HashMap::new(),
                agents: HashMap::new(),
                tasks: HashMap::new(),
                workflows: HashMap::new(),
                patterns: Vec::new(),
                signature: None,
                run: adl::RunSpec {
                    id: None,
                    name: None,
                    created_at: None,
                    defaults: Default::default(),
                    workflow_ref: None,
                    workflow: None,
                    pattern_ref: None,
                    inputs: HashMap::new(),
                    placement: None,
                    remote: None,
                    delegation_policy: None,
                },
            };

            let err = expand_provider_profiles(&doc).expect_err("model conflict should fail");
            assert!(err.to_string().contains("provider_model_id"));
        }
    }

    #[test]
    fn provider_mod_profile_expansion_rejects_malformed_identity_config() {
        for (profile, key, value) in [
            ("z_ai:glm-5", "endpoint", serde_json::json!(123)),
            (
                "z_ai:glm-5",
                "endpoint",
                serde_json::json!({ "url": "https://open.bigmodel.cn/api/paas/v4/chat/completions" }),
            ),
            ("ollama:phi4-mini", "vendor", serde_json::json!(true)),
        ] {
            let doc = adl::AdlDoc {
                version: "0.92".to_string(),
                providers: HashMap::from([(
                    "local".to_string(),
                    adl::ProviderSpec {
                        id: Some("local".to_string()),
                        profile: Some(profile.to_string()),
                        kind: String::new(),
                        base_url: None,
                        default_model: None,
                        config: HashMap::from([(key.to_string(), value)]),
                    },
                )]),
                tools: HashMap::new(),
                agents: HashMap::new(),
                tasks: HashMap::new(),
                workflows: HashMap::new(),
                patterns: Vec::new(),
                signature: None,
                run: adl::RunSpec {
                    id: None,
                    name: None,
                    created_at: None,
                    defaults: Default::default(),
                    workflow_ref: None,
                    workflow: None,
                    pattern_ref: None,
                    inputs: HashMap::new(),
                    placement: None,
                    remote: None,
                    delegation_policy: None,
                },
            };

            let err = expand_provider_profiles(&doc).expect_err("malformed config should fail");
            assert!(err.to_string().contains(key), "{key}: {err:#}");
        }
    }

    #[test]
    fn provider_mod_profile_state_retains_previous_last_known_good() {
        let active_doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:phi4-mini".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::new(),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };
        let active = expand_provider_profiles(&active_doc).expect("active profile expansion");
        let mut doc = active_doc.clone();
        let local = doc.providers.get_mut("local").expect("local provider");
        local.profile = Some("ollama:qwen2.5-7b".to_string());
        local.config.insert(
            "profile_state".to_string(),
            active.providers["local"].config["profile_state"].clone(),
        );

        let expanded = expand_provider_profiles(&doc).expect("profile expansion");
        let state = &expanded.providers["local"].config["profile_state"];
        assert_eq!(state["profile"], serde_json::json!("ollama:qwen2.5-7b"));
        assert_eq!(
            state["last_known_good_profile"],
            serde_json::json!("ollama:phi4-mini")
        );
        assert_eq!(
            state["last_known_good_materialization"]["schema"],
            serde_json::json!("adl.provider_profile_materialization_state.v1")
        );
        assert_eq!(
            state["last_known_good_materialization"]["profile"],
            serde_json::json!("ollama:phi4-mini")
        );
        assert_eq!(
            state["last_known_good_materialization"]["default_model"],
            serde_json::json!("phi4-mini")
        );
    }

    #[test]
    fn provider_mod_profile_activation_preserves_active_materialization_on_invalid_candidate() {
        let active_doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:phi4-mini".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::from([(
                        "private_payload".to_string(),
                        serde_json::json!("do not leak active prompt"),
                    )]),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };
        let active = expand_provider_profiles(&active_doc).expect("active materialization");
        let active_provider = active.providers["local"].clone();
        let active_projection = redacted_provider_profile_projection("local", &active_provider);

        let mut candidate_doc = active_doc.clone();
        let candidate = candidate_doc.providers.get_mut("local").expect("candidate");
        candidate.profile = Some("ollama:qwen2.5-7b".to_string());
        candidate
            .config
            .insert("temperature".to_string(), serde_json::json!(3.0));
        candidate.config.insert(
            "profile_state".to_string(),
            active_provider.config["profile_state"].clone(),
        );

        let rejected = activate_provider_profile_candidate(&active_doc, &candidate_doc)
            .expect("invalid candidate should retain active state");
        assert!(!rejected.accepted);
        assert!(rejected
            .rejection
            .as_deref()
            .unwrap_or_default()
            .contains("config.temperature"));
        assert_eq!(
            rejected.document, active,
            "failed candidate activation must return the retained active materialization"
        );
        assert_eq!(
            redacted_provider_profile_projection("local", &rejected.document.providers["local"]),
            active_projection
        );

        let mut valid_candidate_doc = active_doc.clone();
        valid_candidate_doc
            .providers
            .get_mut("local")
            .expect("candidate")
            .profile = Some("ollama:qwen2.5-7b".to_string());
        let accepted = activate_provider_profile_candidate(&active_doc, &valid_candidate_doc)
            .expect("valid candidate should promote");
        assert!(accepted.accepted);
        assert!(accepted.rejection.is_none());
        let promoted_state = &accepted.document.providers["local"].config["profile_state"];
        assert_eq!(
            promoted_state["last_known_good_profile"],
            serde_json::json!("ollama:qwen2.5-7b")
        );
        assert_eq!(
            promoted_state["last_known_good_materialization"]["profile"],
            serde_json::json!("ollama:qwen2.5-7b")
        );

        let mut chained_invalid_doc = active_doc.clone();
        let chained_invalid = chained_invalid_doc
            .providers
            .get_mut("local")
            .expect("candidate");
        chained_invalid.profile = Some("ollama:phi4-mini".to_string());
        chained_invalid
            .config
            .insert("temperature".to_string(), serde_json::json!(3.0));
        let retained =
            activate_provider_profile_candidate(&accepted.document, &chained_invalid_doc)
                .expect("chained invalid candidate should retain materialized active state");
        assert!(!retained.accepted);
        assert_eq!(
            retained.document, accepted.document,
            "returned materialized activation document must be reusable as active state"
        );

        let accepted_again = activate_provider_profile_candidate(&retained.document, &active_doc)
            .expect("retained materialized active state should allow later valid activation");
        assert!(accepted_again.accepted);
        assert_eq!(
            accepted_again.document.providers["local"].config["profile_state"]
                ["last_known_good_profile"],
            serde_json::json!("ollama:phi4-mini")
        );
    }

    #[test]
    fn provider_mod_profile_state_rejects_unknown_last_known_good() {
        let doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:qwen2.5-7b".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::from([(
                        "profile_state".to_string(),
                        serde_json::json!({
                            "schema": "adl.provider_profile_state.v1",
                            "profile": "ollama:phi4-mini",
                            "last_known_good_profile": "ollama:unknown",
                            "retention": "retain_last_valid_materialization",
                            "activation": "validate_before_activation"
                        }),
                    )]),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };

        let err = expand_provider_profiles(&doc).expect_err("unknown LKG should fail");
        assert!(err.to_string().contains("last_known_good_profile"));
    }
}
