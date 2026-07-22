//! Bounded async transport for conductor-approved workcell task operations.

mod model;

use futures::future::BoxFuture;
pub use model::*;
use std::collections::BTreeMap;
use std::path::{Component, Path};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};

const REQUEST_DIGEST_DOMAIN: &[u8] = b"adl.workcell-task.request.v1\0";
const CONTEXT_DIGEST_DOMAIN: &[u8] = b"adl.workcell-task.context.v1\0";

pub trait AuthorityVerifier: Send + Sync {
    fn verify<'a>(
        &'a self,
        request: &'a TaskRequest,
    ) -> BoxFuture<'a, Result<(), AuthorityFailure>>;
}

pub trait TaskTransport: Send + Sync {
    fn execute<'a>(
        &'a self,
        request: &'a TaskRequest,
    ) -> BoxFuture<'a, Result<TransportReceipt, TransportFailure>>;

    fn observe<'a>(
        &'a self,
        task: &'a TaskRef,
    ) -> BoxFuture<'a, Result<TaskObservation, TransportFailure>>;
}

type CachedResult = Result<TaskReceipt, TaskTransportError>;
type OperationSlot = Arc<Mutex<Option<(String, CachedResult)>>>;

pub struct TaskAdapter<T, A> {
    transport: Arc<T>,
    authority: Arc<A>,
    limits: AdapterLimits,
    operations: Mutex<BTreeMap<String, OperationSlot>>,
    terminal_tasks: Mutex<BTreeMap<String, TaskStatus>>,
}

impl<T, A> TaskAdapter<T, A>
where
    T: TaskTransport + 'static,
    A: AuthorityVerifier + 'static,
{
    pub fn new(transport: Arc<T>, authority: Arc<A>, limits: AdapterLimits) -> Self {
        Self {
            transport,
            authority,
            limits,
            operations: Mutex::new(BTreeMap::new()),
            terminal_tasks: Mutex::new(BTreeMap::new()),
        }
    }

    pub async fn execute(&self, mut request: TaskRequest) -> CachedResult {
        normalize_and_validate(&mut request, &self.limits)?;
        let digest = request_digest(&request)?;
        let slot = {
            let mut operations = self.operations.lock().await;
            if operations.len() >= self.limits.max_idempotency_entries
                && !operations.contains_key(&request.idempotency_key)
            {
                return Err(TaskTransportError::resource_limit());
            }
            operations
                .entry(request.idempotency_key.clone())
                .or_insert_with(|| Arc::new(Mutex::new(None)))
                .clone()
        };

        let mut cached = slot.lock().await;
        if let Some((previous_digest, result)) = &*cached {
            if previous_digest != &digest {
                return Err(TaskTransportError::idempotency_collision());
            }
            return result.clone();
        }

        let result = self.dispatch(&request, &digest).await;
        *cached = Some((digest, result.clone()));
        result
    }

    pub async fn observe(&self, request: TaskRequest) -> CachedResult {
        if !matches!(request.operation, TaskOperation::Inspect { .. }) {
            return Err(TaskTransportError::invalid_request(
                "observe requires an inspect operation",
            ));
        }
        self.execute(request).await
    }

    async fn dispatch(&self, request: &TaskRequest, digest: &str) -> CachedResult {
        self.reject_after_terminal(request).await?;
        let deadline = Duration::from_millis(request.deadline_ms);
        let future = async {
            self.authority
                .verify(request)
                .await
                .map_err(TaskTransportError::from_authority)?;
            let transport = self
                .transport
                .execute(request)
                .await
                .map_err(TaskTransportError::from_transport)?;
            self.finish_receipt(request, digest, transport).await
        };
        match timeout(deadline, future).await {
            Ok(result) => result,
            Err(_) => Err(TaskTransportError::indeterminate()),
        }
    }

    async fn finish_receipt(
        &self,
        request: &TaskRequest,
        digest: &str,
        mut transport: TransportReceipt,
    ) -> CachedResult {
        transport.evidence_refs.sort();
        transport.evidence_refs.dedup();
        if transport.evidence_refs.len() > self.limits.max_evidence_refs {
            return Err(TaskTransportError::resource_limit());
        }

        if let TaskOperation::Cancel { task } = &request.operation {
            let observation = self
                .transport
                .observe(task)
                .await
                .map_err(TaskTransportError::from_transport)?;
            transport.task = Some(observation.task.clone());
            transport.outcome = cancel_outcome(&observation.status);
            self.record_terminal(&observation).await;
        } else if let TaskOutcome::Observed(observation) = &transport.outcome {
            self.record_terminal(observation).await;
        } else if let (Some(task), Some(status)) = (&transport.task, transport.outcome.status()) {
            if status.is_terminal() {
                self.terminal_tasks
                    .lock()
                    .await
                    .insert(task.id.clone(), status);
            }
        }

        Ok(TaskReceipt {
            contract: TASK_ADAPTER_CONTRACT_VERSION.into(),
            idempotency_key: request.idempotency_key.clone(),
            request_digest: digest.into(),
            operation: request.operation.kind(),
            task: transport.task,
            outcome: transport.outcome,
            transport_timestamp_ms: transport.transport_timestamp_ms,
            evidence_refs: transport.evidence_refs,
        })
    }

    async fn reject_after_terminal(&self, request: &TaskRequest) -> Result<(), TaskTransportError> {
        let Some(task) = request.operation.task() else {
            return Ok(());
        };
        let terminal = self.terminal_tasks.lock().await.get(&task.id).cloned();
        if terminal.is_some()
            && matches!(
                request.operation,
                TaskOperation::Message { .. } | TaskOperation::Handoff { .. }
            )
        {
            return Err(TaskTransportError::terminal_task());
        }
        Ok(())
    }

    async fn record_terminal(&self, observation: &TaskObservation) {
        if observation.status.is_terminal() {
            self.terminal_tasks
                .lock()
                .await
                .insert(observation.task.id.clone(), observation.status.clone());
        }
    }
}

fn normalize_and_validate(
    request: &mut TaskRequest,
    limits: &AdapterLimits,
) -> Result<(), TaskTransportError> {
    if request.contract != TASK_ADAPTER_CONTRACT_VERSION
        || request.idempotency_key.trim().is_empty()
        || request.assignment_digest.trim().is_empty()
        || request.dependency_digest.trim().is_empty()
        || request.authority.issue == 0
        || request.authority.claim_id.trim().is_empty()
        || request.authority.claim_owner.trim().is_empty()
        || request.authority.branch.trim().is_empty()
        || request.authority.worktree.trim().is_empty()
        || request.authority.freshness_token.trim().is_empty()
        || request.caller.subject.trim().is_empty()
    {
        return Err(TaskTransportError::invalid_request(
            "required request authority is absent",
        ));
    }
    if request.idempotency_key.len() > limits.max_idempotency_key_bytes
        || request.deadline_ms == 0
        || request.deadline_ms > limits.max_deadline_ms
        || request.observed_unix_seconds >= request.authority.expires_unix_seconds
    {
        return Err(TaskTransportError::invalid_request(
            "request bounds or freshness are invalid",
        ));
    }
    normalize_paths(&mut request.authority.protected_paths)?;
    normalize_paths(&mut request.authority.write_paths)?;
    for path in &request.authority.write_paths {
        if !request
            .authority
            .protected_paths
            .iter()
            .any(|protected| path_contains(protected, path))
        {
            return Err(TaskTransportError::authority_denied());
        }
    }
    normalize_strings(&mut request.context.provenance);
    normalize_strings(&mut request.context.scope);
    normalize_strings(&mut request.context.validation);
    if request.context.provenance.is_empty()
        || request.context.scope.is_empty()
        || request.context.expected_output.trim().is_empty()
        || request.context.validation.is_empty()
        || request.context.freshness_token.trim().is_empty()
        || request.context.content.len() > limits.max_context_bytes
        || contains_secret_marker(&request.context.content)
        || context_digest(&request.context.content) != request.context.content_digest
    {
        return Err(TaskTransportError::invalid_context());
    }
    validate_operation(request)?;
    Ok(())
}

fn validate_operation(request: &TaskRequest) -> Result<(), TaskTransportError> {
    match &request.operation {
        TaskOperation::Create { client_task_key } if client_task_key.trim().is_empty() => Err(
            TaskTransportError::invalid_request("client task key is empty"),
        ),
        TaskOperation::Attach { task }
        | TaskOperation::Message { task }
        | TaskOperation::Handoff { task, .. }
        | TaskOperation::Inspect { task }
            if task.id.trim().is_empty() =>
        {
            Err(TaskTransportError::invalid_request("task id is empty"))
        }
        TaskOperation::Cancel { task } => {
            if task.id.trim().is_empty() {
                Err(TaskTransportError::invalid_request("task id is empty"))
            } else if !request.caller.may_cancel {
                Err(TaskTransportError::authority_denied())
            } else {
                Ok(())
            }
        }
        TaskOperation::Escalate { task, reason_code } => {
            if task.id.trim().is_empty() || reason_code.trim().is_empty() {
                Err(TaskTransportError::invalid_request(
                    "escalation is incomplete",
                ))
            } else if !request.caller.may_escalate {
                Err(TaskTransportError::authority_denied())
            } else {
                Ok(())
            }
        }
        TaskOperation::Handoff { output_ref, .. } if output_ref.trim().is_empty() => Err(
            TaskTransportError::invalid_request("handoff output reference is empty"),
        ),
        _ => Ok(()),
    }
}

fn normalize_paths(paths: &mut Vec<String>) -> Result<(), TaskTransportError> {
    for value in paths.iter() {
        let path = Path::new(value);
        if path.is_absolute()
            || value.trim().is_empty()
            || path
                .components()
                .any(|part| !matches!(part, Component::Normal(_)))
        {
            return Err(TaskTransportError::invalid_path());
        }
    }
    paths.sort();
    paths.dedup();
    Ok(())
}

fn normalize_strings(values: &mut Vec<String>) {
    values.sort();
    values.dedup();
}

fn path_contains(parent: &str, child: &str) -> bool {
    let parent: Vec<_> = parent.split('/').collect();
    let child: Vec<_> = child.split('/').collect();
    child.starts_with(&parent)
}

fn contains_secret_marker(content: &str) -> bool {
    let lower = content.to_ascii_lowercase();
    [
        "authorization:",
        "api_key=",
        "api-key=",
        "aws_secret_access_key",
        "bearer ",
        "token=",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}

fn request_digest(request: &TaskRequest) -> Result<String, TaskTransportError> {
    let bytes = serde_json::to_vec(request).map_err(|_| TaskTransportError::serialization())?;
    let mut hasher = blake3::Hasher::new();
    hasher.update(REQUEST_DIGEST_DOMAIN);
    hasher.update(&bytes);
    Ok(hasher.finalize().to_hex().to_string())
}

pub fn context_digest(content: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(CONTEXT_DIGEST_DOMAIN);
    hasher.update(content.as_bytes());
    hasher.finalize().to_hex().to_string()
}

fn cancel_outcome(status: &TaskStatus) -> TaskOutcome {
    match status {
        TaskStatus::Cancelled => TaskOutcome::Cancelled,
        TaskStatus::Completed => TaskOutcome::CompletedBeforeCancel,
        TaskStatus::Failed => TaskOutcome::CancelRejected,
        TaskStatus::Running | TaskStatus::Unknown => TaskOutcome::Indeterminate,
    }
}
