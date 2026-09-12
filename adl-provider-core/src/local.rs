//! Local and CLI-backed provider implementations.
//!
//! Includes the mock test provider and local command-line Ollama adapter used for
//! offline or deterministic integration behavior.
use super::http_family::timeout_secs;
use super::*;

#[derive(Debug, Clone)]
/// Deterministic local provider used by tests and local smoke flows.
pub struct MockProvider {
    model: String,
    fixed_output: Option<String>,
    sleep_ms: u64,
}

impl MockProvider {
    /// Build the mock provider from a normalized target.
    pub fn from_target(spec: &adl::ProviderSpec, target: &ProviderInvocationTargetV1) -> Self {
        Self {
            model: target.model_ref.clone(),
            fixed_output: spec
                .config
                .get("fixed_output")
                .and_then(|value| value.as_str())
                .map(ToString::to_string),
            sleep_ms: spec
                .config
                .get("sleep_ms")
                .and_then(|value| value.as_u64())
                .unwrap_or(0),
        }
    }
}

impl Provider for MockProvider {
    /// Returns the input prompt unchanged for deterministic pass-through testing.
    fn complete(&self, prompt: &str) -> Result<String> {
        let _model = &self.model;
        if self.sleep_ms > 0 {
            thread::sleep(Duration::from_millis(self.sleep_ms));
        }
        Ok(self
            .fixed_output
            .clone()
            .unwrap_or_else(|| prompt.to_string()))
    }
}

/// Ollama provider (blocking) using the local `ollama` CLI.
/// This keeps v0.1 dependency-light and works well for local prototyping.
#[derive(Debug, Clone)]
/// Local Ollama provider backed by the `ollama` binary.
pub struct OllamaProvider {
    pub model: String,
    pub temperature: Option<f32>,
}

impl OllamaProvider {
    /// Build an Ollama provider from ADL spec and optional run-time model override.
    pub fn from_spec(spec: &adl::ProviderSpec, model_override: Option<&str>) -> Result<Self> {
        let target = provider_substrate::provider_invocation_target_v1(
            spec.id.as_deref().unwrap_or("<anonymous-provider>"),
            spec,
            model_override,
        )?;
        Self::from_target(spec, &target)
    }

    /// Build from a resolved invocation target after `provider_substrate` expansion.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let temperature = cfg_f32(&spec.config, "temperature");

        Ok(Self {
            // Local CLI execution has no separate provider-native model identifier surface,
            // so the stable model_ref is the runtime model we should actually invoke.
            model: target.model_ref.clone(),
            temperature,
        })
    }

    fn complete_streaming(
        &self,
        prompt: &str,
        mut on_chunk: Option<&mut dyn FnMut(&str)>,
    ) -> Result<String> {
        let timeout_secs =
            timeout_secs().map_err(|err| invalid_config("ollama", err.to_string()))?;

        // v0.1: We parse `temperature` from provider config for forward-compatibility,
        // but the `ollama` CLI does not consistently expose a stable flag across versions.
        // Read the field so it does not trip `-D dead-code`, and keep behavior deterministic.
        let _temperature = self.temperature;
        let mut child = Command::new(ollama_bin())
            .arg("run")
            .arg(&self.model)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| "failed to spawn `ollama run` (is Ollama installed and on PATH?)")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        let stdout = child
            .stdout
            .take()
            .context("failed to open stdout for ollama")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        let stderr = child
            .stderr
            .take()
            .context("failed to open stderr for ollama")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        let out_handle = thread::spawn(move || -> std::io::Result<()> {
            let mut r = stdout;
            let mut buf = [0u8; 4096];
            loop {
                let n = r.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                if tx.send(buf[..n].to_vec()).is_err() {
                    break;
                }
            }
            Ok(())
        });

        let err_handle = thread::spawn(move || -> std::io::Result<Vec<u8>> {
            let mut r = stderr;
            let mut buf = Vec::new();
            r.read_to_end(&mut buf)?;
            Ok(buf)
        });

        {
            let mut stdin = child
                .stdin
                .take()
                .context("failed to open stdin for ollama")
                .map_err(|err| runtime_error("ollama", err.to_string()))?;
            stdin
                .write_all(prompt.as_bytes())
                .context("failed writing prompt to ollama stdin")
                .map_err(|err| runtime_error("ollama", err.to_string()))?;
            drop(stdin);
        }

        let start = Instant::now();
        let timeout = Duration::from_secs(timeout_secs);
        let mut out_buf = Vec::new();
        let mut stream_utf8_buf = Vec::new();

        let status = loop {
            while let Ok(chunk) = rx.try_recv() {
                out_buf.extend_from_slice(&chunk);
                if let Some(cb) = on_chunk.as_deref_mut() {
                    emit_valid_utf8_chunks(&mut stream_utf8_buf, &chunk, cb);
                }
            }

            if let Some(status) = child
                .try_wait()
                .context("failed waiting for ollama process")
                .map_err(|err| runtime_error("ollama", err.to_string()))?
            {
                break status;
            }

            if start.elapsed() >= timeout {
                let _ = child.kill();
                let kill_start = Instant::now();
                loop {
                    if let Some(_status) = child
                        .try_wait()
                        .context("failed waiting for ollama process")
                        .map_err(|err| runtime_error("ollama", err.to_string()))?
                    {
                        break;
                    }
                    if kill_start.elapsed() >= Duration::from_secs(1) {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                return Err(timeout_error(
                    "ollama",
                    format!("timed out after {timeout_secs}s (set ADL_TIMEOUT_SECS to override)"),
                ));
            }

            std::thread::sleep(Duration::from_millis(10));
        };

        while let Ok(chunk) = rx.try_recv() {
            out_buf.extend_from_slice(&chunk);
            if let Some(cb) = on_chunk.as_deref_mut() {
                emit_valid_utf8_chunks(&mut stream_utf8_buf, &chunk, cb);
            }
        }

        out_handle
            .join()
            .map_err(|_| panic_error("ollama", "stdout reader thread panicked"))?
            .context("failed reading ollama stdout")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        while let Ok(chunk) = rx.try_recv() {
            out_buf.extend_from_slice(&chunk);
            if let Some(cb) = on_chunk.as_deref_mut() {
                emit_valid_utf8_chunks(&mut stream_utf8_buf, &chunk, cb);
            }
        }
        let err_buf = err_handle
            .join()
            .map_err(|_| panic_error("ollama", "stderr reader thread panicked"))?
            .context("failed reading ollama stderr")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        if !status.success() {
            let stderr = String::from_utf8_lossy(&err_buf);
            return Err(runtime_error(
                "ollama",
                format!(
                    "ollama run failed (exit={:?}): {}",
                    status.code(),
                    stderr.trim()
                ),
            ));
        }

        let stdout = String::from_utf8(out_buf)
            .context("ollama output was not valid UTF-8")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;
        Ok(stdout)
    }
}

/// Runtime-only CLI execution. The public legacy `OllamaProvider` layout stays
/// unchanged, including its literal-construction and streaming compatibility.
pub(super) struct RuntimeOllamaProvider {
    model: String,
    deadline: Duration,
}
const RUNTIME_CLI_MAX_OUTPUT_BYTES: usize = 4_194_304;

impl RuntimeOllamaProvider {
    pub(super) fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        if runtime_output_cap(&spec.config)?.is_some() {
            return Err(unsupported_capability_error(
                "local_ollama",
                "the local CLI cannot enforce a Runtime token-output cap",
            ));
        }
        let seconds = match spec.config.get("timeout_secs") {
            None => 30,
            Some(value) => value
                .as_u64()
                .filter(|seconds| *seconds > 0)
                .ok_or_else(|| {
                    invalid_config(
                        "local_ollama",
                        "Runtime CLI timeout_secs must be a positive integer",
                    )
                })?
                .min(30),
        };
        Ok(Self {
            model: target.provider_model_id.clone(),
            deadline: Duration::from_secs(seconds),
        })
    }

    fn complete_with_command(
        &self,
        mut command: Command,
        prompt: &str,
        output_limit: usize,
    ) -> Result<String> {
        // A caller cannot make the worker allocate arbitrarily large stdin copies.
        if prompt.len() > RUNTIME_CLI_MAX_OUTPUT_BYTES {
            return Err(invalid_config(
                "local_ollama",
                "Runtime CLI input exceeds byte limit",
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| runtime_error("local_ollama", "Runtime CLI supervisor unavailable"))?;
        runtime.block_on(async {
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            let mut child = tokio::process::Command::from(command)
                .kill_on_drop(true)
                .spawn()
                .map_err(|_| runtime_error("local_ollama", "Runtime CLI process unavailable"))?;
            // Keep the group identity even if the leader exits before descendants.
            let process_group = child.id();
            let mut stdin = child.stdin.take().expect("piped Runtime CLI stdin");
            let stdout = child.stdout.take().expect("piped Runtime CLI stdout");
            let input = prompt.as_bytes().to_vec();
            let mut writer =
                tokio::spawn(async move {
                    stdin.write_all(&input).await.map_err(|_| {
                        runtime_error("local_ollama", "Runtime CLI input write failed")
                    })?;
                    stdin.shutdown().await.map_err(|_| {
                        runtime_error("local_ollama", "Runtime CLI input close failed")
                    })
                });
            let mut reader = tokio::spawn(async move {
                let mut output = Vec::new();
                stdout
                    .take(output_limit as u64 + 1)
                    .read_to_end(&mut output)
                    .await
                    .map_err(|_| runtime_error("local_ollama", "Runtime CLI output read failed"))?;
                if output.len() > output_limit {
                    return Err(runtime_error_non_retryable(
                        "local_ollama",
                        "Runtime CLI output exceeds byte limit",
                    ));
                }
                Ok(output)
            });
            // Both pipes are independently scheduled under the SAME deadline.
            // A child which never reads stdin cannot block before the timer starts.
            let result = tokio::time::timeout(self.deadline, async {
                let output = (&mut reader)
                    .await
                    .map_err(|_| runtime_error("local_ollama", "Runtime CLI reader failed"))??;
                (&mut writer)
                    .await
                    .map_err(|_| runtime_error("local_ollama", "Runtime CLI writer failed"))??;
                let status = child
                    .wait()
                    .await
                    .map_err(|_| runtime_error("local_ollama", "Runtime CLI wait failed"))?;
                if !status.success() {
                    return Err(runtime_error_non_retryable(
                        "local_ollama",
                        "Runtime CLI exited unsuccessfully; stderr redacted",
                    ));
                }
                String::from_utf8(output).map_err(|_| {
                    runtime_error_non_retryable("local_ollama", "Runtime CLI output is not UTF-8")
                })
            })
            .await;
            writer.abort();
            reader.abort();
            // Clean up descendants on success too: background children must not
            // survive a completed Runtime invocation. No unrelated process scans.
            #[cfg(unix)]
            if let Some(pid) = process_group {
                // SAFETY: CommandExt created this invocation's isolated process group.
                unsafe {
                    libc::kill(-(pid as i32), libc::SIGKILL);
                }
            }
            let _ = child.kill().await;
            // Await cancellation of both pipe tasks before releasing the worker.
            if !writer.is_finished() {
                let _ = writer.await;
            }
            if !reader.is_finished() {
                let _ = reader.await;
            }
            match result {
                Err(_) => Err(timeout_error(
                    "local_ollama",
                    "Runtime CLI deadline exceeded",
                )),
                Ok(result) => result,
            }
        })
    }
}
impl Provider for RuntimeOllamaProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let mut command = Command::new(ollama_bin());
        command.arg("run").arg(&self.model);
        self.complete_with_command(command, prompt, RUNTIME_CLI_MAX_OUTPUT_BYTES)
    }
}

fn emit_valid_utf8_chunks(pending: &mut Vec<u8>, chunk: &[u8], on_chunk: &mut dyn FnMut(&str)) {
    pending.extend_from_slice(chunk);
    loop {
        match std::str::from_utf8(pending) {
            Ok(text) => {
                if !text.is_empty() {
                    on_chunk(text);
                }
                pending.clear();
                break;
            }
            Err(err) if err.valid_up_to() > 0 => {
                let valid_up_to = err.valid_up_to();
                let text = std::str::from_utf8(&pending[..valid_up_to])
                    .expect("valid_up_to should delimit valid UTF-8");
                on_chunk(text);
                pending.drain(..valid_up_to);
            }
            Err(err) if err.error_len().is_some() => {
                let invalid_len = err.error_len().unwrap_or(1);
                on_chunk("\u{FFFD}");
                pending.drain(..invalid_len);
            }
            Err(_) => break,
        }
    }
}

impl Provider for OllamaProvider {
    /// Execute a prompt through `ollama run` and return complete stdout text.
    fn complete(&self, prompt: &str) -> Result<String> {
        self.complete_streaming(prompt, None)
    }

    fn complete_stream(&self, prompt: &str, on_chunk: &mut dyn FnMut(&str)) -> Result<String> {
        self.complete_streaming(prompt, Some(on_chunk))
    }
}

fn ollama_bin() -> PathBuf {
    // Allows tests (and power users) to override the binary path.
    // Defaults to `ollama` on PATH.
    std::env::var_os("ADL_OLLAMA_BIN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("ollama"))
}

pub(crate) fn cfg_f32(cfg: &HashMap<String, Value>, key: &str) -> Option<f32> {
    cfg.get(key).and_then(|v| {
        if let Some(f) = v.as_f64() {
            Some(f as f32)
        } else if let Some(i) = v.as_i64() {
            Some(i as f32)
        } else if let Some(s) = v.as_str() {
            s.parse::<f32>().ok()
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // PVF: deterministic, release-required local CLI safety proof for #855.
    // Fake local subprocesses only; no Ollama executable, provider credentials,
    // network calls or hosted inference. Unix group cleanup is exercised below.
    #[test]
    fn runtime_cli_constructor_rejects_token_cap_and_preserves_legacy_literal_api() {
        let mut spec = adl::ProviderSpec {
            id: None,
            profile: None,
            kind: "local_ollama".into(),
            base_url: None,
            default_model: Some("fixture".into()),
            config: HashMap::new(),
        };
        let _legacy = OllamaProvider {
            model: "fixture".into(),
            temperature: None,
        };
        spec.config.insert("runtime_max_attempts".into(), 1.into());
        spec.config.insert("timeout_secs".into(), 1.into());
        let target =
            provider_substrate::provider_invocation_target_v1("fixture", &spec, None).unwrap();
        assert_eq!(
            RuntimeOllamaProvider::from_target(&spec, &target)
                .unwrap()
                .deadline,
            Duration::from_secs(1)
        );
        spec.config
            .insert("runtime_max_output_tokens".into(), 256.into());
        let error = build_provider_for_id("fixture", &spec, None)
            .err()
            .expect("CLI must not claim token enforcement");
        assert_eq!(failure_category(&error), "unsupported_capability");
        spec.config.remove("runtime_max_output_tokens");
        spec.config.insert("timeout_secs".into(), 0.into());
        assert!(build_provider_for_id("fixture", &spec, None).is_err());
        spec.config.remove("runtime_max_attempts");
        assert!(
            build_provider_for_id("fixture", &spec, None).is_ok(),
            "legacy constructor behavior remains intact"
        );
    }

    #[cfg(unix)]
    #[test]
    fn runtime_cli_deadline_covers_stalled_stdin_larger_than_pipe_capacity() {
        let provider = RuntimeOllamaProvider {
            model: "fixture".into(),
            deadline: Duration::from_millis(100),
        };
        let mut command = Command::new("/bin/sh");
        command.args(["-c", "sleep 30"]);
        let started = Instant::now();
        let error = provider
            .complete_with_command(
                command,
                &"x".repeat(1_048_576),
                RUNTIME_CLI_MAX_OUTPUT_BYTES,
            )
            .unwrap_err();
        assert_eq!(failure_category(&error), "timeout");
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "stdin write must be inside the deadline"
        );
    }

    #[cfg(unix)]
    #[test]
    fn runtime_cli_kills_descendants_after_the_group_leader_exits_and_reaps_leader() {
        let dir = tempfile::tempdir().unwrap();
        let escaped = dir.path().join("escaped");
        let leader_path = dir.path().join("leader-pid");
        let provider = RuntimeOllamaProvider {
            model: "fixture".into(),
            deadline: Duration::from_millis(100),
        };
        let mut command = Command::new("/bin/sh");
        command
            .args([
                "-c",
                "printf '%s' $$ > \"$2\"; (sleep 0.4; printf escaped > \"$1\") & exit 0",
                "fixture",
            ])
            .arg(&escaped)
            .arg(&leader_path);
        let error = provider
            .complete_with_command(command, "", RUNTIME_CLI_MAX_OUTPUT_BYTES)
            .unwrap_err();
        assert_eq!(failure_category(&error), "timeout");
        let leader: i32 = fs::read_to_string(leader_path).unwrap().parse().unwrap();
        // SAFETY: exact child PID emitted by this test's fixture, no host process scan.
        assert_eq!(
            unsafe { libc::waitpid(leader, std::ptr::null_mut(), libc::WNOHANG) },
            -1,
            "supervisor must reap its child"
        );
        thread::sleep(Duration::from_millis(500));
        assert!(
            !escaped.exists(),
            "a descendant must not execute after the invocation deadline"
        );
    }

    #[cfg(unix)]
    #[test]
    fn runtime_cli_buffers_success_bounds_output_and_redacts_stderr() {
        let provider = RuntimeOllamaProvider {
            model: "fixture".into(),
            deadline: Duration::from_secs(2),
        };
        let prompt = "snow ☃\n".repeat(16_384);
        assert_eq!(
            provider
                .complete_with_command(
                    Command::new("/bin/cat"),
                    &prompt,
                    RUNTIME_CLI_MAX_OUTPUT_BYTES
                )
                .unwrap(),
            prompt
        );
        let mut oversized = Command::new("/bin/sh");
        oversized.args(["-c", "head -c 4194305 /dev/zero"]);
        let error = provider
            .complete_with_command(oversized, "", RUNTIME_CLI_MAX_OUTPUT_BYTES)
            .unwrap_err();
        assert_eq!(failure_category(&error), "invalid_response");
        assert!(error.to_string().contains("output exceeds byte limit"));
        let mut failed = Command::new("/bin/sh");
        failed.args(["-c", "printf 'SECRET-CLI-STDERR' >&2; exit 9"]);
        let error = provider
            .complete_with_command(failed, "", RUNTIME_CLI_MAX_OUTPUT_BYTES)
            .unwrap_err();
        assert!(!error.to_string().contains("SECRET-CLI-STDERR"));
        assert_eq!(failure_category(&error), "invalid_response");
    }

    #[test]
    fn ollama_streaming_buffers_split_multibyte_utf8() {
        let mut pending = Vec::new();
        let mut chunks = Vec::new();
        emit_valid_utf8_chunks(&mut pending, "snow ".as_bytes(), &mut |chunk| {
            chunks.push(chunk.to_string())
        });
        emit_valid_utf8_chunks(&mut pending, &[0xE2, 0x98], &mut |chunk| {
            chunks.push(chunk.to_string())
        });
        assert!(chunks.iter().all(|chunk| !chunk.contains('\u{FFFD}')));
        assert_eq!(pending, vec![0xE2, 0x98]);
        emit_valid_utf8_chunks(&mut pending, &[0x83, b'!'], &mut |chunk| {
            chunks.push(chunk.to_string())
        });
        assert!(pending.is_empty());
        assert_eq!(chunks, vec!["snow ".to_string(), "☃!".to_string()]);

        emit_valid_utf8_chunks(&mut pending, &[0xFF], &mut |chunk| {
            chunks.push(chunk.to_string())
        });
        emit_valid_utf8_chunks(&mut pending, b" after invalid", &mut |chunk| {
            chunks.push(chunk.to_string())
        });
        assert!(pending.is_empty());
        assert_eq!(chunks[2], "\u{FFFD}");
        assert_eq!(chunks[3], " after invalid");
    }
}
