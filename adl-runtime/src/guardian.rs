//! External Runtime v3 process guardian.
//!
//! This module owns the narrow OS-child boundary for
//! `adl-runtime-kernel serve --init <init-path> <continuity-path>`. It intentionally does not
//! become a platform service manager and does not supervise Runtime v2.

use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};
use std::time::{Duration, Instant};

use adl_resilience::capped_exponential_backoff;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::process::{Child, Command};
use tokio::task::JoinHandle;
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;

#[cfg(windows)]
use std::os::windows::io::AsRawHandle;

#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::JobObjects::{AssignProcessToJobObject, CreateJobObjectW, TerminateJobObject},
};

pub const GUARDIAN_SCHEMA: &str = "adl.runtime_v3.external_guardian.v2";
#[cfg(windows)]
const WINDOWS_FORCED_TERMINATION_EXIT_CODE: u32 = 0xAD1D_F0CE;
pub const GUARDIAN_LEASE_ADDRESS_ENV: &str = "ADL_RUNTIME_GUARDIAN_LEASE_ADDRESS";
pub const GUARDIAN_LEASE_TOKEN_ENV: &str = "ADL_RUNTIME_GUARDIAN_LEASE_TOKEN";
pub const GUARDIAN_REQUIRED_ENV: &str = "ADL_RUNTIME_GUARDIAN_REQUIRED";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardianConfig {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub restart_budget: u32,
    pub backoff_base_ms: u64,
    pub backoff_cap_ms: u64,
    pub healthy_window_ms: u64,
    pub child_shutdown_budget_ms: u64,
    pub shutdown_grace_ms: u64,
    pub lease_auth_timeout_ms: u64,
    pub lease_auth_attempts: u32,
    pub capture_max_bytes: u64,
    pub capture_drain_grace_ms: u64,
    pub configuration_exit_codes: Vec<i32>,
}

impl GuardianConfig {
    pub fn runtime_kernel(program: impl Into<PathBuf>, init_path: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: vec!["serve".to_string(), "--init".to_string(), init_path.into()],
            env: Vec::new(),
            restart_budget: 0,
            backoff_base_ms: 0,
            backoff_cap_ms: 0,
            healthy_window_ms: 0,
            child_shutdown_budget_ms: 0,
            shutdown_grace_ms: 0,
            lease_auth_timeout_ms: 0,
            lease_auth_attempts: 0,
            capture_max_bytes: 0,
            capture_drain_grace_ms: 0,
            configuration_exit_codes: Vec::new(),
        }
    }

    pub fn validate(&self) -> Result<(), GuardianConfigError> {
        if self.program.as_os_str().is_empty() {
            return Err(GuardianConfigError::MissingProgram);
        }
        if self
            .env
            .iter()
            .any(|(name, _)| name.is_empty() || name.contains('='))
        {
            return Err(GuardianConfigError::InvalidEnvironmentName);
        }
        if self.backoff_base_ms == 0 {
            return Err(GuardianConfigError::ZeroBackoff);
        }
        if self.backoff_cap_ms < self.backoff_base_ms {
            return Err(GuardianConfigError::BackoffCapBelowBase);
        }
        if self.healthy_window_ms == 0
            || self.child_shutdown_budget_ms == 0
            || self.shutdown_grace_ms == 0
            || self.lease_auth_timeout_ms == 0
            || self.lease_auth_attempts == 0
            || self.capture_max_bytes == 0
            || self.capture_drain_grace_ms == 0
        {
            return Err(GuardianConfigError::ZeroShutdownGrace);
        }
        if self.shutdown_grace_ms <= self.child_shutdown_budget_ms {
            return Err(GuardianConfigError::ShutdownGraceBelowChildBudget);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardianConfigError {
    MissingProgram,
    InvalidEnvironmentName,
    ZeroBackoff,
    BackoffCapBelowBase,
    ZeroShutdownGrace,
    ShutdownGraceBelowChildBudget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardianTerminalState {
    ExitedSuccessfully,
    ConfigurationExit,
    ShutdownCheckpointed,
    RestartBudgetExhausted,
    ShutdownForwarded,
    ShutdownForced,
    SpawnFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardianAttempt {
    pub attempt: u32,
    pub pid: Option<u32>,
    pub exit_code: Option<i32>,
    pub exit_status: Option<String>,
    pub unix_signal: Option<i32>,
    pub windows_ctrl_event: Option<u32>,
    pub forced_shutdown: bool,
    pub clean_checkpointed_shutdown: bool,
    pub stdout: String,
    pub stderr: String,
    pub reason_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardianOutcome {
    pub schema: String,
    pub terminal_state: GuardianTerminalState,
    pub attempts: u32,
    pub restarts: u32,
    pub attempts_detail: Vec<GuardianAttempt>,
}

impl GuardianOutcome {
    pub fn last_reason(&self) -> Option<&str> {
        self.attempts_detail
            .last()
            .map(|attempt| attempt.reason_code.as_str())
    }
}

pub async fn run_guardian(
    config: GuardianConfig,
    shutdown: CancellationToken,
) -> Result<GuardianOutcome, GuardianConfigError> {
    config.validate()?;
    let mut attempts_detail = Vec::new();
    let mut attempts = 0_u32;
    let mut restarts = 0_u32;
    let mut restart_budget_used = 0_u32;

    loop {
        attempts = attempts.saturating_add(1);
        let lease = match GuardianLease::bind().await {
            Ok(lease) => lease,
            Err(error) => {
                attempts_detail.push(GuardianAttempt {
                    attempt: attempts,
                    pid: None,
                    exit_code: None,
                    exit_status: None,
                    unix_signal: None,
                    windows_ctrl_event: None,
                    forced_shutdown: false,
                    clean_checkpointed_shutdown: false,
                    stdout: String::new(),
                    stderr: String::new(),
                    reason_code: format!("guardian_lease_failed:{error}"),
                });
                return Ok(outcome(
                    &config,
                    GuardianTerminalState::SpawnFailed,
                    attempts,
                    restarts,
                    attempts_detail,
                ));
            }
        };
        let spawned = GuardedChild::spawn(&config, &lease);
        let mut child = match spawned {
            Ok(child) => child,
            Err(error) => {
                attempts_detail.push(GuardianAttempt {
                    attempt: attempts,
                    pid: None,
                    exit_code: None,
                    exit_status: None,
                    unix_signal: None,
                    windows_ctrl_event: None,
                    forced_shutdown: false,
                    clean_checkpointed_shutdown: false,
                    stdout: String::new(),
                    stderr: String::new(),
                    reason_code: format!("spawn_failed:{error}"),
                });
                return Ok(outcome(
                    &config,
                    GuardianTerminalState::SpawnFailed,
                    attempts,
                    restarts,
                    attempts_detail,
                ));
            }
        };

        let pid = child.id();
        let attempt_started = Instant::now();
        let stdout = child
            .child
            .stdout
            .take()
            .map(|pipe| capture_pipe(pipe, config.capture_max_bytes))
            .unwrap_or_else(|| tokio::spawn(async { String::new() }));
        let stderr = child
            .child
            .stderr
            .take()
            .map(|pipe| capture_pipe(pipe, config.capture_max_bytes))
            .unwrap_or_else(|| tokio::spawn(async { String::new() }));
        let lease_task = tokio::spawn(lease.authenticate_and_hold(
            Duration::from_millis(config.lease_auth_timeout_ms),
            config.lease_auth_attempts,
        ));

        let attempt_exit = tokio::select! {
            _ = shutdown.cancelled() => {
                lease_task.abort();
                let wait_result = timeout(Duration::from_millis(config.shutdown_grace_ms), child.wait()).await;
                let (status, forced, clean, reason_code, signals) = match wait_result {
                    Ok(Ok(status)) => (
                        Some(status),
                        false,
                        true,
                        "shutdown_clean_checkpointed".to_string(),
                        AttemptSignals::default(),
                    ),
                    Ok(Err(error)) => {
                        let signals = child.force_shutdown();
                        (
                            None,
                            true,
                            false,
                            format!("wait_failed:{error}"),
                            signals,
                        )
                    }
                    Err(_) => {
                        let signals = child.force_shutdown();
                        let status = child.wait().await.ok();
                        (
                            status,
                            true,
                            false,
                            "shutdown_grace_exhausted_forced".to_string(),
                            signals,
                        )
                    }
                };
                let terminal = if forced {
                    GuardianTerminalState::ShutdownForced
                } else {
                    GuardianTerminalState::ShutdownCheckpointed
                };
                let exit_code = status.as_ref().and_then(ExitStatus::code);
                let signals =
                    signals.merge(status.as_ref().and_then(exit_signal).unwrap_or_default());
                let attempt = attempt_record(
                    &config,
                    attempts,
                    AttemptRecordMeta {
                        pid,
                        exit_code,
                        exit_status: status.as_ref().map(|status| format!("{status:?}")),
                        signals,
                        forced_shutdown: forced,
                        clean_checkpointed_shutdown: clean,
                        reason_code,
                    },
                    stdout,
                    stderr,
                ).await;
                attempts_detail.push(attempt);
                AttemptExit::Terminal(terminal)
            }
            status = child.wait() => {
                match status {
                    Ok(status) => {
                        let code = status.code();
                        let healthy_window = attempt_started.elapsed()
                            >= Duration::from_millis(config.healthy_window_ms);
                        let attempt_exit = classify_exit(&config, code, restart_budget_used, healthy_window);
                        let reason_code = reason_code_for_exit(&config, code, restart_budget_used, healthy_window);
                        let attempt = attempt_record(
                            &config,
                            attempts,
                            AttemptRecordMeta {
                                pid,
                                exit_code: code,
                                exit_status: Some(format!("{status:?}")),
                                signals: exit_signal(&status).unwrap_or_default(),
                                forced_shutdown: false,
                                clean_checkpointed_shutdown: false,
                                reason_code,
                            },
                            stdout,
                            stderr,
                        ).await;
                        attempts_detail.push(attempt);
                        attempt_exit
                    }
                    Err(error) => {
                        let attempt = attempt_record(
                            &config,
                            attempts,
                            AttemptRecordMeta {
                                pid,
                                exit_code: None,
                                exit_status: None,
                                signals: AttemptSignals::default(),
                                forced_shutdown: false,
                                clean_checkpointed_shutdown: false,
                                reason_code: format!("wait_failed:{error}"),
                            },
                            stdout,
                            stderr,
                        ).await;
                        attempts_detail.push(attempt);
                        AttemptExit::Terminal(GuardianTerminalState::SpawnFailed)
                    }
                }
            }
        };
        lease_task.abort();

        match attempt_exit {
            AttemptExit::Restart { reset_budget } => {
                restarts = restarts.saturating_add(1);
                restart_budget_used = if reset_budget {
                    1
                } else {
                    restart_budget_used.saturating_add(1)
                };
                tokio::select! {
                    _ = shutdown.cancelled() => {
                        attempts_detail.push(GuardianAttempt {
                            attempt: attempts,
                            pid: None,
                            exit_code: None,
                            exit_status: None,
                            unix_signal: None,
                            windows_ctrl_event: None,
                            forced_shutdown: false,
                            clean_checkpointed_shutdown: false,
                            stdout: String::new(),
                            stderr: String::new(),
                            reason_code: "shutdown_during_restart_backoff".to_string(),
                        });
                        return Ok(outcome(
                            &config,
                            GuardianTerminalState::ShutdownForwarded,
                            attempts,
                            restarts,
                            attempts_detail,
                        ));
                    }
                    _ = sleep(backoff(&config, restarts)) => {}
                }
            }
            AttemptExit::Terminal(terminal) => {
                return Ok(outcome(
                    &config,
                    terminal,
                    attempts,
                    restarts,
                    attempts_detail,
                ));
            }
        }
    }
}

#[cfg(unix)]
pub async fn run_guardian_with_os_signals(
    config: GuardianConfig,
) -> Result<GuardianOutcome, GuardianConfigError> {
    use tokio::signal::unix::{signal, SignalKind};

    let shutdown = CancellationToken::new();
    let signal_shutdown = shutdown.clone();
    let signal_task = tokio::spawn(async move {
        let mut interrupt = signal(SignalKind::interrupt()).ok();
        let mut terminate = signal(SignalKind::terminate()).ok();
        match (interrupt.as_mut(), terminate.as_mut()) {
            (Some(interrupt), Some(terminate)) => {
                tokio::select! {
                    _ = interrupt.recv() => {}
                    _ = terminate.recv() => {}
                }
            }
            (Some(interrupt), None) => {
                let _ = interrupt.recv().await;
            }
            (None, Some(terminate)) => {
                let _ = terminate.recv().await;
            }
            (None, None) => return,
        }
        signal_shutdown.cancel();
    });
    let outcome = run_guardian(config, shutdown).await;
    signal_task.abort();
    outcome
}

#[cfg(not(unix))]
pub async fn run_guardian_with_os_signals(
    config: GuardianConfig,
) -> Result<GuardianOutcome, GuardianConfigError> {
    #[cfg(windows)]
    {
        use tokio::signal::windows::{ctrl_break, ctrl_c, ctrl_close, ctrl_shutdown};

        let shutdown = CancellationToken::new();
        let signal_shutdown = shutdown.clone();
        let signal_task = tokio::spawn(async move {
            let mut ctrl_c = ctrl_c().ok();
            let mut ctrl_break = ctrl_break().ok();
            let mut ctrl_close = ctrl_close().ok();
            let mut ctrl_shutdown = ctrl_shutdown().ok();
            tokio::select! {
                _ = async {
                    if let Some(signal) = ctrl_c.as_mut() {
                        signal.recv().await;
                    } else {
                        std::future::pending::<()>().await;
                    }
                } => {}
                _ = async {
                    if let Some(signal) = ctrl_break.as_mut() {
                        signal.recv().await;
                    } else {
                        std::future::pending::<()>().await;
                    }
                } => {}
                _ = async {
                    if let Some(signal) = ctrl_close.as_mut() {
                        signal.recv().await;
                    } else {
                        std::future::pending::<()>().await;
                    }
                } => {}
                _ = async {
                    if let Some(signal) = ctrl_shutdown.as_mut() {
                        signal.recv().await;
                    } else {
                        std::future::pending::<()>().await;
                    }
                } => {}
            }
            signal_shutdown.cancel();
        });
        let outcome = run_guardian(config, shutdown).await;
        signal_task.abort();
        outcome
    }
    #[cfg(not(windows))]
    {
        run_guardian(config, CancellationToken::new()).await
    }
}

struct GuardianLease {
    listener: TcpListener,
    address: String,
    token: String,
}

impl GuardianLease {
    async fn bind() -> std::io::Result<Self> {
        let listener = TcpListener::bind(("127.0.0.1", 0)).await?;
        let address = listener.local_addr()?.to_string();
        let mut token = [0_u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut token);
        Ok(Self {
            listener,
            address,
            token: URL_SAFE_NO_PAD.encode(token),
        })
    }

    async fn authenticate_and_hold(self, auth_timeout: Duration, max_attempts: u32) {
        for _ in 0..max_attempts {
            let Ok(Ok((mut stream, peer))) = timeout(auth_timeout, self.listener.accept()).await
            else {
                continue;
            };
            if !peer.ip().is_loopback() {
                continue;
            }
            let mut supplied = vec![0_u8; self.token.len()];
            let authenticated = timeout(auth_timeout, stream.read_exact(&mut supplied))
                .await
                .is_ok_and(|result| result.is_ok())
                && bool::from(supplied.as_slice().ct_eq(self.token.as_bytes()));
            if !authenticated {
                continue;
            }
            if stream.write_all(b"ok").await.is_err() {
                return;
            }
            let mut closed = [0_u8; 1];
            let _ = stream.read(&mut closed).await;
            return;
        }
    }
}

struct GuardedChild {
    child: Child,
    #[cfg(windows)]
    job: WindowsJob,
}

impl GuardedChild {
    fn spawn(config: &GuardianConfig, lease: &GuardianLease) -> std::io::Result<Self> {
        let mut command = Command::new(&config.program);
        command
            .args(&config.args)
            .envs(config.env.iter().map(|(name, value)| (name, value)))
            .env(GUARDIAN_LEASE_ADDRESS_ENV, &lease.address)
            .env(GUARDIAN_LEASE_TOKEN_ENV, &lease.token)
            .env(GUARDIAN_REQUIRED_ENV, "1")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        #[cfg(unix)]
        command.process_group(0);
        #[cfg(windows)]
        let job = WindowsJob::create()?;
        #[cfg(windows)]
        let mut child = command.spawn()?;
        #[cfg(not(windows))]
        let child = command.spawn()?;
        #[cfg(windows)]
        {
            if let Err(error) = job.assign(&child) {
                let _ = child.start_kill();
                return Err(error);
            }
            Ok(Self { child, job })
        }
        #[cfg(not(windows))]
        {
            Ok(Self { child })
        }
    }

    fn id(&self) -> Option<u32> {
        self.child.id()
    }

    async fn wait(&mut self) -> std::io::Result<ExitStatus> {
        self.child.wait().await
    }

    fn force_shutdown(&mut self) -> AttemptSignals {
        #[cfg(windows)]
        {
            force_shutdown(&mut self.child, &self.job)
        }
        #[cfg(not(windows))]
        {
            force_shutdown(&mut self.child)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AttemptExit {
    Restart { reset_budget: bool },
    Terminal(GuardianTerminalState),
}

fn classify_exit(
    config: &GuardianConfig,
    code: Option<i32>,
    restarts: u32,
    healthy_window: bool,
) -> AttemptExit {
    if code == Some(0) {
        return AttemptExit::Terminal(GuardianTerminalState::ExitedSuccessfully);
    }
    if code
        .map(|code| config.configuration_exit_codes.contains(&code))
        .unwrap_or(false)
    {
        return AttemptExit::Terminal(GuardianTerminalState::ConfigurationExit);
    }
    let budget_restarts = if healthy_window { 0 } else { restarts };
    if budget_restarts >= config.restart_budget {
        AttemptExit::Terminal(GuardianTerminalState::RestartBudgetExhausted)
    } else {
        AttemptExit::Restart {
            reset_budget: healthy_window,
        }
    }
}

fn reason_code_for_exit(
    config: &GuardianConfig,
    code: Option<i32>,
    restarts: u32,
    healthy_window: bool,
) -> String {
    if code == Some(0) {
        return "child_exited_successfully".to_string();
    }
    if code
        .map(|code| config.configuration_exit_codes.contains(&code))
        .unwrap_or(false)
    {
        return "configuration_exit".to_string();
    }
    let budget_restarts = if healthy_window { 0 } else { restarts };
    if budget_restarts >= config.restart_budget {
        "restart_budget_exhausted".to_string()
    } else if healthy_window {
        "child_failed_after_healthy_window_restart_scheduled".to_string()
    } else {
        "child_failed_restart_scheduled".to_string()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct AttemptSignals {
    unix_signal: Option<i32>,
    windows_ctrl_event: Option<u32>,
}

impl AttemptSignals {
    fn merge(self, other: Self) -> Self {
        Self {
            unix_signal: self.unix_signal.or(other.unix_signal),
            windows_ctrl_event: self.windows_ctrl_event.or(other.windows_ctrl_event),
        }
    }
}

struct AttemptRecordMeta {
    pid: Option<u32>,
    exit_code: Option<i32>,
    exit_status: Option<String>,
    signals: AttemptSignals,
    forced_shutdown: bool,
    clean_checkpointed_shutdown: bool,
    reason_code: String,
}

async fn attempt_record(
    config: &GuardianConfig,
    attempt: u32,
    meta: AttemptRecordMeta,
    stdout: JoinHandle<String>,
    stderr: JoinHandle<String>,
) -> GuardianAttempt {
    let (stdout, stderr) = tokio::join!(
        bounded_capture(stdout, Duration::from_millis(config.capture_drain_grace_ms)),
        bounded_capture(stderr, Duration::from_millis(config.capture_drain_grace_ms)),
    );
    GuardianAttempt {
        attempt,
        pid: meta.pid,
        exit_code: meta.exit_code,
        exit_status: meta.exit_status,
        unix_signal: meta.signals.unix_signal,
        windows_ctrl_event: meta.signals.windows_ctrl_event,
        forced_shutdown: meta.forced_shutdown,
        clean_checkpointed_shutdown: meta.clean_checkpointed_shutdown,
        stdout: stdout.0,
        stderr: stderr.0,
        reason_code: meta.reason_code,
    }
}

async fn bounded_capture(mut capture: JoinHandle<String>, grace: Duration) -> (String, bool) {
    match timeout(grace, &mut capture).await {
        Ok(result) => (result.unwrap_or_default(), false),
        Err(_) => {
            capture.abort();
            ("<adl_guardian_capture_deadline_exceeded>".to_owned(), true)
        }
    }
}

fn capture_pipe<R>(mut pipe: R, max_bytes: u64) -> JoinHandle<String>
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut bytes = Vec::new();
        let mut truncated = false;
        let mut scratch = [0_u8; 8192];
        loop {
            match pipe.read(&mut scratch).await {
                Ok(0) => break,
                Ok(read) => {
                    let remaining = (max_bytes as usize).saturating_sub(bytes.len());
                    if remaining > 0 {
                        bytes.extend_from_slice(&scratch[..read.min(remaining)]);
                    }
                    if read > remaining {
                        truncated = true;
                    }
                }
                Err(error) => return format!("capture_failed:{error}"),
            }
        }
        let mut text = String::from_utf8_lossy(&bytes).into_owned();
        if truncated {
            text.push_str("\n<adl_guardian_output_truncated>");
        }
        text
    })
}

fn backoff(config: &GuardianConfig, restarts: u32) -> Duration {
    capped_exponential_backoff(config.backoff_base_ms, config.backoff_cap_ms, restarts)
}

fn outcome(
    _config: &GuardianConfig,
    terminal_state: GuardianTerminalState,
    attempts: u32,
    restarts: u32,
    attempts_detail: Vec<GuardianAttempt>,
) -> GuardianOutcome {
    GuardianOutcome {
        schema: GUARDIAN_SCHEMA.to_string(),
        terminal_state,
        attempts,
        restarts,
        attempts_detail,
    }
}

#[cfg(windows)]
struct WindowsJob {
    handle: HANDLE,
}

#[cfg(windows)]
unsafe impl Send for WindowsJob {}

#[cfg(windows)]
impl WindowsJob {
    fn create() -> std::io::Result<Self> {
        let handle = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        if handle.is_null() {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(Self { handle })
        }
    }

    fn assign(&self, child: &Child) -> std::io::Result<()> {
        let assigned =
            unsafe { AssignProcessToJobObject(self.handle, child.as_raw_handle() as HANDLE) };
        if assigned == 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn terminate(&self) -> bool {
        unsafe { TerminateJobObject(self.handle, WINDOWS_FORCED_TERMINATION_EXIT_CODE) != 0 }
    }
}

#[cfg(windows)]
impl Drop for WindowsJob {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

#[cfg(unix)]
fn force_shutdown(child: &mut Child) -> AttemptSignals {
    child
        .id()
        .and_then(|pid| i32::try_from(pid).ok())
        .filter(|process_group| unsafe { libc::kill(-*process_group, libc::SIGKILL) == 0 })
        .map(|_| AttemptSignals {
            unix_signal: Some(libc::SIGKILL),
            windows_ctrl_event: None,
        })
        .unwrap_or_else(|| {
            let _ = child.start_kill();
            AttemptSignals::default()
        })
}

#[cfg(windows)]
fn force_shutdown(child: &mut Child, job: &WindowsJob) -> AttemptSignals {
    if job.terminate() {
        AttemptSignals {
            unix_signal: None,
            windows_ctrl_event: None,
        }
    } else {
        let _ = child.start_kill();
        AttemptSignals::default()
    }
}

#[cfg(not(any(unix, windows)))]
fn force_shutdown(child: &mut Child) -> AttemptSignals {
    let _ = child.start_kill();
    AttemptSignals::default()
}

#[cfg(unix)]
fn exit_signal(status: &std::process::ExitStatus) -> Option<AttemptSignals> {
    use std::os::unix::process::ExitStatusExt;
    status.signal().map(|signal| AttemptSignals {
        unix_signal: Some(signal),
        windows_ctrl_event: None,
    })
}

#[cfg(not(unix))]
fn exit_signal(_status: &std::process::ExitStatus) -> Option<AttemptSignals> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_nanos();
            let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join(".csdlc")
                .join("evidence")
                .join("5344")
                .join("work")
                .join("guardian-tests")
                .join(format!("adl-guardian-{name}-{unique}"));
            fs::create_dir_all(&root).expect("test root");
            Self(root)
        }

        fn path(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }

        #[cfg(unix)]
        fn script(&self, name: &str, body: &str) -> PathBuf {
            let path = self.path(name);
            fs::write(&path, body).expect("write script");
            let mut permissions = fs::metadata(&path).expect("metadata").permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&path, permissions).expect("chmod");
            path
        }

        fn rust_child(&self, name: &str, source: &str) -> PathBuf {
            let source_path = self.path(&format!("{name}.rs"));
            let executable = self.path(&format!("{name}{}", std::env::consts::EXE_SUFFIX));
            fs::write(&source_path, source).expect("write Rust child");
            let status = std::process::Command::new("rustc")
                .arg(&source_path)
                .arg("-o")
                .arg(&executable)
                .status()
                .expect("run rustc");
            assert!(status.success(), "compile Rust child");
            executable
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn test_config(program: impl Into<PathBuf>) -> GuardianConfig {
        let mut config = GuardianConfig::runtime_kernel(program, "runtime-init.toml");
        config.restart_budget = 3;
        config.backoff_base_ms = 1;
        config.backoff_cap_ms = 10;
        config.healthy_window_ms = 100;
        config.child_shutdown_budget_ms = 100;
        config.shutdown_grace_ms = 200;
        config.lease_auth_timeout_ms = 100;
        config.lease_auth_attempts = 3;
        config.capture_max_bytes = 65_536;
        config.capture_drain_grace_ms = 100;
        config.configuration_exit_codes = vec![64];
        config
    }

    #[test]
    fn runtime_kernel_config_preserves_single_init_path() {
        let config = test_config("adl-runtime-kernel");
        assert_eq!(config.args, ["serve", "--init", "runtime-init.toml"]);
        assert_eq!(config.configuration_exit_codes, [64]);
        assert_eq!(config.validate(), Ok(()));
    }

    #[test]
    fn invalid_configuration_and_spawn_failure_fail_closed() {
        let base = || test_config("adl-runtime-kernel");
        let mut invalid_env = base();
        invalid_env
            .env
            .push(("BAD=NAME".to_string(), "value".to_string()));
        let mut missing_program = base();
        missing_program.program.clear();
        let mut zero_backoff = base();
        zero_backoff.backoff_base_ms = 0;
        let mut inverted_backoff = base();
        inverted_backoff.backoff_base_ms = inverted_backoff.backoff_cap_ms + 1;
        let mut zero_shutdown_grace = base();
        zero_shutdown_grace.shutdown_grace_ms = 0;
        let mut insufficient_shutdown_grace = base();
        insufficient_shutdown_grace.shutdown_grace_ms = 100;

        for (config, expected) in [
            (invalid_env, GuardianConfigError::InvalidEnvironmentName),
            (missing_program, GuardianConfigError::MissingProgram),
            (zero_backoff, GuardianConfigError::ZeroBackoff),
            (inverted_backoff, GuardianConfigError::BackoffCapBelowBase),
            (zero_shutdown_grace, GuardianConfigError::ZeroShutdownGrace),
            (
                insufficient_shutdown_grace,
                GuardianConfigError::ShutdownGraceBelowChildBudget,
            ),
        ] {
            assert_eq!(config.validate(), Err(expected));
        }
    }

    #[tokio::test]
    async fn missing_child_program_is_reported_without_restart() {
        let root = TestRoot::new("missing-program");
        let mut config = test_config(root.path("adl-guardian-program-that-does-not-exist"));
        config.restart_budget = 5;

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(outcome.terminal_state, GuardianTerminalState::SpawnFailed);
        assert_eq!(outcome.attempts, 1);
        assert_eq!(outcome.restarts, 0);
        assert!(outcome
            .last_reason()
            .is_some_and(|reason| reason.starts_with("spawn_failed:")));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn captured_output_is_bounded() {
        let root = TestRoot::new("bounded-output");
        let script = root.script("noisy.sh", "#!/bin/sh\nyes x | head -c 70000\nexit 0\n");
        let mut config = test_config(script);
        config.args.clear();
        let capture_max_bytes = config.capture_max_bytes;

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ExitedSuccessfully
        );
        let stdout = &outcome.attempts_detail[0].stdout;
        assert!(stdout.len() <= capture_max_bytes as usize + 40);
        assert!(stdout.contains("<adl_guardian_output_truncated>"));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn injects_configured_environment() {
        let root = TestRoot::new("env");
        let script = root.script(
            "env.sh",
            "#!/bin/sh\necho \"$ADL_GUARDIAN_ENV_PROBE\"\nexit 0\n",
        );
        let mut config = test_config(script);
        config.args.clear();
        config
            .env
            .push(("ADL_GUARDIAN_ENV_PROBE".to_string(), "injected".to_string()));

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ExitedSuccessfully
        );
        assert_eq!(outcome.attempts_detail[0].stdout, "injected\n");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn restarts_until_child_exits_successfully_and_captures_output() {
        let root = TestRoot::new("recover");
        let counter = root.path("counter");
        let script = root.script(
            "recover.sh",
            &format!(
                "#!/bin/sh\ncount=$(cat {counter} 2>/dev/null || echo 0)\ncount=$((count + 1))\necho $count > {counter}\necho stdout-$count\necho stderr-$count >&2\nif [ \"$count\" -lt 3 ]; then exit 7; fi\nexit 0\n",
                counter = counter.display()
            ),
        );
        let mut config = test_config(script);
        config.args.clear();
        config.restart_budget = 3;
        config.backoff_base_ms = 1;
        config.backoff_cap_ms = 1;

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ExitedSuccessfully
        );
        assert_eq!(outcome.attempts, 3);
        assert_eq!(outcome.restarts, 2);
        assert_eq!(outcome.last_reason(), Some("child_exited_successfully"));
        assert!(outcome
            .attempts_detail
            .iter()
            .any(|attempt| attempt.stdout.contains("stdout-3")));
        assert!(outcome
            .attempts_detail
            .iter()
            .any(|attempt| attempt.stderr.contains("stderr-2")));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn restart_budget_exhaustion_is_terminal() {
        let root = TestRoot::new("budget");
        let script = root.script("fail.sh", "#!/bin/sh\necho failed >&2\nexit 7\n");
        let mut config = test_config(script);
        config.args.clear();
        config.restart_budget = 2;
        config.backoff_base_ms = 1;
        config.backoff_cap_ms = 1;

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::RestartBudgetExhausted
        );
        assert_eq!(outcome.attempts, 3);
        assert_eq!(outcome.restarts, 2);
        assert_eq!(outcome.last_reason(), Some("restart_budget_exhausted"));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn restart_budget_resets_after_healthy_window() {
        let root = TestRoot::new("healthy-reset");
        let counter = root.path("counter");
        let script = root.script(
            "healthy-reset.sh",
            &format!(
                "#!/bin/sh\ncount=$(cat {counter} 2>/dev/null || echo 0)\ncount=$((count + 1))\necho $count > {counter}\nsleep 0.08\nif [ \"$count\" -lt 3 ]; then exit 7; fi\nexit 0\n",
                counter = counter.display()
            ),
        );
        let mut config = test_config(script);
        config.args.clear();
        config.restart_budget = 1;
        config.backoff_base_ms = 1;
        config.backoff_cap_ms = 1;
        config.healthy_window_ms = 50;

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ExitedSuccessfully
        );
        assert_eq!(outcome.attempts, 3);
        assert_eq!(outcome.restarts, 2);
        assert!(outcome.attempts_detail[..2]
            .iter()
            .all(|attempt| attempt.reason_code
                == "child_failed_after_healthy_window_restart_scheduled"));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn configuration_exit_does_not_restart() {
        let root = TestRoot::new("config");
        let script = root.script("config.sh", "#!/bin/sh\necho bad-config >&2\nexit 64\n");
        let mut config = test_config(script);
        config.args.clear();
        config.restart_budget = 5;

        let outcome = run_guardian(config, CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ConfigurationExit
        );
        assert_eq!(outcome.attempts, 1);
        assert_eq!(outcome.restarts, 0);
        assert_eq!(outcome.last_reason(), Some("configuration_exit"));
    }

    #[tokio::test]
    async fn shutdown_closes_authenticated_lease_and_reaps_child() {
        let root = TestRoot::new("shutdown");
        let ready = root.path("ready");
        let child = root.rust_child(
            "lease-child",
            r#"
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let address = std::env::var("ADL_RUNTIME_GUARDIAN_LEASE_ADDRESS").unwrap();
    let token = std::env::var("ADL_RUNTIME_GUARDIAN_LEASE_TOKEN").unwrap();
    let mut stream = TcpStream::connect(address).unwrap();
    stream.write_all(token.as_bytes()).unwrap();
    let mut acknowledgement = [0_u8; 2];
    stream.read_exact(&mut acknowledgement).unwrap();
    assert_eq!(&acknowledgement, b"ok");
    std::fs::write(std::env::args().nth(1).unwrap(), b"ready").unwrap();
    let mut closed = [0_u8; 1];
    assert_eq!(stream.read(&mut closed).unwrap(), 0);
}
"#,
        );
        let mut config = test_config(child);
        config.args = vec![ready.to_string_lossy().into_owned()];
        config.shutdown_grace_ms = 2_000;
        let shutdown = CancellationToken::new();
        let cancel = shutdown.clone();

        let task = tokio::spawn(run_guardian(config, shutdown));
        let deadline = std::time::Instant::now() + Duration::from_secs(2);
        while !ready.exists() && std::time::Instant::now() < deadline {
            sleep(Duration::from_millis(10)).await;
        }
        assert!(ready.exists(), "child must authenticate its Guardian lease");
        cancel.cancel();
        let outcome = task.await.unwrap().unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ShutdownCheckpointed
        );
        assert_eq!(outcome.attempts, 1);
        assert_eq!(outcome.restarts, 0);
        assert_eq!(outcome.last_reason(), Some("shutdown_clean_checkpointed"));
        let attempt = &outcome.attempts_detail[0];
        assert_eq!(attempt.unix_signal, None);
        assert_eq!(attempt.windows_ctrl_event, None);
        assert!(attempt.clean_checkpointed_shutdown);
        assert!(!attempt.forced_shutdown);
        assert!(attempt.exit_status.is_some());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn child_exit_never_signals_process_group_after_reap() {
        let root = TestRoot::new("no-post-reap-signal");
        let descendant_file = root.path("descendant-pid");
        let descendant_ready = root.path("descendant-ready");
        let script = root.script(
            "descendant.sh",
            &format!(
                "#!/bin/sh\n(trap '' TERM; echo ready > {descendant_ready}; while true; do sleep 1; done) &\necho $! > {descendant_file}\nwhile [ ! -f {descendant_ready} ]; do sleep 0.01; done\nexit 0\n",
                descendant_file = descendant_file.display(),
                descendant_ready = descendant_ready.display()
            ),
        );
        let mut config = test_config(script);
        config.args.clear();

        let outcome = timeout(
            Duration::from_secs(2),
            run_guardian(config, CancellationToken::new()),
        )
        .await
        .expect("guardian capture must be bounded")
        .unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ExitedSuccessfully
        );
        assert_eq!(outcome.attempts_detail[0].unix_signal, None);
        assert!(!outcome.attempts_detail[0].forced_shutdown);
        assert!(!outcome.attempts_detail[0].clean_checkpointed_shutdown);
        let descendant = fs::read_to_string(descendant_file)
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        assert_eq!(
            unsafe { libc::kill(descendant, 0) },
            0,
            "the child-owned descendant should survive because Guardian no longer signals by reaped pgid"
        );
        assert!(outcome.attempts_detail[0]
            .stdout
            .contains("<adl_guardian_capture_deadline_exceeded>"));
        let _ = unsafe { libc::kill(descendant, libc::SIGKILL) };
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn shutdown_timeout_forces_process_group_before_reap() {
        let root = TestRoot::new("shutdown-force");
        let started = root.path("started");
        let script = root.script(
            "ignore-term.sh",
            &format!(
                "#!/bin/sh\ntrap '' TERM\necho started > {started}\nwhile true; do sleep 1; done\n",
                started = started.display()
            ),
        );
        let mut config = test_config(script);
        config.args.clear();
        config.child_shutdown_budget_ms = 10;
        config.shutdown_grace_ms = 50;
        let shutdown = CancellationToken::new();
        let cancel = shutdown.clone();

        let task = tokio::spawn(run_guardian(config, shutdown));
        let deadline = std::time::Instant::now() + Duration::from_secs(2);
        while !started.exists() && std::time::Instant::now() < deadline {
            sleep(Duration::from_millis(10)).await;
        }
        cancel.cancel();
        let outcome = task.await.unwrap().unwrap();

        assert_eq!(
            outcome.terminal_state,
            GuardianTerminalState::ShutdownForced
        );
        assert!(outcome.attempts_detail[0].forced_shutdown);
        assert_eq!(outcome.attempts_detail[0].unix_signal, Some(libc::SIGKILL));
    }

    #[cfg(windows)]
    #[test]
    fn windows_job_object_handle_can_be_owned_by_guardian_task() {
        fn assert_send<T: Send>() {}

        assert_send::<WindowsJob>();
    }
}
