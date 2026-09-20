//! Attach an already committed hosted export to the common journey.
use super::*;
use crate::codefriend::{
    evidence::contracts::Publication, integration::PublicationFormat, publication,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AttachRequest {
    format: PublicationFormat,
}

pub(super) async fn attach(
    State(service): State<Service>,
    headers: HeaderMap,
    HttpPath(operation): HttpPath<String>,
    Json(request): Json<AttachRequest>,
) -> ApiResult<Json<Value>> {
    let _gate = service
        .0
        .gate
        .lock()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"))?;
    let credential = service.auth(&headers)?;
    let op = service.operation(&credential, &operation)?;
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
    let dir = service.dir(&credential.subject, &operation);
    let bundle = publication_directory(&service, &credential.subject, &operation, request.format);
    let review = internal(publication::read_review(
        &dir.join("work/review/review-record.json"),
    ))?;
    let publication: Publication =
        internal(read_json(&bundle.join("publication.json"), MAX_RESULT))?;
    let decision = internal(publication::read_decision_head(
        &bundle.join("approval"),
        &review,
        &publication,
    ))?;
    // The native export owner checks its reservation and committed bytes. This
    // endpoint never invokes a renderer, even after an interrupted render.
    let observed = internal(publication_export::observe(
        &service,
        &credential.subject,
        &operation,
        request.format,
        &review,
        &publication,
        decision.as_ref(),
    ))?;
    if observed["export_status"] != "complete" {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "journey_committed_export_required",
        ));
    }
    let value = journey::with_owned_journey(&service, &headers, &operation, |journey| {
        if *_gate
            && journey.manifest().stages[request.format.key()].status
                != crate::codefriend::integration::journey::StageStatus::Complete
        {
            return Err(ApiError(
                StatusCode::SERVICE_UNAVAILABLE,
                "service_draining",
            ));
        }
        internal(journey.attach_hosted_publication(
            request.format,
            &credential.subject,
            &operation,
        ))?;
        internal(serde_json::to_value(journey.manifest()).map_err(Into::into))
    })?;
    publication_export::checked_response(&service, &headers, &operation, &credential.subject, value)
}
