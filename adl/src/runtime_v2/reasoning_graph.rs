//! Runtime-v2 reasoning graph contract for v0.91.7 WP-11.
//!
//! This module makes the pre-v0.92 reasoning graph path executable enough to
//! validate producer/consumer handoff without claiming the broader v0.94
//! reasoning/provenance graph engine. It binds hypotheses, evidence,
//! decisions, outcomes, trace references, ObsMem references, and PVF proof
//! hooks into one deterministic artifact.

use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

pub const RUNTIME_V2_REASONING_GRAPH_SCHEMA: &str = "runtime_v2.reasoning_graph.v1";
pub const RUNTIME_V2_REASONING_GRAPH_PATH: &str = "runtime_v2/reasoning_graph/reasoning_graph.json";
pub const RUNTIME_V2_REASONING_GRAPH_FEATURE_DOC: &str =
    "docs/milestones/v0.91.7/features/REASONING_GRAPH_LOOP_SKILL_STANDARD_BRIDGE_v0.91.7.md";
pub const RUNTIME_V2_REASONING_GRAPH_TEST_MARKER: &str = "runtime_v2_reasoning_graph";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ReasoningGraphPacket {
    pub schema_version: String,
    pub graph_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub graph: RuntimeV2ReasoningGraph,
    pub handoff: RuntimeV2ReasoningGraphHandoff,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ReasoningGraph {
    pub graph_kind: String,
    pub nodes: Vec<RuntimeV2ReasoningNode>,
    pub edges: Vec<RuntimeV2ReasoningEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ReasoningNode {
    pub node_id: String,
    pub node_kind: RuntimeV2ReasoningNodeKind,
    pub summary: String,
    pub trace_refs: Vec<String>,
    pub obsmem_refs: Vec<String>,
    pub pvf_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeV2ReasoningNodeKind {
    PromptInput,
    Hypothesis,
    Evidence,
    Decision,
    Outcome,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ReasoningEdge {
    pub edge_id: String,
    pub from: String,
    pub to: String,
    pub edge_kind: RuntimeV2ReasoningEdgeKind,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeV2ReasoningEdgeKind {
    Proposes,
    Supports,
    Challenges,
    Decides,
    Produces,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ReasoningGraphHandoff {
    pub runtime_consumer_refs: Vec<String>,
    pub trace_handoff_refs: Vec<String>,
    pub obsmem_handoff_refs: Vec<String>,
    pub pvf_handoff_refs: Vec<String>,
    pub replay_guarantees: Vec<String>,
    pub blocked_claims: Vec<String>,
}

impl RuntimeV2ReasoningGraphPacket {
    pub fn prototype() -> Result<Self> {
        let packet = Self {
            schema_version: RUNTIME_V2_REASONING_GRAPH_SCHEMA.to_string(),
            graph_id: "reasoning-graph-v0-91-7-wp-11".to_string(),
            milestone: "v0.91.7".to_string(),
            wp: "WP-11".to_string(),
            artifact_path: RUNTIME_V2_REASONING_GRAPH_PATH.to_string(),
            source_feature_doc: RUNTIME_V2_REASONING_GRAPH_FEATURE_DOC.to_string(),
            graph: RuntimeV2ReasoningGraph {
                graph_kind: "minimal_runtime_reasoning_graph".to_string(),
                nodes: prototype_nodes(),
                edges: prototype_edges(),
            },
            handoff: RuntimeV2ReasoningGraphHandoff {
                runtime_consumer_refs: vec![
                    "adl/src/runtime_v2/mod.rs".to_string(),
                    "adl/src/runtime_v2/moral_trace_schema.rs".to_string(),
                    "adl/src/obsmem_contract/models.rs".to_string(),
                    "docs/milestones/v0.91.4/features/PVF_INITIAL_LANE_INVENTORY_MANIFEST_v0.91.4.json"
                        .to_string(),
                ],
                trace_handoff_refs: vec![
                    "runtime_v2/trace/reasoning-graph-v0-91-7-wp-11.json".to_string(),
                    "runtime_v2/moral_trace/reasoning-graph-decision-0001.json".to_string(),
                ],
                obsmem_handoff_refs: vec![
                    "obsmem/reasoning_graph/reasoning-graph-v0-91-7-wp-11.json".to_string(),
                ],
                pvf_handoff_refs: vec![
                    "cargo test --manifest-path adl/Cargo.toml runtime_v2_reasoning_graph -- --nocapture".to_string(),
                    "git diff --check".to_string(),
                ],
                replay_guarantees: vec![
                    "nodes and edges are sorted deterministically before serialization".to_string(),
                    "every edge endpoint must resolve to an existing node".to_string(),
                    "hypotheses must be evidence-linked before a decision can consume them".to_string(),
                    "outcomes must be decision-linked before replay can treat them as produced".to_string(),
                ],
                blocked_claims: vec![
                    "does not implement the full v0.94 reasoning/provenance graph engine".to_string(),
                    "does not ratify adl.skill.v1".to_string(),
                    "does not implement the WP-11 loop runtime sibling issue".to_string(),
                ],
            },
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_reasoning_graph -- --nocapture".to_string(),
                "git diff --check".to_string(),
            ],
            claim_boundary:
                "WP-11 #4694 proves a bounded Runtime v2 reasoning graph contract with deterministic validation, trace handoff refs, ObsMem refs, and PVF proof hooks. It does not claim the sibling loop runtime, adl.skill.v1 ratification, or the full v0.94 reasoning/provenance graph engine."
                    .to_string(),
            non_claims: vec![
                "does not implement the WP-11 loop runtime sibling issue".to_string(),
                "does not ratify or implement the full adl.skill.v1 standard".to_string(),
                "does not replace moral trace, ObsMem, PVF, UTS, ACC, or Runtime v2 contracts".to_string(),
                "does not claim v0.92 activation beyond this bounded proof surface".to_string(),
                "does not implement unbounded graph mutation, probabilistic planning, or autonomous reasoning authority".to_string(),
            ],
        };
        packet.validate()?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        require_exact(
            &self.schema_version,
            RUNTIME_V2_REASONING_GRAPH_SCHEMA,
            "reasoning_graph.schema_version",
        )?;
        normalize_id(self.graph_id.clone(), "reasoning_graph.graph_id")?;
        require_exact(&self.milestone, "v0.91.7", "reasoning_graph.milestone")?;
        require_exact(&self.wp, "WP-11", "reasoning_graph.wp")?;
        require_exact(
            &self.artifact_path,
            RUNTIME_V2_REASONING_GRAPH_PATH,
            "reasoning_graph.artifact_path",
        )?;
        validate_relative_path(&self.artifact_path, "reasoning_graph.artifact_path")?;
        require_exact(
            &self.source_feature_doc,
            RUNTIME_V2_REASONING_GRAPH_FEATURE_DOC,
            "reasoning_graph.source_feature_doc",
        )?;
        validate_relative_path(
            &self.source_feature_doc,
            "reasoning_graph.source_feature_doc",
        )?;

        validate_reasoning_graph(&self.graph)?;
        validate_reasoning_graph_handoff(&self.handoff)?;
        validate_reasoning_validation_commands(&self.validation_commands)?;
        validate_non_claims(&self.non_claims)?;
        validate_nonempty_text(&self.claim_boundary, "reasoning_graph.claim_boundary")?;
        ensure_contains(
            &self.claim_boundary,
            "bounded Runtime v2 reasoning graph contract",
            "reasoning graph claim boundary must stay bounded to this runtime contract",
        )?;
        ensure_contains(
            &self.claim_boundary,
            "does not claim the sibling loop runtime",
            "reasoning graph claim boundary must preserve sibling-loop non-claim",
        )?;

        Ok(())
    }

    pub fn canonicalized(&self) -> Result<Self> {
        self.validate()?;
        let mut canonical = self.clone();
        canonical
            .graph
            .nodes
            .sort_by(|a, b| a.node_id.cmp(&b.node_id));
        canonical
            .graph
            .edges
            .sort_by(|a, b| a.edge_id.cmp(&b.edge_id));
        canonical.validation_commands.sort();
        canonical.non_claims.sort();
        canonical.handoff.runtime_consumer_refs.sort();
        canonical.handoff.trace_handoff_refs.sort();
        canonical.handoff.obsmem_handoff_refs.sort();
        canonical.handoff.pvf_handoff_refs.sort();
        canonical.handoff.replay_guarantees.sort();
        canonical.handoff.blocked_claims.sort();
        canonical.validate()?;
        Ok(canonical)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        let canonical = self.canonicalized()?;
        serde_json::to_vec_pretty(&canonical).context("serialize Runtime v2 reasoning graph packet")
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create reasoning graph parent '{}'", parent.display()))?;
        }
        fs::write(path, self.pretty_json_bytes()?)
            .with_context(|| format!("write reasoning graph packet to '{}'", path.display()))
    }
}

pub fn runtime_v2_reasoning_graph_contract() -> Result<RuntimeV2ReasoningGraphPacket> {
    RuntimeV2ReasoningGraphPacket::prototype()
}

fn prototype_nodes() -> Vec<RuntimeV2ReasoningNode> {
    vec![
        node(
            "prompt-input-0001",
            RuntimeV2ReasoningNodeKind::PromptInput,
            "The runtime receives a bounded issue-local prompt that asks for a reasoned action.",
            &["trace://runtime-v2/prompt-input-0001"],
            &["obsmem://reasoning/prompt-input-0001"],
            &["pvf://wp-11/reasoning-graph/input-shape"],
        ),
        node(
            "hypothesis-0001",
            RuntimeV2ReasoningNodeKind::Hypothesis,
            "A candidate implementation path is proposed from the prompt and current runtime state.",
            &["trace://runtime-v2/hypothesis-0001"],
            &["obsmem://reasoning/hypothesis-0001"],
            &["pvf://wp-11/reasoning-graph/hypothesis-shape"],
        ),
        node(
            "evidence-0001",
            RuntimeV2ReasoningNodeKind::Evidence,
            "Evidence links the hypothesis to repository facts, prior trace, and ObsMem references.",
            &["trace://runtime-v2/evidence-0001"],
            &["obsmem://reasoning/evidence-0001"],
            &["pvf://wp-11/reasoning-graph/evidence-required"],
        ),
        node(
            "decision-0001",
            RuntimeV2ReasoningNodeKind::Decision,
            "The runtime selects a bounded action only after the hypothesis is evidence-linked.",
            &["trace://runtime-v2/decision-0001"],
            &["obsmem://reasoning/decision-0001"],
            &["pvf://wp-11/reasoning-graph/decision-shape"],
        ),
        node(
            "outcome-0001",
            RuntimeV2ReasoningNodeKind::Outcome,
            "The action outcome records trace, ObsMem, and validation refs for later replay.",
            &["trace://runtime-v2/outcome-0001"],
            &["obsmem://reasoning/outcome-0001"],
            &["pvf://wp-11/reasoning-graph/outcome-shape"],
        ),
    ]
}

fn prototype_edges() -> Vec<RuntimeV2ReasoningEdge> {
    vec![
        edge(
            "edge-prompt-proposes-hypothesis",
            "prompt-input-0001",
            "hypothesis-0001",
            RuntimeV2ReasoningEdgeKind::Proposes,
            "A bounded prompt may propose a candidate hypothesis.",
        ),
        edge(
            "edge-evidence-supports-hypothesis",
            "evidence-0001",
            "hypothesis-0001",
            RuntimeV2ReasoningEdgeKind::Supports,
            "A hypothesis must cite evidence before decision.",
        ),
        edge(
            "edge-hypothesis-decides-action",
            "hypothesis-0001",
            "decision-0001",
            RuntimeV2ReasoningEdgeKind::Decides,
            "The selected decision consumes the evidence-linked hypothesis.",
        ),
        edge(
            "edge-decision-produces-outcome",
            "decision-0001",
            "outcome-0001",
            RuntimeV2ReasoningEdgeKind::Produces,
            "Outcomes are produced by explicit decisions, not by orphan claims.",
        ),
    ]
}

fn node(
    node_id: &str,
    node_kind: RuntimeV2ReasoningNodeKind,
    summary: &str,
    trace_refs: &[&str],
    obsmem_refs: &[&str],
    pvf_refs: &[&str],
) -> RuntimeV2ReasoningNode {
    RuntimeV2ReasoningNode {
        node_id: node_id.to_string(),
        node_kind,
        summary: summary.to_string(),
        trace_refs: strings(trace_refs),
        obsmem_refs: strings(obsmem_refs),
        pvf_refs: strings(pvf_refs),
    }
}

fn edge(
    edge_id: &str,
    from: &str,
    to: &str,
    edge_kind: RuntimeV2ReasoningEdgeKind,
    rationale: &str,
) -> RuntimeV2ReasoningEdge {
    RuntimeV2ReasoningEdge {
        edge_id: edge_id.to_string(),
        from: from.to_string(),
        to: to.to_string(),
        edge_kind,
        rationale: rationale.to_string(),
    }
}

fn validate_reasoning_graph(graph: &RuntimeV2ReasoningGraph) -> Result<()> {
    require_exact(
        &graph.graph_kind,
        "minimal_runtime_reasoning_graph",
        "reasoning_graph.graph_kind",
    )?;
    if graph.nodes.len() < 5 {
        return Err(anyhow!(
            "reasoning graph must include prompt, hypothesis, evidence, decision, and outcome nodes"
        ));
    }
    if graph.edges.is_empty() {
        return Err(anyhow!("reasoning graph edges must not be empty"));
    }

    let mut node_ids = BTreeSet::new();
    let mut node_kind_by_id = BTreeMap::new();
    let mut kinds = BTreeSet::new();
    for node in &graph.nodes {
        normalize_id(node.node_id.clone(), "reasoning_graph.node_id")?;
        if !node_ids.insert(node.node_id.clone()) {
            return Err(anyhow!(
                "reasoning graph contains duplicate node '{}'",
                node.node_id
            ));
        }
        node_kind_by_id.insert(node.node_id.clone(), node.node_kind.clone());
        kinds.insert(node.node_kind.clone());
        validate_nonempty_text(&node.summary, "reasoning_graph.node.summary")?;
        validate_uri_refs(
            &node.trace_refs,
            "reasoning_graph.node.trace_refs",
            "trace://",
        )?;
        validate_uri_refs(
            &node.obsmem_refs,
            "reasoning_graph.node.obsmem_refs",
            "obsmem://",
        )?;
        validate_uri_refs(&node.pvf_refs, "reasoning_graph.node.pvf_refs", "pvf://")?;
    }

    for required in [
        RuntimeV2ReasoningNodeKind::PromptInput,
        RuntimeV2ReasoningNodeKind::Hypothesis,
        RuntimeV2ReasoningNodeKind::Evidence,
        RuntimeV2ReasoningNodeKind::Decision,
        RuntimeV2ReasoningNodeKind::Outcome,
    ] {
        if !kinds.contains(&required) {
            return Err(anyhow!("reasoning graph missing required node kind"));
        }
    }

    let mut edge_ids = BTreeSet::new();
    let mut incoming_by_kind: BTreeMap<String, BTreeSet<RuntimeV2ReasoningEdgeKind>> =
        BTreeMap::new();
    for edge in &graph.edges {
        normalize_id(edge.edge_id.clone(), "reasoning_graph.edge_id")?;
        if !edge_ids.insert(edge.edge_id.clone()) {
            return Err(anyhow!(
                "reasoning graph contains duplicate edge '{}'",
                edge.edge_id
            ));
        }
        if !node_ids.contains(&edge.from) {
            return Err(anyhow!(
                "reasoning graph edge '{}' has missing from node '{}'",
                edge.edge_id,
                edge.from
            ));
        }
        if !node_ids.contains(&edge.to) {
            return Err(anyhow!(
                "reasoning graph edge '{}' has missing to node '{}'",
                edge.edge_id,
                edge.to
            ));
        }
        validate_nonempty_text(&edge.rationale, "reasoning_graph.edge.rationale")?;
        validate_edge_semantics(edge, &node_kind_by_id)?;
        incoming_by_kind
            .entry(edge.to.clone())
            .or_default()
            .insert(edge.edge_kind.clone());
    }

    require_incoming_for_kind(
        graph,
        &incoming_by_kind,
        RuntimeV2ReasoningNodeKind::Hypothesis,
        RuntimeV2ReasoningEdgeKind::Supports,
        "reasoning graph hypotheses must be evidence-supported",
    )?;
    require_incoming_for_kind(
        graph,
        &incoming_by_kind,
        RuntimeV2ReasoningNodeKind::Decision,
        RuntimeV2ReasoningEdgeKind::Decides,
        "reasoning graph decisions must consume a hypothesis",
    )?;
    require_incoming_for_kind(
        graph,
        &incoming_by_kind,
        RuntimeV2ReasoningNodeKind::Outcome,
        RuntimeV2ReasoningEdgeKind::Produces,
        "reasoning graph outcomes must be decision-produced",
    )
}

fn validate_edge_semantics(
    edge: &RuntimeV2ReasoningEdge,
    node_kind_by_id: &BTreeMap<String, RuntimeV2ReasoningNodeKind>,
) -> Result<()> {
    let from_kind = node_kind_by_id.get(&edge.from).ok_or_else(|| {
        anyhow!(
            "reasoning graph edge '{}' has unresolved from kind",
            edge.edge_id
        )
    })?;
    let to_kind = node_kind_by_id.get(&edge.to).ok_or_else(|| {
        anyhow!(
            "reasoning graph edge '{}' has unresolved to kind",
            edge.edge_id
        )
    })?;

    let valid = match edge.edge_kind {
        RuntimeV2ReasoningEdgeKind::Proposes => {
            from_kind == &RuntimeV2ReasoningNodeKind::PromptInput
                && to_kind == &RuntimeV2ReasoningNodeKind::Hypothesis
        }
        RuntimeV2ReasoningEdgeKind::Supports => {
            from_kind == &RuntimeV2ReasoningNodeKind::Evidence
                && to_kind == &RuntimeV2ReasoningNodeKind::Hypothesis
        }
        RuntimeV2ReasoningEdgeKind::Challenges => {
            matches!(
                from_kind,
                RuntimeV2ReasoningNodeKind::Evidence | RuntimeV2ReasoningNodeKind::Hypothesis
            ) && to_kind == &RuntimeV2ReasoningNodeKind::Hypothesis
        }
        RuntimeV2ReasoningEdgeKind::Decides => {
            from_kind == &RuntimeV2ReasoningNodeKind::Hypothesis
                && to_kind == &RuntimeV2ReasoningNodeKind::Decision
        }
        RuntimeV2ReasoningEdgeKind::Produces => {
            from_kind == &RuntimeV2ReasoningNodeKind::Decision
                && to_kind == &RuntimeV2ReasoningNodeKind::Outcome
        }
    };
    if valid {
        Ok(())
    } else {
        Err(anyhow!(
            "reasoning graph edge '{}' has invalid {:?} endpoints",
            edge.edge_id,
            edge.edge_kind
        ))
    }
}

fn require_incoming_for_kind(
    graph: &RuntimeV2ReasoningGraph,
    incoming_by_kind: &BTreeMap<String, BTreeSet<RuntimeV2ReasoningEdgeKind>>,
    node_kind: RuntimeV2ReasoningNodeKind,
    edge_kind: RuntimeV2ReasoningEdgeKind,
    message: &str,
) -> Result<()> {
    for node in graph
        .nodes
        .iter()
        .filter(|node| node.node_kind == node_kind)
    {
        let Some(kinds) = incoming_by_kind.get(&node.node_id) else {
            return Err(anyhow!(message.to_string()));
        };
        if !kinds.contains(&edge_kind) {
            return Err(anyhow!(message.to_string()));
        }
    }
    Ok(())
}

fn validate_reasoning_graph_handoff(handoff: &RuntimeV2ReasoningGraphHandoff) -> Result<()> {
    validate_path_refs(
        &handoff.runtime_consumer_refs,
        "reasoning_graph.handoff.runtime_consumer_refs",
    )?;
    validate_path_refs(
        &handoff.trace_handoff_refs,
        "reasoning_graph.handoff.trace_handoff_refs",
    )?;
    validate_path_refs(
        &handoff.obsmem_handoff_refs,
        "reasoning_graph.handoff.obsmem_handoff_refs",
    )?;
    if handoff.pvf_handoff_refs.is_empty() {
        return Err(anyhow!(
            "reasoning graph PVF handoff refs must not be empty"
        ));
    }
    if !handoff
        .pvf_handoff_refs
        .iter()
        .any(|value| value.contains(RUNTIME_V2_REASONING_GRAPH_TEST_MARKER))
    {
        return Err(anyhow!(
            "reasoning graph PVF handoff must include the focused test marker"
        ));
    }
    for value in &handoff.pvf_handoff_refs {
        validate_nonempty_text(value, "reasoning_graph.handoff.pvf_handoff_refs")?;
    }
    validate_requirement_list(
        &handoff.replay_guarantees,
        "reasoning_graph.handoff.replay_guarantees",
    )?;
    validate_requirement_list(
        &handoff.blocked_claims,
        "reasoning_graph.handoff.blocked_claims",
    )?;
    ensure_contains_in_list(
        &handoff.replay_guarantees,
        "edge endpoint",
        "reasoning graph replay guarantees must cover endpoint validation",
    )?;
    ensure_contains_in_list(
        &handoff.blocked_claims,
        "v0.94 reasoning/provenance graph engine",
        "reasoning graph blocked claims must preserve the v0.94 non-claim",
    )
}

fn validate_reasoning_validation_commands(commands: &[String]) -> Result<()> {
    validate_requirement_list(commands, "reasoning_graph.validation_commands")?;
    ensure_contains_in_list(
        commands,
        RUNTIME_V2_REASONING_GRAPH_TEST_MARKER,
        "reasoning graph validation commands must include the focused test marker",
    )?;
    ensure_contains_in_list(
        commands,
        "git diff --check",
        "reasoning graph validation commands must include git diff hygiene",
    )
}

fn validate_non_claims(non_claims: &[String]) -> Result<()> {
    validate_requirement_list(non_claims, "reasoning_graph.non_claims")?;
    for needle in [
        "loop runtime sibling issue",
        "adl.skill.v1",
        "moral trace, ObsMem, PVF, UTS, ACC, or Runtime v2",
        "unbounded graph mutation",
    ] {
        ensure_contains_in_list(
            non_claims,
            needle,
            "reasoning graph non-claims must preserve sibling and boundary limits",
        )?;
    }
    Ok(())
}

fn validate_uri_refs(values: &[String], field: &str, prefix: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_nonempty_text(value, field)?;
        if !value.starts_with(prefix) {
            return Err(anyhow!("{field} must use {prefix} refs"));
        }
    }
    Ok(())
}

fn validate_path_refs(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_relative_path(value, field)?;
    }
    Ok(())
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
        Err(anyhow!("{field} must be '{expected}'"))
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}
