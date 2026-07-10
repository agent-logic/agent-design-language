//! Runtime-v2 bounded loop runtime integrated with the WP-11 reasoning graph.
//!
//! This is the v0.91.7 loop runtime contract: deterministic, termination-limited
//! execution over a validated reasoning graph packet and explicit runtime state.

use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

pub const RUNTIME_V2_LOOP_RUNTIME_SCHEMA: &str = "runtime_v2.loop_runtime.v1";
pub const RUNTIME_V2_LOOP_RUNTIME_PATH: &str = "runtime_v2/loop_runtime/loop_runtime.json";
pub const RUNTIME_V2_LOOP_RUNTIME_TEST_MARKER: &str = "runtime_v2_loop_runtime";
const MAX_LOOP_ITERATIONS: u32 = 8;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2LoopRuntimePacket {
    pub schema_version: String,
    pub runtime_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub reasoning_graph_ref: String,
    pub reasoning_graph_id: String,
    pub loop_definition: RuntimeV2LoopDefinition,
    pub initial_state: RuntimeV2LoopState,
    pub replay: RuntimeV2LoopReplay,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2LoopDefinition {
    pub loop_id: String,
    pub graph_id: String,
    pub start_node_id: String,
    pub terminal_node_ids: Vec<String>,
    pub max_iterations: u32,
    pub steps: Vec<RuntimeV2LoopStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2LoopStep {
    pub step_id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub edge_id: String,
    pub action: RuntimeV2LoopAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeV2LoopAction {
    Propose,
    CollectEvidence,
    Decide,
    ProduceOutcome,
    Terminate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2LoopState {
    pub state_id: String,
    pub graph_id: String,
    pub current_node_id: String,
    pub completed_step_ids: Vec<String>,
    pub iteration: u32,
    pub status: RuntimeV2LoopStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeV2LoopStatus {
    Ready,
    Running,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2LoopReplay {
    pub events: Vec<RuntimeV2LoopEvent>,
    pub final_state: RuntimeV2LoopState,
    pub replay_guarantees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2LoopEvent {
    pub event_sequence: u32,
    pub step_id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub edge_id: String,
    pub action: RuntimeV2LoopAction,
    pub iteration: u32,
}

impl RuntimeV2LoopRuntimePacket {
    pub fn prototype() -> Result<Self> {
        let graph = runtime_v2_reasoning_graph_contract()?;
        runtime_v2_loop_runtime_contract_for_graph(
            &graph,
            RuntimeV2LoopState::ready_for_graph(&graph),
        )
    }

    pub fn validate(&self) -> Result<()> {
        require_exact(
            &self.schema_version,
            RUNTIME_V2_LOOP_RUNTIME_SCHEMA,
            "loop_runtime.schema_version",
        )?;
        normalize_id(self.runtime_id.clone(), "loop_runtime.runtime_id")?;
        require_exact(&self.milestone, "v0.91.7", "loop_runtime.milestone")?;
        require_exact(&self.wp, "WP-11", "loop_runtime.wp")?;
        require_exact(
            &self.artifact_path,
            RUNTIME_V2_LOOP_RUNTIME_PATH,
            "loop_runtime.artifact_path",
        )?;
        validate_relative_path(&self.artifact_path, "loop_runtime.artifact_path")?;
        validate_relative_path(
            &self.reasoning_graph_ref,
            "loop_runtime.reasoning_graph_ref",
        )?;
        normalize_id(
            self.reasoning_graph_id.clone(),
            "loop_runtime.reasoning_graph_id",
        )?;
        if self.loop_definition.graph_id != self.reasoning_graph_id
            || self.initial_state.graph_id != self.reasoning_graph_id
            || self.replay.final_state.graph_id != self.reasoning_graph_id
        {
            return Err(anyhow!(
                "loop runtime graph/state ids must match the reasoning graph id"
            ));
        }
        let graph = runtime_v2_reasoning_graph_contract()
            .context("loop runtime validation requires the WP-11 reasoning graph contract")?;
        if self.reasoning_graph_id != graph.graph_id {
            return Err(anyhow!(
                "loop runtime reasoning graph id does not match the WP-11 reasoning graph"
            ));
        }
        validate_loop_definition_against_graph(&self.loop_definition, &graph)?;
        validate_loop_graph_state(&graph, &self.initial_state)?;
        validate_loop_state(&self.initial_state, "loop_runtime.initial_state")?;
        validate_loop_replay(&self.loop_definition, &self.initial_state, &self.replay)?;
        validate_loop_validation_commands(&self.validation_commands)?;
        validate_requirement_list(&self.non_claims, "loop_runtime.non_claims")?;
        ensure_contains_in_list(
            &self.non_claims,
            "unbounded",
            "loop runtime non-claims must reject unbounded execution",
        )?;
        validate_nonempty_text(&self.claim_boundary, "loop_runtime.claim_boundary")?;
        ensure_contains(
            &self.claim_boundary,
            "bounded Runtime v2 loop runtime",
            "loop runtime claim boundary must stay bounded",
        )?;
        ensure_contains(
            &self.claim_boundary,
            "validated reasoning graph",
            "loop runtime claim boundary must name reasoning graph integration",
        )
    }

    pub fn canonicalized(&self) -> Result<Self> {
        self.validate()?;
        let mut canonical = self.clone();
        canonical.loop_definition.terminal_node_ids.sort();
        canonical
            .loop_definition
            .steps
            .sort_by(|a, b| a.step_id.cmp(&b.step_id));
        canonical.initial_state.completed_step_ids.sort();
        canonical.validation_commands.sort();
        canonical.non_claims.sort();
        canonical.replay.replay_guarantees.sort();
        canonical.validate()?;
        Ok(canonical)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        let canonical = self.canonicalized()?;
        serde_json::to_vec_pretty(&canonical).context("serialize Runtime v2 loop runtime packet")
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create loop runtime parent '{}'", parent.display()))?;
        }
        fs::write(path, self.pretty_json_bytes()?)
            .with_context(|| format!("write loop runtime packet to '{}'", path.display()))
    }
}

impl RuntimeV2LoopState {
    pub fn ready_for_graph(graph: &RuntimeV2ReasoningGraphPacket) -> Self {
        Self {
            state_id: "loop-state-0001".to_string(),
            graph_id: graph.graph_id.clone(),
            current_node_id: "prompt-input-0001".to_string(),
            completed_step_ids: Vec::new(),
            iteration: 0,
            status: RuntimeV2LoopStatus::Ready,
        }
    }
}

pub fn runtime_v2_loop_runtime_contract() -> Result<RuntimeV2LoopRuntimePacket> {
    RuntimeV2LoopRuntimePacket::prototype()
}

pub fn runtime_v2_loop_runtime_contract_for_graph(
    graph: &RuntimeV2ReasoningGraphPacket,
    state: RuntimeV2LoopState,
) -> Result<RuntimeV2LoopRuntimePacket> {
    graph
        .validate()
        .context("loop runtime requires a validated reasoning graph")?;
    validate_loop_graph_state(graph, &state)?;
    let definition = prototype_loop_definition(graph);
    validate_loop_definition_against_graph(&definition, graph)?;
    let replay = execute_loop(&definition, &state)?;
    let packet = RuntimeV2LoopRuntimePacket {
        schema_version: RUNTIME_V2_LOOP_RUNTIME_SCHEMA.to_string(),
        runtime_id: "loop-runtime-v0-91-7-wp-11".to_string(),
        milestone: "v0.91.7".to_string(),
        wp: "WP-11".to_string(),
        artifact_path: RUNTIME_V2_LOOP_RUNTIME_PATH.to_string(),
        reasoning_graph_ref: RUNTIME_V2_REASONING_GRAPH_PATH.to_string(),
        reasoning_graph_id: graph.graph_id.clone(),
        loop_definition: definition,
        initial_state: state,
        replay,
        validation_commands: vec![
            format!(
                "cargo test --manifest-path adl/Cargo.toml {} -- --nocapture",
                RUNTIME_V2_LOOP_RUNTIME_TEST_MARKER
            ),
            "git diff --check".to_string(),
        ],
        claim_boundary:
            "WP-11 #4695 proves a bounded Runtime v2 loop runtime that consumes a validated reasoning graph, applies explicit state transitions, enforces termination limits, and emits deterministic replay events. It does not claim unbounded autonomy, adl.skill.v1 ratification, or the full v0.94 reasoning/provenance graph engine."
                .to_string(),
        non_claims: vec![
            "does not implement unbounded autonomous loops".to_string(),
            "does not ratify or implement the full adl.skill.v1 standard".to_string(),
            "does not replace moral trace, ObsMem, PVF, UTS, ACC, or Runtime v2 contracts".to_string(),
            "does not claim the full v0.94 reasoning/provenance graph engine".to_string(),
        ],
    };
    packet.validate()?;
    Ok(packet)
}

fn prototype_loop_definition(graph: &RuntimeV2ReasoningGraphPacket) -> RuntimeV2LoopDefinition {
    RuntimeV2LoopDefinition {
        loop_id: "reasoning-graph-loop-0001".to_string(),
        graph_id: graph.graph_id.clone(),
        start_node_id: "prompt-input-0001".to_string(),
        terminal_node_ids: vec!["outcome-0001".to_string()],
        max_iterations: 4,
        steps: vec![
            loop_step(
                "step-0001-propose",
                "prompt-input-0001",
                "hypothesis-0001",
                "edge-prompt-proposes-hypothesis",
                RuntimeV2LoopAction::Propose,
            ),
            loop_step(
                "step-0002-collect-evidence",
                "evidence-0001",
                "hypothesis-0001",
                "edge-evidence-supports-hypothesis",
                RuntimeV2LoopAction::CollectEvidence,
            ),
            loop_step(
                "step-0003-decide",
                "hypothesis-0001",
                "decision-0001",
                "edge-hypothesis-decides-action",
                RuntimeV2LoopAction::Decide,
            ),
            loop_step(
                "step-0004-produce-outcome",
                "decision-0001",
                "outcome-0001",
                "edge-decision-produces-outcome",
                RuntimeV2LoopAction::ProduceOutcome,
            ),
        ],
    }
}

fn loop_step(
    step_id: &str,
    from_node_id: &str,
    to_node_id: &str,
    edge_id: &str,
    action: RuntimeV2LoopAction,
) -> RuntimeV2LoopStep {
    RuntimeV2LoopStep {
        step_id: step_id.to_string(),
        from_node_id: from_node_id.to_string(),
        to_node_id: to_node_id.to_string(),
        edge_id: edge_id.to_string(),
        action,
    }
}

fn execute_loop(
    definition: &RuntimeV2LoopDefinition,
    initial_state: &RuntimeV2LoopState,
) -> Result<RuntimeV2LoopReplay> {
    validate_loop_definition(definition)?;
    validate_loop_state(initial_state, "loop_runtime.execution.initial_state")?;
    if initial_state.status == RuntimeV2LoopStatus::Terminated {
        return Err(anyhow!(
            "loop runtime cannot execute from a terminated state"
        ));
    }
    if initial_state.iteration >= definition.max_iterations {
        return Err(anyhow!(
            "loop runtime termination limit reached before execution"
        ));
    }

    let mut state = initial_state.clone();
    let completed: BTreeSet<String> = state.completed_step_ids.iter().cloned().collect();
    let mut events = Vec::new();
    for step in definition
        .steps
        .iter()
        .filter(|step| !completed.contains(&step.step_id))
    {
        if state.iteration >= definition.max_iterations {
            return Err(anyhow!(
                "loop runtime exceeded termination limit before step '{}'",
                step.step_id
            ));
        }
        if state.current_node_id != step.from_node_id
            && step.action != RuntimeV2LoopAction::CollectEvidence
        {
            return Err(anyhow!(
                "loop runtime state/node mismatch before step '{}'",
                step.step_id
            ));
        }
        state.iteration += 1;
        state.status = RuntimeV2LoopStatus::Running;
        events.push(RuntimeV2LoopEvent {
            event_sequence: events.len() as u32 + 1,
            step_id: step.step_id.clone(),
            from_node_id: step.from_node_id.clone(),
            to_node_id: step.to_node_id.clone(),
            edge_id: step.edge_id.clone(),
            action: step.action.clone(),
            iteration: state.iteration,
        });
        state.current_node_id = step.to_node_id.clone();
        state.completed_step_ids.push(step.step_id.clone());
    }
    if definition
        .terminal_node_ids
        .iter()
        .any(|node_id| node_id == &state.current_node_id)
    {
        state.status = RuntimeV2LoopStatus::Terminated;
    }
    state.completed_step_ids.sort();
    let replay = RuntimeV2LoopReplay {
        events,
        final_state: state,
        replay_guarantees: vec![
            "loop steps are sorted by stable step_id before canonical serialization".to_string(),
            "event_sequence is contiguous and starts at one".to_string(),
            "execution fails closed when graph, state, or loop definition references diverge"
                .to_string(),
            "max_iterations bounds every replay".to_string(),
        ],
    };
    validate_loop_replay(definition, initial_state, &replay)?;
    Ok(replay)
}

fn validate_loop_graph_state(
    graph: &RuntimeV2ReasoningGraphPacket,
    state: &RuntimeV2LoopState,
) -> Result<()> {
    if state.graph_id != graph.graph_id {
        return Err(anyhow!("loop runtime missing graph/state binding"));
    }
    let node_ids: BTreeSet<&str> = graph
        .graph
        .nodes
        .iter()
        .map(|node| node.node_id.as_str())
        .collect();
    if !node_ids.contains(state.current_node_id.as_str()) {
        return Err(anyhow!(
            "loop runtime state references missing graph node '{}'",
            state.current_node_id
        ));
    }
    Ok(())
}

fn validate_loop_definition(definition: &RuntimeV2LoopDefinition) -> Result<()> {
    normalize_id(
        definition.loop_id.clone(),
        "loop_runtime.definition.loop_id",
    )?;
    normalize_id(
        definition.graph_id.clone(),
        "loop_runtime.definition.graph_id",
    )?;
    normalize_id(
        definition.start_node_id.clone(),
        "loop_runtime.definition.start_node_id",
    )?;
    if definition.terminal_node_ids.is_empty() {
        return Err(anyhow!(
            "loop runtime definition must declare terminal nodes"
        ));
    }
    if definition.max_iterations == 0 || definition.max_iterations > MAX_LOOP_ITERATIONS {
        return Err(anyhow!(
            "loop runtime max_iterations must be between 1 and {MAX_LOOP_ITERATIONS}"
        ));
    }
    if definition.steps.is_empty() {
        return Err(anyhow!(
            "loop runtime definition must include ordered steps"
        ));
    }
    if definition.steps.len() as u32 > definition.max_iterations {
        return Err(anyhow!(
            "loop runtime definition steps exceed termination limit"
        ));
    }
    let mut step_ids = BTreeSet::new();
    let mut prior_to: Option<&str> = None;
    for step in &definition.steps {
        normalize_id(step.step_id.clone(), "loop_runtime.step.step_id")?;
        if !step_ids.insert(step.step_id.clone()) {
            return Err(anyhow!(
                "loop runtime definition contains duplicate step '{}'",
                step.step_id
            ));
        }
        normalize_id(step.from_node_id.clone(), "loop_runtime.step.from_node_id")?;
        normalize_id(step.to_node_id.clone(), "loop_runtime.step.to_node_id")?;
        normalize_id(step.edge_id.clone(), "loop_runtime.step.edge_id")?;
        if let Some(prior_to) = prior_to {
            if prior_to != step.from_node_id && step.action != RuntimeV2LoopAction::CollectEvidence
            {
                return Err(anyhow!(
                    "loop runtime definition steps must form deterministic replay order"
                ));
            }
        }
        prior_to = Some(&step.to_node_id);
    }
    Ok(())
}

fn validate_loop_definition_against_graph(
    definition: &RuntimeV2LoopDefinition,
    graph: &RuntimeV2ReasoningGraphPacket,
) -> Result<()> {
    validate_loop_definition(definition)?;
    if definition.graph_id != graph.graph_id {
        return Err(anyhow!(
            "loop runtime definition graph id does not match reasoning graph"
        ));
    }
    let nodes: BTreeSet<&str> = graph
        .graph
        .nodes
        .iter()
        .map(|node| node.node_id.as_str())
        .collect();
    let edges: BTreeMap<&str, (&str, &str)> = graph
        .graph
        .edges
        .iter()
        .map(|edge| {
            (
                edge.edge_id.as_str(),
                (edge.from.as_str(), edge.to.as_str()),
            )
        })
        .collect();
    if !nodes.contains(definition.start_node_id.as_str()) {
        return Err(anyhow!(
            "loop runtime definition start node is missing from graph"
        ));
    }
    for terminal in &definition.terminal_node_ids {
        if !nodes.contains(terminal.as_str()) {
            return Err(anyhow!(
                "loop runtime definition terminal node is missing from graph"
            ));
        }
    }
    for step in &definition.steps {
        if !nodes.contains(step.from_node_id.as_str()) || !nodes.contains(step.to_node_id.as_str())
        {
            return Err(anyhow!(
                "loop runtime definition references missing graph node"
            ));
        }
        let Some((edge_from, edge_to)) = edges.get(step.edge_id.as_str()) else {
            return Err(anyhow!(
                "loop runtime definition references missing graph edge '{}'",
                step.edge_id
            ));
        };
        if edge_from != &step.from_node_id.as_str() || edge_to != &step.to_node_id.as_str() {
            return Err(anyhow!(
                "loop runtime step endpoints must match the referenced graph edge"
            ));
        }
    }
    Ok(())
}

fn validate_loop_state(state: &RuntimeV2LoopState, field: &str) -> Result<()> {
    normalize_id(state.state_id.clone(), field)?;
    normalize_id(state.graph_id.clone(), field)?;
    normalize_id(state.current_node_id.clone(), field)?;
    let mut completed = BTreeSet::new();
    for step_id in &state.completed_step_ids {
        normalize_id(step_id.clone(), field)?;
        if !completed.insert(step_id) {
            return Err(anyhow!("{field} contains duplicate completed step ids"));
        }
    }
    Ok(())
}

fn validate_loop_replay(
    definition: &RuntimeV2LoopDefinition,
    initial_state: &RuntimeV2LoopState,
    replay: &RuntimeV2LoopReplay,
) -> Result<()> {
    if replay.events.len() as u32 > definition.max_iterations {
        return Err(anyhow!("loop runtime replay exceeds termination limit"));
    }
    let mut expected_sequence = 1;
    let mut expected_iteration = initial_state.iteration + 1;
    let mut expected_state = initial_state.clone();
    let completed: BTreeSet<String> = initial_state.completed_step_ids.iter().cloned().collect();
    let definition_step_ids: BTreeSet<&str> = definition
        .steps
        .iter()
        .map(|step| step.step_id.as_str())
        .collect();
    for step_id in &completed {
        if !definition_step_ids.contains(step_id.as_str()) {
            return Err(anyhow!(
                "loop runtime initial state references unknown completed step '{}'",
                step_id
            ));
        }
    }
    let mut found_pending_step = false;
    for step in &definition.steps {
        if completed.contains(&step.step_id) {
            if found_pending_step {
                return Err(anyhow!(
                    "loop runtime completed steps must be a deterministic prefix of the loop definition"
                ));
            }
        } else {
            found_pending_step = true;
        }
    }
    let expected_steps: Vec<&RuntimeV2LoopStep> = definition
        .steps
        .iter()
        .filter(|step| !completed.contains(&step.step_id))
        .collect();
    if replay.events.len() != expected_steps.len() {
        return Err(anyhow!(
            "loop runtime replay must cover every pending loop step exactly once"
        ));
    }
    for event in &replay.events {
        if event.event_sequence != expected_sequence {
            return Err(anyhow!(
                "loop runtime replay event sequence must be contiguous"
            ));
        }
        if event.iteration != expected_iteration {
            return Err(anyhow!(
                "loop runtime replay iteration order must be deterministic"
            ));
        }
        let expected_step = expected_steps[(expected_sequence - 1) as usize];
        if event.step_id != expected_step.step_id
            || event.from_node_id != expected_step.from_node_id
            || event.to_node_id != expected_step.to_node_id
            || event.edge_id != expected_step.edge_id
            || event.action != expected_step.action
        {
            return Err(anyhow!(
                "loop runtime replay event does not match deterministic loop definition order"
            ));
        }
        if expected_state.current_node_id != expected_step.from_node_id
            && expected_step.action != RuntimeV2LoopAction::CollectEvidence
        {
            return Err(anyhow!(
                "loop runtime replay state/node mismatch before step '{}'",
                expected_step.step_id
            ));
        }
        expected_state.iteration = event.iteration;
        expected_state.status = RuntimeV2LoopStatus::Running;
        expected_state.current_node_id = expected_step.to_node_id.clone();
        expected_state
            .completed_step_ids
            .push(expected_step.step_id.clone());
        expected_sequence += 1;
        expected_iteration += 1;
    }
    if definition
        .terminal_node_ids
        .iter()
        .any(|node_id| node_id == &expected_state.current_node_id)
    {
        expected_state.status = RuntimeV2LoopStatus::Terminated;
    }
    expected_state.completed_step_ids.sort();
    if replay.final_state != expected_state {
        return Err(anyhow!(
            "loop runtime replay final state must match deterministic execution"
        ));
    }
    if replay.final_state.iteration > definition.max_iterations {
        return Err(anyhow!(
            "loop runtime final state exceeds termination limit"
        ));
    }
    validate_loop_state(&replay.final_state, "loop_runtime.replay.final_state")?;
    validate_requirement_list(
        &replay.replay_guarantees,
        "loop_runtime.replay.replay_guarantees",
    )?;
    ensure_contains_in_list(
        &replay.replay_guarantees,
        "event_sequence",
        "loop runtime replay guarantees must cover event sequence ordering",
    )
}

fn validate_loop_validation_commands(commands: &[String]) -> Result<()> {
    validate_requirement_list(commands, "loop_runtime.validation_commands")?;
    ensure_contains_in_list(
        commands,
        RUNTIME_V2_LOOP_RUNTIME_TEST_MARKER,
        "loop runtime validation commands must include the focused test marker",
    )?;
    ensure_contains_in_list(
        commands,
        "git diff --check",
        "loop runtime validation commands must include git diff hygiene",
    )
}

fn validate_requirement_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
    }
    Ok(())
}

fn ensure_contains_in_list(values: &[String], needle: &str, message: &str) -> Result<()> {
    if values.iter().any(|value| value.contains(needle)) {
        Ok(())
    } else {
        Err(anyhow!(message.to_string()))
    }
}

fn ensure_contains(value: &str, needle: &str, message: &str) -> Result<()> {
    if value.contains(needle) {
        Ok(())
    } else {
        Err(anyhow!(message.to_string()))
    }
}

fn require_exact(actual: &str, expected: &str, field: &str) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(anyhow!("{field} must be '{expected}' but found '{actual}'"))
    }
}
