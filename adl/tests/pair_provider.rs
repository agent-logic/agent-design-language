//! #904 PVF: deterministic canonical-sidecar validation plus an ignored,
//! operator-attended PAIR/Runtime integration probe. The live probe uses the
//! production workflow executor and provider reload owner; it does not start
//! PAIR, pair nodes, download models, or infer serving-node identity.

use adl::adl::AdlDoc;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpusRow {
    id: String,
    prompt: String,
    expected_text: String,
}

fn workflow(rows: &[CorpusRow], model: &str, concurrency: usize) -> AdlDoc {
    let tasks = rows
        .iter()
        .map(|row| (row.id.clone(), json!({"prompt": {"user": row.prompt}})))
        .collect::<serde_json::Map<_, _>>();
    let steps = rows
        .iter()
        .map(|row| json!({"id": row.id, "agent": "pair_agent", "task": row.id}))
        .collect::<Vec<_>>();
    serde_json::from_value(json!({
        "version": "0.3",
        "providers": {"pair_smoke": {
            "type": "mock",
            "default_model": "echo-v1",
            "config": {"fixed_output": "BASE_PROVIDER_MUST_NOT_EXECUTE"}
        }},
        "agents": {"pair_agent": {"provider": "pair_smoke", "model": model}},
        "tasks": tasks,
        "run": {"name": "pair-runtime-smoke", "workflow": {
            "kind": "concurrent", "max_concurrency": concurrency, "steps": steps
        }}
    }))
    .expect("valid bounded PAIR workflow")
}

#[test]
fn pair_workflow_shape_is_bounded_and_concurrent() {
    let rows = vec![
        CorpusRow {
            id: "q1".into(),
            prompt: "Return one".into(),
            expected_text: "one".into(),
        },
        CorpusRow {
            id: "q2".into(),
            prompt: "Return two".into(),
            expected_text: "two".into(),
        },
    ];
    let doc = workflow(&rows, "fixture-model", 2);
    let resolved = adl::resolve::resolve_run(&doc).expect("production workflow resolves");
    assert_eq!(resolved.execution_plan.nodes.len(), 2);
    assert_eq!(
        resolved.execution_plan.workflow_kind,
        adl::adl::WorkflowKind::Concurrent
    );
}

/// Required actual Runtime route gate; ignored by ordinary deterministic CI.
/// The caller supplies a previously reviewed canonical provider sidecar and a
/// small exact-output corpus. The receipt omits prompts and response content.
#[test]
#[ignore = "requires an operator-attended PAIR service and preloaded model"]
fn pair_actual_runtime_workflow() {
    use sha2::{Digest, Sha256};
    use std::{
        collections::BTreeMap, fs, io::Write, path::PathBuf, process::Command, time::Instant,
    };

    assert_eq!(
        std::env::var("ADL_PAIR_LIVE").as_deref(),
        Ok("1"),
        "explicit live opt-in required"
    );
    let required = |key: &str| {
        std::env::var(key)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| panic!("{key} is required"))
    };
    let source = PathBuf::from(required("ADL_PAIR_SIDECAR"));
    let expected_digest = required("ADL_PAIR_SIDECAR_SHA256");
    let corpus_path = PathBuf::from(required("ADL_PAIR_CORPUS"));
    let model = required("ADL_PAIR_MODEL");
    let evidence = PathBuf::from(required("ADL_PAIR_EVIDENCE_DIR"));
    let concurrency: usize = required("ADL_PAIR_CONCURRENCY")
        .parse()
        .expect("ADL_PAIR_CONCURRENCY must be an integer");
    assert!(source.is_absolute() && corpus_path.is_absolute() && evidence.is_absolute());
    assert!(evidence.is_dir());
    assert!((1..=16).contains(&concurrency));
    let receipt_path = evidence.join("pair-runtime-smoke.json");
    assert!(
        !receipt_path.exists(),
        "receipt already exists; use a new evidence directory"
    );

    let raw = fs::read(&source).expect("read approved provider sidecar");
    assert_eq!(
        format!("{:x}", Sha256::digest(&raw)),
        expected_digest,
        "sidecar pin mismatch"
    );
    let corpus_bytes = fs::read(&corpus_path).expect("read approved corpus");
    assert!(
        corpus_bytes.len() <= 1024 * 1024,
        "corpus exceeds 1 MiB bound"
    );
    let rows: Vec<CorpusRow> = serde_json::from_slice(&corpus_bytes).expect("valid corpus JSON");
    assert!((2..=32).contains(&rows.len()));
    assert!(rows.len() >= concurrency);
    for row in &rows {
        assert!(!row.id.is_empty() && row.id.len() <= 100);
        assert!(!row.prompt.is_empty() && row.prompt.len() <= 16 * 1024);
        assert!(!row.expected_text.is_empty() && row.expected_text.len() <= 16 * 1024);
    }
    let git = Command::new("git")
        .args(["-C", env!("CARGO_MANIFEST_DIR"), "rev-parse", "HEAD"])
        .output()
        .expect("resolve candidate git revision");
    assert!(git.status.success() && git.stdout.len() <= 65);
    let candidate_sha = String::from_utf8(git.stdout)
        .expect("git revision is UTF-8")
        .trim()
        .to_owned();
    assert_eq!(candidate_sha.len(), 40);

    let run = tempfile::Builder::new()
        .prefix("pair-runtime-proof-")
        .tempdir_in(&evidence)
        .expect("create bounded run directory");
    let pinned = run.path().join("providers.yaml");
    fs::write(&pinned, &raw).expect("freeze approved sidecar bytes");
    let base = workflow(&rows, &model, concurrency);
    let resolved = adl::resolve::resolve_run(&base).expect("resolve real PAIR workflow");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let owner = runtime
        .block_on(adl::provider::ProviderReloadOwner::start(
            pinned,
            base,
            adl_runtime_kernel::config_reload::ConfigReloadOptions::default(),
        ))
        .expect("activate pinned canonical sidecar");
    let handle = owner.handle();
    let snapshot = handle.current_snapshot();
    assert_eq!(snapshot.document.providers.len(), 1);
    let spec = &snapshot.document.providers["pair_smoke"];
    assert_eq!(
        spec.kind, "ollama",
        "PAIR remains infrastructure beneath ollama"
    );
    assert_eq!(spec.default_model.as_deref(), Some(model.as_str()));
    let endpoint = spec
        .base_url
        .as_deref()
        .expect("explicit PAIR loopback endpoint");
    let parsed = reqwest::Url::parse(endpoint).expect("valid endpoint");
    assert_eq!(parsed.scheme(), "http");
    assert!(matches!(parsed.host_str(), Some("127.0.0.1") | Some("::1")));
    assert!(parsed.port().is_some());

    let started = Instant::now();
    let mut trace = adl::trace::Trace::new("pair-runtime-smoke", &resolved.workflow_id, "0.3");
    let result = adl::execute::execute_sequential_with_provider_reload_handle(
        &resolved,
        &mut trace,
        false,
        false,
        run.path(),
        run.path(),
        &handle,
    );
    let elapsed = started.elapsed();
    runtime
        .block_on(owner.shutdown())
        .expect("settle provider reload owner");
    let result = result.expect("actual Runtime workflow through PAIR");
    assert_eq!(result.outputs.len(), rows.len());
    let expected = rows
        .iter()
        .map(|row| (row.id.as_str(), row.expected_text.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut outputs = Vec::new();
    for output in &result.outputs {
        let expected_text = expected
            .get(output.step_id.as_str())
            .expect("Runtime returned an unknown step id");
        assert_eq!(output.model_output.trim(), *expected_text);
        outputs.push(json!({
            "id": output.step_id,
            "output_bytes": output.model_output.len(),
            "output_sha256": format!("{:x}", Sha256::digest(output.model_output.as_bytes()))
        }));
    }
    outputs.sort_by(|left, right| left["id"].as_str().cmp(&right["id"].as_str()));
    let receipt = json!({
        "schema": "adl.pair.runtime-smoke.v1",
        "result": "pass",
        "candidate_sha": candidate_sha,
        "dispatch": "execute_sequential_with_provider_reload_handle",
        "provider_kind": "ollama",
        "model": model,
        "sidecar_sha256": expected_digest,
        "corpus_sha256": format!("{:x}", Sha256::digest(&corpus_bytes)),
        "snapshot_digest": snapshot.digest,
        "concurrency": concurrency,
        "attempts": rows.len(),
        "elapsed_ms": elapsed.as_millis(),
        "outputs": outputs,
        "pair_service_started_by_probe": false,
        "serving_node_proven_by_probe": false
    });
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(&receipt_path)
        .expect("create-only Runtime receipt");
    file.write_all(&serde_json::to_vec_pretty(&receipt).unwrap())
        .expect("write Runtime receipt");
    println!("actual PAIR Runtime workflow PASS; content-redacted receipt retained");
}
