use serde_json::{json, Value};

use crate::doctor::DoctorReport;
use crate::model::IssueRecord;
use crate::store::{BootstrapRequest, EditRequest};

pub fn public_schema_bundle() -> Value {
    json!({
        "schema": "csdlc.public_schema_bundle.v1",
        "bootstrap_request": schemars::schema_for!(BootstrapRequest),
        "edit_request": schemars::schema_for!(EditRequest),
        "issue_record": schemars::schema_for!(IssueRecord),
        "doctor_report": schemars::schema_for!(DoctorReport),
    })
}
