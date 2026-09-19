//! Private, instance-bound operator control. Never mount this on public HTTP.
use super::Service;
use anyhow::{ensure, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::Path,
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};

const SCHEMA: &str = "codefriend.host_control.v1";
const MAX_REQUEST: usize = 4096;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema: String,
    action: Action,
    instance: Option<String>,
    attempt: Option<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Action {
    Status,
    Drain,
    Resume,
}

/// One control listener per service instance. An existing socket is a hard stop;
/// its supervisor must establish that the former process stopped before cleanup.
pub struct ControlServer {
    listener: UnixListener,
    service: Service,
    instance: String,
    attempt: Option<String>,
    retired: BTreeSet<String>,
    owner: u32,
}
impl ControlServer {
    pub fn bind(service: Service, path: &Path) -> Result<Self> {
        let parent = fs::symlink_metadata(
            path.parent()
                .ok_or_else(|| anyhow::anyhow!("control_parent_required"))?,
        )?;
        let owner = fs::metadata(&service.0.config.root)?.uid();
        ensure!(
            parent.is_dir()
                && !parent.file_type().is_symlink()
                && parent.mode() & 0o077 == 0
                && parent.uid() == owner,
            "private_control_parent_required"
        );
        // bind itself excludes existing files/sockets, including dangling symlinks.
        let listener = UnixListener::bind(path)?;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
        let instance = rand::random::<[u8; 32]>()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        Ok(Self {
            listener,
            service,
            instance,
            attempt: None,
            retired: BTreeSet::new(),
            owner,
        })
    }
    pub async fn serve(mut self) -> Result<()> {
        loop {
            let (mut stream, _) = self.listener.accept().await?;
            let peer_uid = stream.peer_cred()?.uid();
            if peer_uid != self.owner && peer_uid != 0 {
                continue;
            }
            // Connections are serialized and bounded; only same-owner clients can
            // traverse the private directory. No caller-provided path or command.
            let _ =
                tokio::time::timeout(Duration::from_secs(2), self.connection(&mut stream)).await;
        }
    }
    async fn connection(&mut self, stream: &mut UnixStream) -> Result<()> {
        let mut bytes = Vec::new();
        let mut chunk = [0u8; 512];
        loop {
            let n = stream.read(&mut chunk).await?;
            if n == 0 {
                return Ok(());
            }
            ensure!(bytes.len() + n <= MAX_REQUEST, "control_request_too_large");
            bytes.extend_from_slice(&chunk[..n]);
            if bytes.contains(&b'\n') {
                break;
            }
        }
        let response = match serde_json::from_slice::<Request>(&bytes) {
            Ok(request) => match self.handle(request) {
                Ok(response) => response,
                Err(_) => json!({"schema":SCHEMA,"ok":false,"error":"control_request_rejected"}),
            },
            Err(_) => json!({"schema":SCHEMA,"ok":false,"error":"control_request_rejected"}),
        };
        let mut output = serde_json::to_vec(&response)?;
        output.push(b'\n');
        stream.write_all(&output).await?;
        stream.shutdown().await?;
        Ok(())
    }
    fn handle(&mut self, request: Request) -> Result<Value> {
        ensure!(request.schema == SCHEMA, "control_schema_mismatch");
        if !matches!(request.action, Action::Status) {
            ensure!(
                request.instance.as_deref() == Some(&self.instance),
                "stale_service_instance"
            );
            let attempt = request
                .attempt
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("attempt_required"))?;
            ensure!(
                attempt.len() == 32 && attempt.bytes().all(|b| b.is_ascii_hexdigit()),
                "invalid_attempt"
            );
            match request.action {
                Action::Drain => {
                    ensure!(!self.retired.contains(attempt), "retired_drain_attempt");
                    // Never evict retired IDs: exhaustion requires a new service
                    // instance rather than silently allowing an old command again.
                    ensure!(
                        self.attempt.is_some() || self.retired.len() < 1024,
                        "drain_attempt_capacity_exhausted"
                    );
                    ensure!(
                        self.attempt.as_deref().is_none_or(|a| a == attempt),
                        "another_drain_active"
                    );
                    self.service.begin_drain()?;
                    self.attempt = Some(attempt.to_owned());
                }
                Action::Resume => {
                    ensure!(
                        self.attempt.as_deref() == Some(attempt),
                        "drain_attempt_mismatch"
                    );
                    self.service.resume_admissions()?;
                    self.retired.insert(attempt.to_owned());
                    self.attempt = None;
                }
                Action::Status => unreachable!(),
            }
        }
        let safe = self.service.drained_without_payloads()?;
        let quiescent = self.service.quiescent_without_payloads()?;
        Ok(json!({"schema":SCHEMA,"ok":true,"service":"gateway",
            "instance":self.instance,"pid":std::process::id(),"candidate_revision":super::build_revision(),
            "attempt":self.attempt,"draining":self.attempt.is_some(),
            "drained_without_payloads":safe,"quiescent_without_payloads":quiescent}))
    }
}
