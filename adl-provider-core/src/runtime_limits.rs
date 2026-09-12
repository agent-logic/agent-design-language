//! Runtime-only limits. Absence preserves the existing ADL adapter defaults.
use super::*;

pub(super) fn runtime_bounded_calls(cfg: &HashMap<String, Value>) -> Result<bool> {
    match cfg.get("runtime_max_attempts") {
        None => Ok(false),
        Some(value) if value.as_u64() == Some(1) => Ok(true),
        Some(_) => Err(invalid_config(
            "runtime",
            "config.runtime_max_attempts must be 1",
        )),
    }
}
pub(super) fn runtime_output_cap(cfg: &HashMap<String, Value>) -> Result<Option<u64>> {
    match cfg.get("runtime_max_output_tokens") {
        None => Ok(None),
        Some(value) => match value.as_u64() {
            Some(cap) if (1..=32_768).contains(&cap) => Ok(Some(cap)),
            _ => Err(invalid_config(
                "runtime",
                "config.runtime_max_output_tokens must be an integer between 1 and 32768",
            )),
        },
    }
}
pub(super) fn bound_output_tokens(cfg: &HashMap<String, Value>, configured: u64) -> Result<u64> {
    match runtime_output_cap(cfg)? {
        Some(_) if configured == 0 => Err(invalid_config(
            "runtime",
            "configured output limit must be positive",
        )),
        Some(cap) => Ok(cap.min(configured)),
        None => Ok(configured),
    }
}

/// Capture at most 64 KiB in memory and kill/reap authentication on its deadline.
/// Stderr is discarded; neither subprocess diagnostics nor tokens enter errors.
pub(super) fn bounded_auth_output(mut command: Command, timeout: Duration) -> Result<Vec<u8>> {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| {
            invalid_config(
                "vertex_ai_gemini",
                "ADC token acquisition runtime unavailable",
            )
        })?;
    runtime.block_on(async move {
        use tokio::io::AsyncReadExt;
        let mut command = tokio::process::Command::from(command);
        command.kill_on_drop(true);
        let mut child = command.spawn().map_err(|_| {
            invalid_config(
                "vertex_ai_gemini",
                "ADC token acquisition process unavailable",
            )
        })?;
        let mut output = zeroize::Zeroizing::new(Vec::new());
        let result = tokio::time::timeout(timeout, async {
            let stdout = child.stdout.take().ok_or_else(|| {
                invalid_config(
                    "vertex_ai_gemini",
                    "ADC token acquisition output unavailable",
                )
            })?;
            stdout
                .take(65_537)
                .read_to_end(&mut output)
                .await
                .map_err(|_| {
                    invalid_config(
                        "vertex_ai_gemini",
                        "ADC token acquisition output read failed",
                    )
                })?;
            if output.len() > 65_536 {
                return Err(invalid_config(
                    "vertex_ai_gemini",
                    "ADC token acquisition output exceeds limit",
                ));
            }
            let status = child.wait().await.map_err(|_| {
                invalid_config("vertex_ai_gemini", "ADC token acquisition wait failed")
            })?;
            if !status.success() {
                return Err(invalid_config(
                    "vertex_ai_gemini",
                    "ADC token acquisition returned a non-zero status",
                ));
            }
            Ok(())
        })
        .await;
        if !matches!(result, Ok(Ok(()))) {
            // Terminate the process group too: a token helper may own descendants
            // retaining its stdout pipe after the immediate child exits.
            #[cfg(unix)]
            if let Some(pid) = child.id() {
                // SAFETY: this is the isolated process group created above, not the caller's group.
                unsafe {
                    libc::kill(-(pid as i32), libc::SIGKILL);
                }
            }
            let _ = child.kill().await;
            return match result {
                Err(_) => Err(timeout_error(
                    "vertex_ai_gemini",
                    "authentication deadline exceeded",
                )),
                Ok(Err(error)) => Err(error),
                Ok(Ok(())) => unreachable!(),
            };
        }
        Ok(std::mem::take(&mut *output))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    // PVF: deterministic local contract and bounded subprocess proof; no external
    // services or real credentials. Required for Runtime adapter limits in #855.
    #[test]
    fn runtime_limits_are_opt_in_strict_and_do_not_raise_existing_caps() {
        let mut cfg = HashMap::new();
        assert_eq!(runtime_output_cap(&cfg).unwrap(), None);
        assert_eq!(bound_output_tokens(&cfg, 220).unwrap(), 220);
        assert!(!runtime_bounded_calls(&cfg).unwrap());
        cfg.insert("runtime_max_output_tokens".into(), 256.into());
        assert_eq!(bound_output_tokens(&cfg, 1024).unwrap(), 256);
        assert_eq!(bound_output_tokens(&cfg, 128).unwrap(), 128);
        for invalid in [
            Value::Null,
            0.into(),
            (-1).into(),
            32_769.into(),
            "256".into(),
        ] {
            cfg.insert("runtime_max_output_tokens".into(), invalid);
            assert!(runtime_output_cap(&cfg).is_err());
        }
        for invalid in [Value::Null, 0.into(), 2.into(), "1".into()] {
            cfg.insert("runtime_max_attempts".into(), invalid);
            assert!(runtime_bounded_calls(&cfg).is_err());
        }
        cfg.insert("runtime_max_attempts".into(), 1.into());
        assert!(runtime_bounded_calls(&cfg).unwrap());
    }
    #[cfg(unix)]
    #[test]
    fn authentication_capture_bounds_time_output_and_redacts_errors() {
        let mut success = Command::new("/bin/sh");
        success.args(["-c", "printf fixture-token"]);
        assert_eq!(
            bounded_auth_output(success, Duration::from_secs(1)).unwrap(),
            b"fixture-token"
        );
        let mut sleeping = Command::new("/bin/sh");
        sleeping.args(["-c", "sleep 30"]);
        let started = Instant::now();
        let error = bounded_auth_output(sleeping, Duration::from_millis(50)).unwrap_err();
        assert_eq!(failure_category(&error), "timeout");
        assert!(started.elapsed() < Duration::from_secs(2));
        let mut oversized = Command::new("/bin/sh");
        oversized.args(["-c", "head -c 70000 /dev/zero"]);
        let error = bounded_auth_output(oversized, Duration::from_secs(1)).unwrap_err();
        assert!(error.to_string().contains("output exceeds limit"));
        assert!(!error.to_string().contains("fixture-token"));
    }
}
