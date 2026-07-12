use serde_json::{json, Value};

use crate::doctor::DoctorReport;
use crate::lifecycle::{BindRequest, BindResult, RecoverClaimRequest};
use crate::model::IssueRecord;
use crate::store::ApproveDesignRequest;
use crate::store::{BootstrapRequest, EditRequest};

pub fn public_schema_bundle() -> Value {
    json!({
        "schema": "csdlc.public_schema_bundle.v1",
        "bootstrap_request": schemars::schema_for!(BootstrapRequest),
        "approve_design_request": schemars::schema_for!(ApproveDesignRequest),
        "edit_request": schemars::schema_for!(EditRequest),
        "bind_request": schemars::schema_for!(BindRequest),
        "bind_result": schemars::schema_for!(BindResult),
        "recover_claim_request": schemars::schema_for!(RecoverClaimRequest),
        "issue_record": schemars::schema_for!(IssueRecord),
        "doctor_report": schemars::schema_for!(DoctorReport),
    })
}
