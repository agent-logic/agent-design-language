//! Authenticated hosted export effects and read-only recovery of committed output.
use super::*;
use crate::codefriend::{
    evidence::contracts::{Publication, ReviewRecord},
    integration::PublicationFormat,
    publication::{self, DecisionKind, DecisionRecord},
};
use base64::Engine;

const MAX_EXPORT_BYTES: usize = 2 * 1024 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RenderRequest {
    format: PublicationFormat,
    binding_digest: String,
    decision_digest: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Reservation {
    schema: String,
    operation: String,
    candidate_revision: String,
    format: PublicationFormat,
    binding_digest: String,
    decision_digest: String,
}

fn file_bytes(path: &Path, limit: usize) -> Result<Vec<u8>> {
    let metadata = fs::symlink_metadata(path)?;
    ensure!(
        metadata.is_file() && !metadata.file_type().is_symlink(),
        "export_file_invalid"
    );
    ensure!(metadata.len() <= limit as u64, "export_output_limit");
    let mut bytes = Vec::new();
    File::open(path)?
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= limit, "export_output_limit");
    Ok(bytes)
}

fn selected_file(format: PublicationFormat) -> (&'static str, &'static str) {
    match format {
        PublicationFormat::Markdown => ("report.md", "text/markdown"),
        PublicationFormat::Html => ("report.html", "text/html"),
        PublicationFormat::Pdf => ("report.pdf", "application/pdf"),
    }
}

fn reservation(
    operation: &str,
    format: PublicationFormat,
    decision: &DecisionRecord,
) -> Result<Reservation> {
    Ok(Reservation {
        schema: "codefriend.hosted_export_reservation.v1".into(),
        operation: operation.into(),
        candidate_revision: build::REVISION.into(),
        format,
        binding_digest: decision.publication.binding_digest()?,
        decision_digest: decision.digest.clone(),
    })
}

/// Does not execute a renderer. A prior reservation with no committed output stays unresolved.
pub(super) fn observe(
    service: &Service,
    subject: &str,
    operation: &str,
    format: PublicationFormat,
    review: &ReviewRecord,
    publication: &Publication,
    decision: Option<&DecisionRecord>,
) -> Result<Value> {
    let mut response = json!({"schema":"codefriend.publication_result.v1", "candidate_revision":build::REVISION,
        "format":format, "publication":publication, "decision":decision, "exports":[],
        "export_status":"not_rendered", "render_result":null, "render_manifest":null, "manifest_bytes_base64":null});
    let bundle = publication_directory(service, subject, operation, format);
    let marker = bundle.join("render-reservation.json");
    if !marker.exists() {
        return Ok(response);
    }
    response["export_status"] = json!("effect_unresolved");
    let Some(decision) = decision.filter(|d| d.decision == DecisionKind::Approved) else {
        return Ok(response);
    };
    let saved: Reservation = read_json(&marker, 4096)?;
    if saved != reservation(operation, format, decision)? {
        return Ok(response);
    }
    let target = service
        .dir(subject, operation)
        .join("work/exports")
        .join(&publication.target);
    if !target.exists() {
        return Ok(response);
    }
    let metadata = fs::symlink_metadata(&target)?;
    ensure!(
        metadata.is_dir() && !metadata.file_type().is_symlink(),
        "export_target_invalid"
    );
    let manifest_bytes = file_bytes(&target.join("manifest.json"), MAX_EXPORT_BYTES)?;
    let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
    let (name, media_type) = selected_file(format);
    let bytes = file_bytes(&target.join(name), MAX_EXPORT_BYTES)?;
    let result = publication::relay::result_from_manifest(format, &manifest)?;
    publication::relay::verify_rendered(review, decision, format, &result, &manifest, &bytes)?;
    ensure!(
        result["manifest_digest"].as_str()
            == Some(crate::codefriend::ingestion::digest(&manifest_bytes).as_str()),
        "export_manifest_bytes_changed"
    );
    response["manifest_bytes_base64"] =
        json!(base64::engine::general_purpose::STANDARD.encode(manifest_bytes));
    response["export_status"] = json!("complete");
    response["render_result"] = result;
    response["render_manifest"] = manifest;
    response["exports"] = json!([{"name":name,"media_type":media_type,
        "digest":crate::codefriend::ingestion::digest(&bytes),
        "bytes_base64":base64::engine::general_purpose::STANDARD.encode(bytes)}]);
    ensure!(
        serde_json::to_vec(&response)?.len() <= MAX_RESULT,
        "export_response_limit"
    );
    Ok(response)
}

fn render_owned(
    service: &Service,
    subject: &str,
    operation: &str,
    format: PublicationFormat,
    publication: &Publication,
) -> Result<()> {
    let dir = service.dir(subject, operation);
    let bundle = publication_directory(service, subject, operation, format);
    let review_record = dir.join("work/review/review-record.json");
    let publication_path = bundle.join("publication.json");
    let approval_store = bundle.join("approval");
    let artifact_root = bundle.join("artifacts");
    let synthesis = PathBuf::from("synthesis/synthesis.json");
    let remediation_plan = PathBuf::from("remediation/remediation-plan.json");
    let test_plan = PathBuf::from("tests/test-plan.json");
    let destination_root = dir.join("work/exports");
    let out = destination_root.join(&publication.target);
    match format {
        PublicationFormat::Markdown => {
            publication::render_markdown(publication::MarkdownRenderOptions {
                review_record,
                publication: publication_path,
                approval_store,
                artifact_root,
                synthesis,
                remediation_plan,
                test_plan,
                destination_root,
                out,
            })?;
        }
        PublicationFormat::Html => {
            publication::render_html(publication::HtmlRenderOptions {
                review_record,
                publication: publication_path,
                approval_store,
                artifact_root,
                synthesis,
                remediation_plan,
                test_plan,
                destination_root,
                out,
            })?;
        }
        PublicationFormat::Pdf => {
            publication::render_pdf(publication::PdfRenderOptions {
                review_record,
                publication: publication_path,
                approval_store,
                artifact_root,
                synthesis,
                remediation_plan,
                test_plan,
                destination_root,
                out,
                font: service.0.config.root.join("publication-font.ttf"),
            })?;
        }
    }
    Ok(())
}

pub(super) async fn render(
    State(service): State<Service>,
    headers: HeaderMap,
    HttpPath(operation): HttpPath<String>,
    Json(request): Json<RenderRequest>,
) -> ApiResult<Json<Value>> {
    let _guard = service
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
            "publication_operation_not_current",
        ));
    }
    let dir = service.dir(&credential.subject, &operation);
    let bundle = publication_directory(&service, &credential.subject, &operation, request.format);
    let review: ReviewRecord = internal(read_json(
        &dir.join("work/review/review-record.json"),
        MAX_RESULT,
    ))?;
    let publication: Publication =
        internal(read_json(&bundle.join("publication.json"), MAX_RESULT))?;
    internal(publication.validate(&review))?;
    internal(publication::verify_artifacts(
        &bundle.join("artifacts"),
        &publication.artifact_manifest,
    ))?;
    let decision = internal(publication::read_decision_head(
        &bundle.join("approval"),
        &review,
        &publication,
    ))?
    .ok_or(ApiError(
        StatusCode::CONFLICT,
        "publication_approval_required",
    ))?;
    if decision.decision != DecisionKind::Approved
        || decision.digest != request.decision_digest
        || internal(publication.binding_digest())? != request.binding_digest
    {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "publication_approval_changed",
        ));
    }
    let marker = bundle.join("render-reservation.json");
    if !marker.exists() {
        // Prepare immutable intent before the effect. Any later request only observes it.
        internal(publication::write_json_create_only(
            &marker,
            &reservation(&operation, request.format, &decision)
                .map_err(|_| ApiError(StatusCode::CONFLICT, "publication_binding_invalid"))?,
        ))?;
        let _render_outcome = render_owned(
            &service,
            &credential.subject,
            &operation,
            request.format,
            &publication,
        );
        // Failed/unknown outcomes remain reserved. Only validated committed bytes establish success.
    }
    let current = service.auth(&headers)?;
    let op = service.operation(&current, &operation)?;
    if current.subject != credential.subject
        || current.mode != Mode::Hosted
        || op.status != Status::Complete
        || op.expires_at <= now()
    {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "publication_operation_not_current",
        ));
    }
    let decision = internal(publication::read_decision_head(
        &bundle.join("approval"),
        &review,
        &publication,
    ))?;
    let value = internal(observe(
        &service,
        &current.subject,
        &operation,
        request.format,
        &review,
        &publication,
        decision.as_ref(),
    ))?;
    checked_response(&service, &headers, &operation, &current.subject, value)
}

/// Check authority after artifact verification and response sizing, immediately
/// before returning any retained source bytes to the authenticated transport.
pub(super) fn checked_response(
    service: &Service,
    headers: &HeaderMap,
    operation: &str,
    subject: &str,
    value: Value,
) -> ApiResult<Json<Value>> {
    let size = internal(serde_json::to_vec(&value).map_err(Into::into))?.len();
    if size > MAX_RESULT {
        return Err(ApiError(StatusCode::CONFLICT, "export_response_limit"));
    }
    let credential = service.auth(headers)?;
    let op = service.operation(&credential, operation)?;
    if credential.subject != subject
        || credential.mode != Mode::Hosted
        || op.status != Status::Complete
        || op.expires_at <= now()
        || op.candidate_revision != build::REVISION
    {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "publication_operation_not_current",
        ));
    }
    Ok(Json(value))
}
