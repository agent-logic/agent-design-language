//! Runtime Chronosense service for integrated temporal capture.
//!
//! This module turns the earlier Chronosense contracts into a reusable runtime
//! service. It intentionally keeps the first integration small: callers provide
//! epoch-millisecond runtime timestamps and receive validated UTC/local/lifetime
//! frames backed by the existing `TemporalContext` contract.

use anyhow::{anyhow, Context, Result};
use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use super::{
    ChronosenseFoundation, IdentityProfile, TemporalContext, CHRONOSENSE_CLOCK_STACK_SCHEMA,
    CHRONOSENSE_RUNTIME_SERVICE_SCHEMA, CHRONOSENSE_TIME_SYNC_STATUS_SCHEMA,
};

const NTPD_RS_STATUS_ENV: &str = "ADL_CSM_NTPD_RS_STATUS";
const NTPD_RS_CTL_ENV: &str = "ADL_CSM_NTPD_RS_CTL";
const NTPD_RS_CTL_TIMEOUT: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChronosenseRuntimeServiceConfig {
    pub schema_version: String,
    pub timezone: String,
    pub identity: Option<IdentityProfile>,
    pub started_at_epoch_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChronosenseClockStack {
    pub schema_version: String,
    pub utc_timestamp_rfc3339: String,
    pub local_timestamp_rfc3339: String,
    pub timezone: String,
    pub utc_offset: String,
    pub lifetime_elapsed_ms: u128,
    pub monotonic_elapsed_ms: u128,
    pub reference_frames: Vec<String>,
    pub temporal_context: TemporalContext,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChronosenseTimeSyncStatus {
    pub schema_version: String,
    pub substrate: String,
    pub source: String,
    pub mode: String,
    pub health: String,
    pub confidence: String,
    pub drift_status: String,
    pub failure_state: Option<String>,
    pub reason: String,
    pub observed_at_rfc3339: String,
    pub poll_command: String,
    pub port_policy: String,
    pub parsed_offset_seconds: Option<f64>,
    pub parsed_uncertainty_seconds: Option<f64>,
    pub raw_summary: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChronosenseRuntimeService {
    config: ChronosenseRuntimeServiceConfig,
}

impl ChronosenseRuntimeServiceConfig {
    pub fn utc(started_at_epoch_ms: u128) -> Self {
        Self {
            schema_version: CHRONOSENSE_RUNTIME_SERVICE_SCHEMA.to_string(),
            timezone: "UTC".to_string(),
            identity: None,
            started_at_epoch_ms,
        }
    }

    pub fn with_identity(
        timezone: impl Into<String>,
        identity: IdentityProfile,
        started_at_epoch_ms: u128,
    ) -> Self {
        Self {
            schema_version: CHRONOSENSE_RUNTIME_SERVICE_SCHEMA.to_string(),
            timezone: timezone.into(),
            identity: Some(identity),
            started_at_epoch_ms,
        }
    }
}

impl ChronosenseRuntimeService {
    pub fn new(config: ChronosenseRuntimeServiceConfig) -> Result<Self> {
        if config.schema_version != CHRONOSENSE_RUNTIME_SERVICE_SCHEMA {
            return Err(anyhow!(
                "unsupported chronosense runtime service schema version '{}'",
                config.schema_version
            ));
        }
        let started_at = utc_datetime_from_epoch_millis(config.started_at_epoch_ms)?;
        TemporalContext::from_now(started_at, &config.timezone, config.identity.as_ref())
            .with_context(|| "invalid chronosense runtime service configuration")?;
        Ok(Self { config })
    }

    pub fn config(&self) -> &ChronosenseRuntimeServiceConfig {
        &self.config
    }

    pub fn foundation(&self) -> ChronosenseFoundation {
        ChronosenseFoundation::bounded_v088()
    }

    pub fn capture_epoch_millis(&self, epoch_ms: u128) -> Result<ChronosenseClockStack> {
        let now_utc = utc_datetime_from_epoch_millis(epoch_ms)?;
        let temporal_context = TemporalContext::from_now(
            now_utc,
            &self.config.timezone,
            self.config.identity.as_ref(),
        )
        .with_context(|| "failed to capture chronosense temporal context")?;
        let elapsed_ms = epoch_ms.saturating_sub(self.config.started_at_epoch_ms);

        Ok(ChronosenseClockStack {
            schema_version: CHRONOSENSE_CLOCK_STACK_SCHEMA.to_string(),
            utc_timestamp_rfc3339: now_utc.to_rfc3339(),
            local_timestamp_rfc3339: temporal_context.local_timestamp_rfc3339.clone(),
            timezone: temporal_context.timezone.clone(),
            utc_offset: temporal_context.utc_offset.clone(),
            lifetime_elapsed_ms: elapsed_ms,
            monotonic_elapsed_ms: elapsed_ms,
            reference_frames: vec![
                "utc_epoch_millis".to_string(),
                "local_civil_time".to_string(),
                "runtime_lifetime".to_string(),
                "runtime_monotonic_elapsed".to_string(),
            ],
            temporal_context,
        })
    }

    pub fn rfc3339_for_epoch_millis(&self, epoch_ms: u128) -> Result<String> {
        Ok(self.capture_epoch_millis(epoch_ms)?.utc_timestamp_rfc3339)
    }
}

pub fn capture_ntpd_rs_time_sync_status() -> ChronosenseTimeSyncStatus {
    if std::env::var(NTPD_RS_STATUS_ENV).ok().as_deref() != Some("1") {
        return ChronosenseTimeSyncStatus::unavailable(
            "ntpd_rs_probe_disabled",
            format!("{NTPD_RS_STATUS_ENV}=1"),
            None,
        );
    }
    let command = std::env::var(NTPD_RS_CTL_ENV).unwrap_or_else(|_| "ntp-ctl".to_string());
    let command_label = ntpd_rs_command_label(&command);
    match ntpd_rs_status_output(&command) {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            chronosense_time_sync_status_from_ntp_ctl_output(&stdout, &command_label)
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            ChronosenseTimeSyncStatus::unavailable(
                "ntpd_rs_status_command_failed",
                command_label,
                first_non_empty_line(&stderr).map(|line| sanitize_probe_summary(&line)),
            )
        }
        Err(err) => {
            let failure_state = if err.kind() == std::io::ErrorKind::TimedOut {
                "ntpd_rs_status_command_timeout"
            } else {
                "ntpd_rs_status_command_unavailable"
            };
            ChronosenseTimeSyncStatus::unavailable(
                failure_state,
                command_label,
                Some(err.kind().to_string()),
            )
        }
    }
}

pub fn chronosense_time_sync_status_from_ntp_ctl_output(
    output: &str,
    command: &str,
) -> ChronosenseTimeSyncStatus {
    let summary = output
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(sanitize_probe_summary);
    if !output.contains("Synchronization status:") && !output.contains("ntp_source_offset_seconds")
    {
        return ChronosenseTimeSyncStatus::unavailable(
            "ntpd_rs_status_unrecognized",
            command.to_string(),
            summary,
        );
    }

    let missing_poll_count = ntpd_rs_metric_values(output, "ntp_source_unanswered_polls")
        .map(|value| value.max(0.0) as u64)
        .sum::<u64>()
        + output
            .lines()
            .filter_map(|line| {
                line.split_once("missing polls:")
                    .and_then(|(_, count)| count.trim().parse::<u64>().ok())
            })
            .sum::<u64>();
    let offset = ntpd_rs_metric_values(output, "ntp_source_offset_seconds")
        .next()
        .or_else(|| parse_first_source_offset(output));
    let uncertainty = ntpd_rs_metric_values(output, "ntp_source_uncertainty_seconds")
        .next()
        .or_else(|| parse_first_uncertainty(output));
    let (health, confidence, drift_status, failure_state, reason) = match offset {
        Some(value) if value.abs() <= 0.5 && missing_poll_count == 0 => (
            "synced",
            "high",
            "within_ntpd_rs_reported_bounds",
            None,
            "ntpd_rs_status_reports_active_sources",
        ),
        Some(_) => (
            "degraded",
            "medium",
            "offset_or_missing_polls_observed",
            Some("ntpd_rs_status_degraded"),
            "ntpd_rs_status_reports_degraded_sources",
        ),
        None => (
            "unknown",
            "low",
            "unknown",
            Some("ntpd_rs_status_offset_unavailable"),
            "ntpd_rs_status_missing_parseable_offset",
        ),
    };

    ChronosenseTimeSyncStatus {
        schema_version: CHRONOSENSE_TIME_SYNC_STATUS_SCHEMA.to_string(),
        substrate: "ntpd-rs".to_string(),
        source: "ntp-ctl status --format prometheus".to_string(),
        mode: "external_status_projection".to_string(),
        health: health.to_string(),
        confidence: confidence.to_string(),
        drift_status: drift_status.to_string(),
        failure_state: failure_state.map(ToString::to_string),
        reason: reason.to_string(),
        observed_at_rfc3339: Utc::now().to_rfc3339(),
        poll_command: command.to_string(),
        port_policy: "observes_ntpd_rs_status_only_no_csm_listener_on_udp_123".to_string(),
        parsed_offset_seconds: offset,
        parsed_uncertainty_seconds: uncertainty,
        raw_summary: summary,
    }
}

impl ChronosenseTimeSyncStatus {
    fn unavailable(
        reason: impl Into<String>,
        command: impl Into<String>,
        raw_summary: Option<String>,
    ) -> Self {
        Self {
            schema_version: CHRONOSENSE_TIME_SYNC_STATUS_SCHEMA.to_string(),
            substrate: "ntpd-rs".to_string(),
            source: "ntp-ctl status --format prometheus".to_string(),
            mode: "external_status_projection".to_string(),
            health: "unavailable".to_string(),
            confidence: "none".to_string(),
            drift_status: "unknown".to_string(),
            failure_state: Some(reason.into()),
            reason: "ntpd_rs_status_unavailable_without_csm_failure".to_string(),
            observed_at_rfc3339: Utc::now().to_rfc3339(),
            poll_command: command.into(),
            port_policy: "observes_ntpd_rs_status_only_no_csm_listener_on_udp_123".to_string(),
            parsed_offset_seconds: None,
            parsed_uncertainty_seconds: None,
            raw_summary,
        }
    }
}

fn ntpd_rs_status_output(command: &str) -> std::io::Result<Output> {
    let mut child = Command::new(command)
        .arg("status")
        .arg("--format")
        .arg("prometheus")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let deadline = Instant::now() + NTPD_RS_CTL_TIMEOUT;
    loop {
        if child.try_wait()?.is_some() {
            return child.wait_with_output();
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "ntpd-rs status probe timed out",
            ));
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

pub(super) fn ntpd_rs_command_label(command: &str) -> String {
    if command == "ntp-ctl"
        || Path::new(command)
            .file_name()
            .and_then(|name| name.to_str())
            == Some("ntp-ctl")
    {
        "ntp-ctl".to_string()
    } else {
        "custom_ntp_ctl".to_string()
    }
}

fn first_non_empty_line(raw: &str) -> Option<String> {
    raw.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToString::to_string)
}

pub(super) fn sanitize_probe_summary(raw: &str) -> String {
    let raw = if raw.contains("/Users/")
        || raw.contains("/home/")
        || raw.contains("/private/")
        || raw.contains("/var/folders/")
        || raw.contains("Authorization:")
        || raw.to_ascii_lowercase().contains("bearer ")
        || raw.to_ascii_lowercase().contains("aws_secret_access_key")
        || raw.contains("arn:aws:")
        || contains_cloud_account_identifier(raw)
    {
        "[redacted]".to_string()
    } else {
        raw.to_string()
    };
    raw.chars().take(160).collect()
}

fn contains_cloud_account_identifier(raw: &str) -> bool {
    let mut digits = 0usize;
    for byte in raw.bytes() {
        if byte.is_ascii_digit() {
            digits += 1;
        } else {
            if digits == 12 {
                return true;
            }
            digits = 0;
        }
    }
    digits == 12
}

fn parse_first_source_offset(output: &str) -> Option<f64> {
    output
        .lines()
        .filter(|line| line.contains(":123") && line.contains('±'))
        .find_map(|line| {
            let after_paren = line.split_once("):").map(|(_, rest)| rest.trim())?;
            let offset_raw = after_paren
                .trim_start_matches(':')
                .split('±')
                .next()?
                .trim();
            offset_raw.parse::<f64>().ok()
        })
}

fn ntpd_rs_metric_values<'a>(
    output: &'a str,
    metric_name: &'static str,
) -> impl Iterator<Item = f64> + 'a {
    output.lines().filter_map(move |line| {
        let line = line.trim();
        if line.starts_with('#') || !line.starts_with(metric_name) {
            return None;
        }
        line.rsplit_once(char::is_whitespace)
            .and_then(|(_, raw_value)| raw_value.parse::<f64>().ok())
    })
}

fn parse_first_uncertainty(output: &str) -> Option<f64> {
    output
        .lines()
        .filter(|line| line.contains(":123") && line.contains('±'))
        .find_map(|line| {
            let after_first = line.split_once('±')?.1;
            let uncertainty_raw = after_first
                .split(|ch: char| ch == '(' || ch.is_whitespace())
                .find(|part| !part.is_empty())?;
            uncertainty_raw.parse::<f64>().ok()
        })
}

fn utc_datetime_from_epoch_millis(epoch_ms: u128) -> Result<chrono::DateTime<Utc>> {
    let epoch_ms = i64::try_from(epoch_ms)
        .with_context(|| format!("epoch millis value {epoch_ms} exceeds supported i64 range"))?;
    Utc.timestamp_millis_opt(epoch_ms)
        .single()
        .ok_or_else(|| anyhow!("epoch millis value {epoch_ms} is not representable as UTC time"))
}
