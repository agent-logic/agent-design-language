//! Hosted adapter over the common journey owner and existing operation admission.
use super::*;
use crate::codefriend::{
    architecture::artifact::{
        BoundaryPolicyArtifact, ChangeSetArtifact, RationaleSelectionArtifact, StructureArtifact,
    },
    evidence::hash,
    governance::artifact::PolicyArtifact,
    integration::journey::{self, Continuation, OwnedAdmissionJourneyOptions},
    publication,
    review::runner::FourPerspectiveReviewRun,
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PrepareRequest {
    boundary_policy: BoundaryPolicyArtifact,
    fitness_policy: PolicyArtifact,
}

#[derive(Deserialize)]
#[serde(tag = "stage", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum StepRequest {
    Impact {
        changes: ChangeSetArtifact,
    },
    Rationale {
        selection: RationaleSelectionArtifact,
    },
    Drift {
        baseline_operation: String,
    },
    PalaceComparison {
        baseline_operation: String,
    },
}

struct BaselineOwner {
    operation: String,
    store: crate::codefriend::evidence::store::Store,
    graph: StructureArtifact,
    review: crate::codefriend::evidence::contracts::ReviewRecord,
    root: PathBuf,
    expires_at: u64,
}
impl BaselineOwner {
    fn context(&self) -> journey::owned_baseline::OwnedBaseline<'_> {
        journey::owned_baseline::OwnedBaseline {
            operation: &self.operation,
            store: &self.store,
            graph: &self.graph,
            review: &self.review,
            baseline_root: &self.root,
            expires_at: self.expires_at,
        }
    }
    fn load(
        service: &Service,
        headers: &HeaderMap,
        operation: &str,
        subject: &str,
    ) -> ApiResult<Self> {
        let (credential, op) = owned(service, headers, operation)?;
        if credential.subject != subject {
            return Err(ApiError(StatusCode::UNAUTHORIZED, "unauthorized"));
        }
        let dir = service.dir(&credential.subject, operation);
        let work = dir.join("work");
        let run: FourPerspectiveReviewRun =
            internal(read_json(&dir.join("result.json"), MAX_RESULT))?;
        let graph: StructureArtifact =
            internal(read_json(&work.join("journey/structure.json"), MAX_RESULT))?;
        if run.run_id != operation
            || graph.record().admission != run.review_record.admission
            || graph.record().run.packet_id != op.packet_id
        {
            return Err(ApiError(
                StatusCode::CONFLICT,
                "journey_baseline_identity_changed",
            ));
        }
        internal(journey::owned_baseline::validate_graph_source(
            &work.join("journey"),
            &graph,
        ))?;
        let review = internal(journey::owned_baseline::original_review(
            &work.join("journey"),
        ))?;
        if review != run.review_record {
            return Err(ApiError(
                StatusCode::CONFLICT,
                "journey_baseline_review_changed",
            ));
        }
        let store = internal(crate::codefriend::evidence::store::Store::open(
            &work.join("evidence"),
            now,
        ))?;
        let owner = Self {
            operation: operation.into(),
            expires_at: op.expires_at.min(graph.record().admission.expires_at),
            graph,
            review,
            store,
            root: work.join("hosted-baselines"),
        };
        internal(owner.context().validate())?;
        Ok(owner)
    }
}

// All callers hold the service gate. Owner references are reconstructed per
// observation and kept live through callback output validation.
pub(super) fn with_owned_journey<T: Serialize>(
    service: &Service,
    headers: &HeaderMap,
    operation: &str,
    callback: impl FnOnce(&mut journey::Journey) -> ApiResult<T>,
) -> ApiResult<T> {
    with_selected_baseline(service, headers, operation, None, |j, _| callback(j))
}

fn with_selected_baseline<T: Serialize>(
    service: &Service,
    headers: &HeaderMap,
    operation: &str,
    selected: Option<&str>,
    callback: impl FnOnce(
        &mut journey::Journey,
        Option<&journey::owned_baseline::OwnedBaseline<'_>>,
    ) -> ApiResult<T>,
) -> ApiResult<T> {
    let (credential, _) = owned(service, headers, operation)?;
    let output = service
        .dir(&credential.subject, operation)
        .join("work/journey");
    let saved_drift = internal(journey::owned_baseline::operation(&output))?;
    let saved_palace = internal(journey::owned_palace::operation(&output))?;
    if saved_drift.is_some() && saved_palace.is_some() && saved_drift != saved_palace {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "journey_baseline_selection_changed",
        ));
    }
    let saved = saved_drift.or(saved_palace);
    if selected.is_some() && saved.as_deref().is_some_and(|s| Some(s) != selected) {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "journey_baseline_selection_changed",
        ));
    }
    let selected = selected.or(saved.as_deref());
    if selected == Some(operation) {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "journey_baseline_must_be_distinct",
        ));
    }
    let owner = selected
        .map(|id| BaselineOwner::load(service, headers, id, &credential.subject))
        .transpose()?;
    let context = owner.as_ref().map(BaselineOwner::context);
    let palace = palace_context(service, &output);
    let mut current = internal(journey::resume_with_owners(
        &output,
        context.as_ref(),
        Some(&palace),
    ))?;
    let value = callback(&mut current, context.as_ref())?;
    if internal(serde_json::to_vec(&value).map_err(Into::into))?.len() > MAX_RESULT {
        return Err(ApiError(StatusCode::CONFLICT, "journey_response_limit"));
    }
    internal(current.validate_owned_palace(context.as_ref(), &palace))?;
    internal(current.continue_with(Continuation::Status))?;
    if owned(service, headers, operation)?.0.subject != credential.subject {
        return Err(ApiError(StatusCode::UNAUTHORIZED, "unauthorized"));
    }
    if let Some(owner) = &owner {
        if owned(service, headers, &owner.operation)?.0.subject != credential.subject {
            return Err(ApiError(StatusCode::UNAUTHORIZED, "unauthorized"));
        }
        internal(owner.context().validate())?;
    }
    Ok(value)
}

fn owned(
    service: &Service,
    headers: &HeaderMap,
    operation: &str,
) -> ApiResult<(Credential, Operation)> {
    let credential = service.auth(headers)?;
    let op = service.operation(&credential, operation)?;
    if credential.mode != Mode::Hosted
        || op.status != Status::Complete
        || op.expires_at <= now()
        || op.candidate_revision != build::REVISION
    {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "journey_operation_not_current",
        ));
    }
    Ok((credential, op))
}

pub(super) async fn prepare(
    State(service): State<Service>,
    headers: HeaderMap,
    HttpPath(operation): HttpPath<String>,
    Json(request): Json<PrepareRequest>,
) -> ApiResult<Json<Value>> {
    let gate = service
        .0
        .gate
        .lock()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"))?;
    let (credential, op) = owned(&service, &headers, &operation)?;
    let work = service.dir(&credential.subject, &operation).join("work");
    let output = work.join("journey");
    let reservation = work.join("journey-reservation.json");
    let digest = internal(hash(&request))?;
    if reservation.exists() {
        let prior: String = internal(read_json(&reservation, 256))?;
        if prior != digest {
            return Err(ApiError(StatusCode::CONFLICT, "journey_request_changed"));
        }
    } else {
        if *gate {
            return Err(ApiError(
                StatusCode::SERVICE_UNAVAILABLE,
                "service_draining",
            ));
        }
        let completed_run: FourPerspectiveReviewRun = internal(read_json(
            &service
                .dir(&credential.subject, &operation)
                .join("result.json"),
            MAX_RESULT,
        ))?;
        // Reject invalid policies before reserving this operation's journey.
        if matches!(&request.boundary_policy, BoundaryPolicyArtifact::V2(_))
            != matches!(&request.fitness_policy, PolicyArtifact::V2(_))
        {
            return Err(ApiError(
                StatusCode::BAD_REQUEST,
                "journey_policy_version_mismatch",
            ));
        }

        request
            .boundary_policy
            .validate(&completed_run.review_record.admission)
            .map_err(|_| ApiError(StatusCode::BAD_REQUEST, "journey_boundary_policy_invalid"))?;
        request
            .fitness_policy
            .validate()
            .map_err(|_| ApiError(StatusCode::BAD_REQUEST, "journey_fitness_policy_invalid"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            internal(
                fs::set_permissions(&work, fs::Permissions::from_mode(0o700)).map_err(Into::into),
            )?;
        }
        internal(publication::write_json_create_only(&reservation, &digest))?;
        let journey = internal(journey::prepare_owned_admission(
            OwnedAdmissionJourneyOptions {
                store: work.join("evidence"),
                output: output.clone(),
                owner_root: work.clone(),
                review_root: work.join("review"),
                packet_id: op.packet_id,
                admission_digest: completed_run.review_record.admission.digest.clone(),
                operation_id: operation.clone(),
                candidate_revision: op.candidate_revision,
                expires_at: op.expires_at,
                completed_run,
                boundary_policy: request.boundary_policy,
                fitness_policy: request.fitness_policy,
            },
        ))?;
        drop(journey);
    }
    // An incomplete reservation is observed through the owner; never re-run it.
    let value = with_owned_journey(&service, &headers, &operation, |j| {
        internal(serde_json::to_value(j.manifest()).map_err(Into::into))
    })?;
    Ok(Json(value))
}

pub(super) async fn status(
    State(service): State<Service>,
    headers: HeaderMap,
    HttpPath(operation): HttpPath<String>,
) -> ApiResult<Json<Value>> {
    let _gate = service
        .0
        .gate
        .lock()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"))?;
    Ok(Json(with_owned_journey(
        &service,
        &headers,
        &operation,
        |j| internal(serde_json::to_value(j.manifest()).map_err(Into::into)),
    )?))
}

pub(super) async fn step(
    State(service): State<Service>,
    headers: HeaderMap,
    HttpPath(operation): HttpPath<String>,
    Json(request): Json<StepRequest>,
) -> ApiResult<Json<Value>> {
    let gate = service
        .0
        .gate
        .lock()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"))?;
    if *gate {
        return Err(ApiError(
            StatusCode::SERVICE_UNAVAILABLE,
            "service_draining",
        ));
    }
    let selected = match &request {
        StepRequest::Drift { baseline_operation }
        | StepRequest::PalaceComparison { baseline_operation } => Some(baseline_operation.clone()),
        _ => None,
    };
    let value = with_selected_baseline(
        &service,
        &headers,
        &operation,
        selected.as_deref(),
        |j, baseline| {
            let result = match request {
                StepRequest::Impact { changes } => {
                    j.continue_with(Continuation::Impact { changes })
                }
                StepRequest::Rationale { selection } => {
                    j.continue_with(Continuation::Rationale { selection })
                }
                StepRequest::PalaceComparison { .. } => j.continue_owned_palace(
                    baseline.ok_or(ApiError(StatusCode::CONFLICT, "journey_baseline_missing"))?,
                    &palace_context(&service, j.output()),
                ),
                StepRequest::Drift { .. } => j.continue_owned_drift(
                    baseline.ok_or(ApiError(StatusCode::CONFLICT, "journey_baseline_missing"))?,
                ),
            };
            result.map_err(|_| ApiError(StatusCode::CONFLICT, "journey_step_rejected"))?;
            internal(serde_json::to_value(j.manifest()).map_err(Into::into))
        },
    )?;
    Ok(Json(value))
}

/// Fixed native graph projection for browser selectors; no caller-selected path.
pub(super) async fn graph(
    State(service): State<Service>,
    headers: HeaderMap,
    HttpPath(operation): HttpPath<String>,
) -> ApiResult<Json<Value>> {
    let _gate = service
        .0
        .gate
        .lock()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"))?;
    Ok(Json(with_owned_journey(
        &service,
        &headers,
        &operation,
        |journey| {
            let graph = journey
                .graph_artifact()
                .ok_or(ApiError(StatusCode::CONFLICT, "journey_graph_unavailable"))?;
            internal(serde_json::to_value(graph).map_err(Into::into))
        },
    )?))
}

/// Fixed report names only; resume revalidates retained owner artifacts first.
pub(super) async fn artifact(
    State(service): State<Service>,
    headers: HeaderMap,
    HttpPath((operation, stage)): HttpPath<(String, String)>,
) -> ApiResult<Json<Value>> {
    let file = match stage.as_str() {
        "structure" => "structure.json",
        "fitness" => "fitness.json",
        "impact" => "impact.json",
        "rationale" => "rationale.json",
        "drift" => "drift.json",
        "palace_comparison" => "palace_comparison.json",
        _ => {
            return Err(ApiError(
                StatusCode::NOT_FOUND,
                "journey_artifact_not_found",
            ))
        }
    };
    let _gate = service
        .0
        .gate
        .lock()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"))?;
    Ok(Json(with_owned_journey(
        &service,
        &headers,
        &operation,
        |journey| {
            if journey
                .manifest()
                .stages
                .get(&stage)
                .and_then(|s| s.artifact.as_deref())
                != Some(file)
            {
                return Err(ApiError(
                    StatusCode::CONFLICT,
                    "journey_artifact_unavailable",
                ));
            }
            internal(read_json(&journey.output().join(file), MAX_RESULT))
        },
    )?))
}

fn palace_context(
    service: &Service,
    output: &std::path::Path,
) -> journey::owned_palace::AuthorityContext {
    journey::owned_palace::AuthorityContext {
        root: service.0.config.root.join("palace-authority"),
        palace_root: output
            .parent()
            .expect("owned Journey has work parent")
            .join("palace"),
    }
}
