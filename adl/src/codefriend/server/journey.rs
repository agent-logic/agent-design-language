//! Hosted adapter over the common journey owner and existing operation admission.
use super::*;
use crate::codefriend::{
    architecture::{impact, rationale, structure},
    evidence::hash,
    governance::local as fitness,
    integration::journey::{self, Continuation, OwnedAdmissionJourneyOptions},
    publication,
    review::runner::FourPerspectiveReviewRun,
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PrepareRequest {
    boundary_policy: structure::BoundaryPolicy,
    fitness_policy: fitness::Policy,
}

#[derive(Deserialize)]
#[serde(tag = "stage", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum StepRequest {
    Impact {
        changes: impact::ChangeSet,
    },
    Rationale {
        selection: rationale::RationaleSelection,
    },
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

fn response(
    service: &Service,
    headers: &HeaderMap,
    operation: &str,
    subject: &str,
    value: Value,
) -> ApiResult<Json<Value>> {
    if internal(serde_json::to_vec(&value).map_err(Into::into))?.len() > MAX_RESULT {
        return Err(ApiError(StatusCode::CONFLICT, "journey_response_limit"));
    }
    let (current, _) = owned(service, headers, operation)?;
    if current.subject != subject {
        return Err(ApiError(StatusCode::UNAUTHORIZED, "unauthorized"));
    }
    Ok(Json(value))
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
    let journey = internal(journey::resume(&output))?;
    let value = internal(serde_json::to_value(journey.manifest()).map_err(Into::into))?;
    response(&service, &headers, &operation, &credential.subject, value)
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
    let (credential, _) = owned(&service, &headers, &operation)?;
    let journey = internal(journey::resume(
        &service
            .dir(&credential.subject, &operation)
            .join("work/journey"),
    ))?;
    response(
        &service,
        &headers,
        &operation,
        &credential.subject,
        internal(serde_json::to_value(journey.manifest()).map_err(Into::into))?,
    )
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
    let (credential, _) = owned(&service, &headers, &operation)?;
    let mut journey = internal(journey::resume(
        &service
            .dir(&credential.subject, &operation)
            .join("work/journey"),
    ))?;
    let step = match request {
        StepRequest::Impact { changes } => Continuation::Impact { changes },
        StepRequest::Rationale { selection } => Continuation::Rationale { selection },
    };
    journey
        .continue_with(step)
        .map_err(|_| ApiError(StatusCode::CONFLICT, "journey_step_rejected"))?;
    response(
        &service,
        &headers,
        &operation,
        &credential.subject,
        internal(serde_json::to_value(journey.manifest()).map_err(Into::into))?,
    )
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
    let (credential, _) = owned(&service, &headers, &operation)?;
    let journey = internal(journey::resume(
        &service
            .dir(&credential.subject, &operation)
            .join("work/journey"),
    ))?;
    let graph = journey
        .graph()
        .ok_or(ApiError(StatusCode::CONFLICT, "journey_graph_unavailable"))?;
    response(
        &service,
        &headers,
        &operation,
        &credential.subject,
        internal(serde_json::to_value(graph).map_err(Into::into))?,
    )
}
