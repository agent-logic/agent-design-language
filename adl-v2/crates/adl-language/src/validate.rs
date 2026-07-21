use crate::{AdlDocument, Diagnostic, DiagnosticCode, Workflow, WorkflowStep};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub fn validate(document: &AdlDocument) -> Result<(), Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    if !matches!(document.version.as_str(), "0.3" | "0.5") {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::InvalidVersion,
            "$.version",
            format!("unsupported language version `{}`", document.version),
        ));
    }
    validate_named_ids(
        "providers",
        &document.providers,
        |value| value.id.as_deref(),
        &mut diagnostics,
    );
    validate_named_ids(
        "tools",
        &document.tools,
        |value| value.id.as_deref(),
        &mut diagnostics,
    );
    validate_named_ids(
        "agents",
        &document.agents,
        |value| value.id.as_deref(),
        &mut diagnostics,
    );
    validate_named_ids(
        "tasks",
        &document.tasks,
        |value| value.id.as_deref(),
        &mut diagnostics,
    );
    validate_named_ids(
        "workflows",
        &document.workflows,
        |value| value.id.as_deref(),
        &mut diagnostics,
    );

    for (name, agent) in &document.agents {
        if !document.providers.contains_key(&agent.provider) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::UnknownProvider,
                format!("$.agents.{name}.provider"),
                format!("references unknown provider `{}`", agent.provider),
            ));
        }
        for tool in &agent.tools {
            if !document.tools.contains_key(tool) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::UnknownTool,
                    format!("$.agents.{name}.tools"),
                    format!("references unknown tool `{tool}`"),
                ));
            }
        }
    }
    for (name, task) in &document.tasks {
        if let Some(agent) = &task.agent_ref {
            if !document.agents.contains_key(agent) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::UnknownAgent,
                    format!("$.tasks.{name}.agent_ref"),
                    format!("references unknown agent `{agent}`"),
                ));
            }
        }
        for tool in &task.tool_allowlist {
            if !document.tools.contains_key(tool) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::UnknownTool,
                    format!("$.tasks.{name}.tool_allowlist"),
                    format!("references unknown tool `{tool}`"),
                ));
            }
        }
    }
    for (name, workflow) in &document.workflows {
        validate_workflow(
            document,
            workflow,
            &format!("$.workflows.{name}"),
            &mut diagnostics,
        );
    }
    match (&document.run.workflow_ref, &document.run.workflow) {
        (Some(reference), None) => {
            if !document.workflows.contains_key(reference) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::UnknownWorkflow,
                    "$.run.workflow_ref",
                    format!("references unknown workflow `{reference}`"),
                ));
            }
        }
        (None, Some(workflow)) => {
            validate_workflow(document, workflow, "$.run.workflow", &mut diagnostics)
        }
        _ => diagnostics.push(Diagnostic::new(
            DiagnosticCode::InvalidRunTarget,
            "$.run",
            "run must declare exactly one of workflow_ref or workflow",
        )),
    }
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn validate_named_ids<T>(
    section: &str,
    values: &BTreeMap<String, T>,
    id: impl Fn(&T) -> Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut explicit = BTreeSet::new();
    for (name, value) in values {
        if !valid_id(name) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::InvalidIdentity,
                format!("$.{section}.{name}"),
                format!("invalid identity `{name}`"),
            ));
        }
        if let Some(explicit_id) = id(value) {
            if explicit_id != name {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::InvalidIdentity,
                    format!("$.{section}.{name}.id"),
                    format!("explicit id `{explicit_id}` must equal map identity `{name}`"),
                ));
            }
            if !explicit.insert(explicit_id.to_owned()) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::DuplicateIdentity,
                    format!("$.{section}.{name}.id"),
                    format!("duplicate explicit identity `{explicit_id}`"),
                ));
            }
        }
    }
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
        })
}

fn validate_workflow(
    document: &AdlDocument,
    workflow: &Workflow,
    path: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut step_ids = BTreeSet::new();
    let mut states = BTreeMap::new();
    for (index, step) in workflow.steps.iter().enumerate() {
        let step_path = format!("{path}.steps[{index}]");
        if !valid_id(&step.id) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::InvalidIdentity,
                format!("{step_path}.id"),
                format!("invalid step identity `{}`", step.id),
            ));
        }
        if !step_ids.insert(step.id.clone()) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::DuplicateIdentity,
                format!("{step_path}.id"),
                format!("duplicate step identity `{}`", step.id),
            ));
        }
        if !document.tasks.contains_key(&step.task) {
            diagnostics.push(Diagnostic::new(
                DiagnosticCode::UnknownTask,
                format!("{step_path}.task"),
                format!("references unknown task `{}`", step.task),
            ));
        }
        if let Some(agent) = &step.agent {
            if !document.agents.contains_key(agent) {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::UnknownAgent,
                    format!("{step_path}.agent"),
                    format!("references unknown agent `{agent}`"),
                ));
            }
        }
        if let Some(state) = &step.save_as {
            if states.insert(state.clone(), step.id.clone()).is_some() {
                diagnostics.push(Diagnostic::new(
                    DiagnosticCode::DuplicateIdentity,
                    format!("{step_path}.save_as"),
                    format!("duplicate saved state `{state}`"),
                ));
            }
        }
    }
    let mut dependencies: BTreeMap<String, BTreeSet<String>> = workflow
        .steps
        .iter()
        .map(|step| (step.id.clone(), BTreeSet::new()))
        .collect();
    for (index, step) in workflow.steps.iter().enumerate() {
        for reference in state_references(step) {
            match states.get(&reference) {
                Some(owner) => {
                    dependencies
                        .entry(step.id.clone())
                        .or_default()
                        .insert(owner.clone());
                }
                None => diagnostics.push(Diagnostic::new(
                    DiagnosticCode::UnknownState,
                    format!("{path}.steps[{index}].inputs"),
                    format!("references unknown saved state `{reference}`"),
                )),
            }
        }
    }
    if has_cycle(&dependencies) {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::DependencyCycle,
            path,
            "workflow contains a dependency cycle",
        ));
    }
}

fn state_references(step: &WorkflowStep) -> Vec<String> {
    let mut references = Vec::new();
    for value in step.inputs.values() {
        collect_state_references(value, &mut references);
    }
    references
}

fn collect_state_references(value: &Value, references: &mut Vec<String>) {
    match value {
        Value::String(value) => {
            if let Some(reference) = value.strip_prefix("@state:") {
                references.push(reference.to_owned());
            }
        }
        Value::Array(values) => values
            .iter()
            .for_each(|value| collect_state_references(value, references)),
        Value::Object(values) => values
            .values()
            .for_each(|value| collect_state_references(value, references)),
        _ => {}
    }
}

fn has_cycle(graph: &BTreeMap<String, BTreeSet<String>>) -> bool {
    fn visit(
        node: &str,
        graph: &BTreeMap<String, BTreeSet<String>>,
        visiting: &mut BTreeSet<String>,
        visited: &mut BTreeSet<String>,
    ) -> bool {
        if visited.contains(node) {
            return false;
        }
        if !visiting.insert(node.to_owned()) {
            return true;
        }
        if graph
            .get(node)
            .into_iter()
            .flatten()
            .any(|next| visit(next, graph, visiting, visited))
        {
            return true;
        }
        visiting.remove(node);
        visited.insert(node.to_owned());
        false
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    graph
        .keys()
        .any(|node| visit(node, graph, &mut visiting, &mut visited))
}
