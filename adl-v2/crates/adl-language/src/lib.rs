//! Pure clean-room ADL v2 six-primitives language core.

mod canonical;
mod diagnostic;
mod model;
mod parse;
mod validate;

pub use canonical::{canonical_bytes, canonical_json};
pub use diagnostic::{Diagnostic, DiagnosticCode};
pub use model::{
    AdlDocument, Agent, Placement, Prompt, Provider, Run, Task, Tool, Workflow, WorkflowKind,
    WorkflowStep,
};
pub use parse::{parse_and_validate_json, parse_and_validate_yaml, parse_json, parse_yaml};
pub use validate::validate;

/// Generate the schema from the same types used for parsing.
pub fn json_schema() -> serde_json::Value {
    let mut schema = serde_json::to_value(schemars::schema_for!(AdlDocument))
        .expect("schema serialization is infallible");
    schema["properties"]["version"] = serde_json::json!({"enum": ["0.3", "0.5"]});
    schema["$defs"]["Run"]["oneOf"] = serde_json::json!([
        {"required": ["workflow_ref"], "not": {"required": ["workflow"]}},
        {"required": ["workflow"], "not": {"required": ["workflow_ref"]}}
    ]);
    schema
}
