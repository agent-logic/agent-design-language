//! #903 PVF: deterministic public profile/platform contracts (CPU only), plus
//! ignored, explicitly authorized observational Metal smoke (local GPU/memory).
//! The live gate proves ADL workflow Runtime dispatch, not resident-kernel MLX
//! routing or server-side cancellation. It never starts a server/download.
use adl::adl::AdlDoc;
use adl::provider::{expand_provider_profiles, provider_profile_names};
use serde_json::json;

fn workflow(model: &str) -> AdlDoc {
    serde_json::from_value(json!({
        "version": "0.3",
        "providers": {"mlx_smoke": {"type": "mock", "default_model": "echo-v1",
            "config": {"fixed_output": "BASE_PROVIDER_MUST_NOT_EXECUTE"}}},
        "agents": {"smoke_agent": {"provider": "mlx_smoke", "model": model}},
        "tasks": {"smoke": {"prompt": {"user": "Reply with one short greeting."}}},
        "run": {"name": "mlx-smoke", "workflow": {"kind": "sequential", "steps": [
            {"id": "mlx-smoke-step", "agent": "smoke_agent", "task": "smoke"}
        ]}}
    }))
    .expect("valid bounded workflow")
}

#[test]
fn mlx_profile_expands_without_execution() {
    assert!(provider_profile_names().contains(&"mlx:llama-3.2-3b".to_owned()));
    let mut doc = workflow("fixture-model");
    adl::resolve::resolve_run(&doc).expect("production smoke workflow resolves");
    doc.providers.insert(
        "mlx_smoke".into(),
        serde_json::from_value(json!({
            "profile": "mlx:llama-3.2-3b"
        }))
        .unwrap(),
    );
    let expanded = expand_provider_profiles(&doc).expect("registered MLX profile");
    let spec = &expanded.providers["mlx_smoke"];
    assert_eq!(spec.kind, "mlx");
    assert_eq!(
        spec.default_model.as_deref(),
        Some("mlx-community/Llama-3.2-3B-Instruct-4bit")
    );
}

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
#[test]
fn mlx_unsupported_platform_fails_before_transport() {
    let spec = serde_json::from_value(json!({
        "type": "mlx", "default_model": "explicit-test-model",
        "config": {"endpoint": "http://127.0.0.1:1/v1/chat/completions",
                   "timeout_secs": 1, "max_output_tokens": 1}
    }))
    .unwrap();
    let error = adl::provider::build_provider(&spec, None)
        .err()
        .expect("unsupported platform must fail");
    assert!(format!("{error:#}").contains("unsupported_platform"));
}

/// Required actual-platform milestone gate; ignored by ordinary deterministic CI.
/// Explicit invocation (all values required, no endpoint/model defaults):
/// ADL_MLX_LIVE=1 ADL_MLX_SIDECAR=<absolute-file> \
/// ADL_MLX_SIDECAR_SHA256=<sha256> ADL_MLX_MODEL=<exact-model-id> \
/// ADL_MLX_EVIDENCE_DIR=<private-existing-dir> \
/// cargo test --manifest-path adl/Cargo.toml --test mlx_provider \
/// mlx_actual_hardware_production_workflow -- --ignored --exact --nocapture
/// Sidecar must define only mlx_smoke; the operator must pre-provision its exact
/// model/server and record macOS/Metal/MLX/hardware/candidate and memory limits.
#[test]
#[ignore = "requires explicit operator-approved preloaded MLX model and Apple silicon"]
fn mlx_actual_hardware_production_workflow() {
    use sha2::{Digest, Sha256};
    use std::{fs, io::Write, path::PathBuf, time::Instant};
    // The production adapter constructor enforces the platform gate before I/O.
    assert_eq!(
        std::env::var("ADL_MLX_LIVE").as_deref(),
        Ok("1"),
        "explicit live opt-in required"
    );
    let required = |key: &str| {
        std::env::var(key)
            .ok()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| panic!("{key} is required"))
    };
    let source = PathBuf::from(required("ADL_MLX_SIDECAR"));
    let expected_digest = required("ADL_MLX_SIDECAR_SHA256");
    let expected_model = required("ADL_MLX_MODEL");
    let evidence = PathBuf::from(required("ADL_MLX_EVIDENCE_DIR"));
    assert!(source.is_absolute() && evidence.is_absolute() && evidence.is_dir());
    assert!(
        !evidence.join("mlx-production-smoke.json").exists(),
        "receipt already exists; choose a new evidence directory before inference"
    );
    let raw = fs::read(&source).expect("read approved sidecar");
    assert_eq!(
        format!("{:x}", Sha256::digest(&raw)),
        expected_digest,
        "sidecar pin mismatch"
    );
    // Freeze approved bytes so a concurrently edited operator sidecar cannot
    // change the model/endpoint after the pin is checked.
    let run = tempfile::Builder::new()
        .prefix("mlx-proof-")
        .tempdir_in(&evidence)
        .unwrap();
    let pinned = run.path().join("providers.yaml");
    fs::write(&pinned, &raw).unwrap();
    let base = workflow(&expected_model);
    let resolved = adl::resolve::resolve_run(&base).expect("resolve real workflow");
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
    let spec = &snapshot.document.providers["mlx_smoke"];
    assert_eq!(spec.kind, "mlx", "mock/alternate provider forbidden");
    let target =
        adl::provider_substrate::provider_invocation_target_v1("mlx_smoke", spec, None).unwrap();
    assert_eq!(
        target.provider_model_id, expected_model,
        "selected model pin mismatch"
    );
    assert_ne!(expected_model, "default_model");
    let timeout = spec
        .config
        .get("timeout_secs")
        .and_then(|v| v.as_u64())
        .expect("explicit timeout");
    let started = Instant::now();
    let result = execute_workflow(&resolved, run.path(), &handle);
    let elapsed = started.elapsed();
    runtime
        .block_on(owner.shutdown())
        .expect("settle local reload watcher");
    let result = result.expect("actual production MLX workflow response");
    assert_eq!(result.outputs.len(), 1);
    let output = &result.outputs[0].model_output;
    assert!(!output.trim().is_empty());
    assert!(!output.contains("BASE_PROVIDER_MUST_NOT_EXECUTE"));
    assert!(output.len() <= 1_048_576);
    assert!(
        elapsed.as_secs() <= timeout + 5,
        "workflow exceeded bounded transport budget"
    );
    let receipt = json!({"schema": "adl.mlx.production-smoke.v1", "result": "pass",
        "dispatch": "execute_sequential_with_provider_reload_handle", "model": expected_model,
        "sidecar_sha256": expected_digest, "elapsed_ms": elapsed.as_millis(),
        "output_bytes": output.len(), "output_sha256": format!("{:x}", Sha256::digest(output.as_bytes())),
        "snapshot_digest": snapshot.digest, "server_cancellation_proven": false});
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(evidence.join("mlx-production-smoke.json"))
        .expect("new evidence receipt (do not overwrite)");
    file.write_all(&serde_json::to_vec_pretty(&receipt).unwrap())
        .unwrap();
    println!("actual MLX production workflow PASS; receipt retained (output content omitted)");
}

fn execute_workflow(
    resolved: &adl::resolve::AdlResolved,
    directory: &std::path::Path,
    handle: &adl::provider::ProviderReloadHandle,
) -> anyhow::Result<adl::execute::ExecutionResult> {
    let mut trace = adl::trace::Trace::new("mlx-smoke", &resolved.workflow_id, "0.3");
    adl::execute::execute_sequential_with_provider_reload_handle(
        resolved, &mut trace, false, false, directory, directory, handle,
    )
}

/// Deterministic CPU/loopback integration: registered MLX transport is exercised
/// through exactly the executor helper used by the live smoke. No Metal service.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[test]
fn mlx_loopback_production_workflow_uses_pinned_agent_and_reloaded_provider() {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        time::{Duration, Instant},
    };
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let endpoint = format!(
        "http://{}/v1/chat/completions",
        listener.local_addr().unwrap()
    );
    let server = std::thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut socket = loop {
            match listener.accept() {
                Ok((socket, _)) => break socket,
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock && Instant::now() < deadline =>
                {
                    std::thread::sleep(Duration::from_millis(10))
                }
                Err(e) => panic!("bounded fixture accept failed: {e}"),
            }
        };
        socket
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        let mut bytes = Vec::new();
        let (start, length) = loop {
            let mut chunk = [0; 2048];
            let count = socket.read(&mut chunk).unwrap();
            assert!(count > 0);
            bytes.extend_from_slice(&chunk[..count]);
            assert!(bytes.len() < 65536);
            if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                let headers = std::str::from_utf8(&bytes[..end]).unwrap();
                assert!(headers.starts_with("POST /v1/chat/completions HTTP/1.1"));
                let length: usize = headers
                    .lines()
                    .find_map(|line| {
                        let (key, value) = line.split_once(':')?;
                        key.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse().unwrap())
                    })
                    .unwrap();
                assert!(length < 32768);
                break (end + 4, length);
            }
        };
        while bytes.len() < start + length {
            let mut chunk = [0; 2048];
            let count = socket.read(&mut chunk).unwrap();
            assert!(count > 0);
            bytes.extend_from_slice(&chunk[..count]);
        }
        let request: serde_json::Value =
            serde_json::from_slice(&bytes[start..start + length]).unwrap();
        assert_eq!(request["model"], "pinned-fixture-model");
        assert_eq!(request["max_tokens"], 8);
        assert_eq!(request["stream"], false);
        let body = json!({"model":"pinned-fixture-model", "choices":[{
            "message":{"role":"assistant", "content":"fixture response"}, "finish_reason":"stop"
        }]})
        .to_string();
        write!(socket, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
    });
    let base = workflow("pinned-fixture-model");
    let resolved = adl::resolve::resolve_run(&base).unwrap();
    let scratch =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.adl/runs/mlx-fixtures");
    std::fs::create_dir_all(&scratch).unwrap();
    let directory = tempfile::tempdir_in(scratch).unwrap();
    let sidecar = directory.path().join("providers.yaml");
    std::fs::write(
        &sidecar,
        serde_json::to_vec(&json!({
            "schema":"adl.provider_reload_sidecar.v1", "version":"0.3",
            "providers":{"mlx_smoke":{"type":"mlx", "default_model":"pinned-fixture-model",
                "config":{"endpoint":endpoint,"timeout_secs":3,"max_output_tokens":8}}}
        }))
        .unwrap(),
    )
    .unwrap();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let owner = runtime
        .block_on(adl::provider::ProviderReloadOwner::start(
            sidecar,
            base,
            adl_runtime_kernel::config_reload::ConfigReloadOptions::default(),
        ))
        .unwrap();
    let result = execute_workflow(&resolved, directory.path(), &owner.handle());
    runtime.block_on(owner.shutdown()).unwrap();
    server.join().unwrap();
    let result = result.unwrap();
    assert_eq!(result.outputs.len(), 1);
    assert_eq!(result.outputs[0].model_output, "fixture response");
}
