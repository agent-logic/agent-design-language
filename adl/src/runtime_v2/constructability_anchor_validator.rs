//! Runtime-v2 Constructability Anchor Validator contract for v0.91.7 WP-10.
//!
//! This module owns the deterministic boundary that separates provisional
//! cognition from shared-reality publication. It validates that construction
//! events cite admissible anchors, retain validator decisions, and fail closed
//! when promotion is attempted without evidence.

use super::*;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

pub const RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_SCHEMA: &str =
    "runtime_v2.constructability_anchor_validator.v1";
pub const RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_PATH: &str =
    "runtime_v2/constructability/anchor-validator.json";
pub const RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_FEATURE_DOC: &str =
    "docs/milestones/v0.91.7/features/CONSTRUCTABILITY_GATE_v0.91.7.md";
pub const RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_TEST_MARKER: &str =
    "runtime_v2_constructability_anchor_validator";
pub const RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF: &str =
    "validator-constructability-anchor-v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ConstructabilityAnchorValidatorPacket {
    pub schema_version: String,
    pub validator_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub runtime_module_ref: String,
    pub construction_events: Vec<RuntimeV2ConstructionEvent>,
    pub admissible_anchors: Vec<RuntimeV2ConstructabilityAnchor>,
    pub decisions: Vec<RuntimeV2ConstructabilityDecision>,
    pub shared_reality_boundary: RuntimeV2SharedRealityBoundary,
    pub failure_modes: Vec<RuntimeV2ConstructabilityFailureMode>,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ConstructionEvent {
    pub event_id: String,
    pub source_ref: String,
    pub provisional_claim: String,
    pub requested_publication: RuntimeV2ConstructabilityPublicationScope,
    pub anchor_refs: Vec<String>,
    pub validator_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeV2ConstructabilityPublicationScope {
    InternalTraceOnly,
    ReviewPacket,
    SharedReality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ConstructabilityAnchor {
    pub anchor_id: String,
    pub anchor_kind: RuntimeV2ConstructabilityAnchorKind,
    pub source_ref: String,
    pub admissibility: RuntimeV2ConstructabilityAdmissibility,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeV2ConstructabilityAnchorKind {
    RetainedArtifact,
    RuntimeTrace,
    OperatorApproval,
    ExternalRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeV2ConstructabilityAdmissibility {
    Admissible,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ConstructabilityDecision {
    pub decision_id: String,
    pub event_id: String,
    pub outcome: RuntimeV2ConstructabilityOutcome,
    pub blocking_reasons: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub reviewer_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeV2ConstructabilityOutcome {
    Pass,
    FailClosed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2SharedRealityBoundary {
    pub promotion_requires_anchor: bool,
    pub promotion_requires_validator_pass: bool,
    pub promotion_requires_operator_review: bool,
    pub provisional_storage_path: String,
    pub shared_reality_publication_path: String,
    pub prohibited_promotions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2ConstructabilityFailureMode {
    pub failure_id: String,
    pub rejected_event_id: String,
    pub expected_error: String,
    pub blocks_shared_reality: bool,
}

impl RuntimeV2ConstructabilityAnchorValidatorPacket {
    pub fn prototype() -> Result<Self> {
        let packet = Self {
            schema_version: RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_SCHEMA.to_string(),
            validator_id: RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF.to_string(),
            milestone: "v0.91.7".to_string(),
            wp: "WP-10".to_string(),
            artifact_path: RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_PATH.to_string(),
            source_feature_doc: RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_FEATURE_DOC
                .to_string(),
            runtime_module_ref: "adl/src/runtime_v2/constructability_anchor_validator.rs"
                .to_string(),
            construction_events: prototype_construction_events(),
            admissible_anchors: prototype_anchors(),
            decisions: prototype_decisions(),
            shared_reality_boundary: RuntimeV2SharedRealityBoundary {
                promotion_requires_anchor: true,
                promotion_requires_validator_pass: true,
                promotion_requires_operator_review: true,
                provisional_storage_path:
                    "runtime_v2/constructability/provisional-claims.jsonl".to_string(),
                shared_reality_publication_path:
                    "runtime_v2/constructability/shared-reality-publications.jsonl".to_string(),
                prohibited_promotions: vec![
                    "unanchored_claim".to_string(),
                    "validator_failed_claim".to_string(),
                    "operator_unreviewed_external_claim".to_string(),
                    "private_reasoning_as_public_truth".to_string(),
                ],
            },
            failure_modes: vec![RuntimeV2ConstructabilityFailureMode {
                failure_id: "failure-unanchored-shared-reality".to_string(),
                rejected_event_id: "event-unanchored-promotion-attempt".to_string(),
                expected_error:
                    "shared-reality promotion requires at least one admissible anchor"
                        .to_string(),
                blocks_shared_reality: true,
            }],
            validation_commands: vec![
                format!("cargo test --manifest-path adl/Cargo.toml {RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_TEST_MARKER} -- --nocapture"),
                "cargo test --manifest-path adl/Cargo.toml trace_runtime_v2_constructability_anchor_validator -- --nocapture".to_string(),
                "adl/target/debug/adl runtime-v2 constructability-anchor-validator --input .adl/local-artifacts/wp10-constructability/anchor-validator.json --out .adl/local-artifacts/wp10-constructability/validated-anchor-decision.json".to_string(),
                "git diff --check".to_string(),
            ],
            claim_boundary:
                "WP-10 #4693 proves a bounded Runtime v2 constructability anchor validator for construction-event admissibility and shared-reality promotion gating. It is ready for later CSM runtime-component hosting, but does not claim universal truth adjudication, autonomous publication, or WP-07A supervisor integration."
                    .to_string(),
            non_claims: vec![
                "does not adjudicate universal truth".to_string(),
                "does not publish shared reality without admissible anchors".to_string(),
                "does not bypass Freedom Gate, CAV, or operator review".to_string(),
                "does not claim WP-07A CSM component supervisor integration".to_string(),
                "does not replace human review".to_string(),
            ],
        };
        packet.validate()?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        require_exact(
            &self.schema_version,
            RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_SCHEMA,
            "constructability.schema_version",
        )?;
        require_exact(
            &self.validator_id,
            RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF,
            "constructability.validator_id",
        )?;
        require_exact(&self.milestone, "v0.91.7", "constructability.milestone")?;
        require_exact(&self.wp, "WP-10", "constructability.wp")?;
        require_exact(
            &self.artifact_path,
            RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_PATH,
            "constructability.artifact_path",
        )?;
        require_exact(
            &self.source_feature_doc,
            RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_FEATURE_DOC,
            "constructability.source_feature_doc",
        )?;
        validate_relative_path(&self.artifact_path, "constructability.artifact_path")?;
        validate_relative_path(
            &self.source_feature_doc,
            "constructability.source_feature_doc",
        )?;
        validate_relative_path(
            &self.runtime_module_ref,
            "constructability.runtime_module_ref",
        )?;
        validate_anchors(&self.admissible_anchors)?;
        validate_events(&self.construction_events, &self.admissible_anchors)?;
        validate_decisions(
            &self.decisions,
            &self.construction_events,
            &self.admissible_anchors,
        )?;
        validate_shared_reality_boundary(&self.shared_reality_boundary)?;
        validate_failure_modes(
            &self.failure_modes,
            &self.construction_events,
            &self.decisions,
        )?;
        validate_command_list(&self.validation_commands)?;
        ensure_contains_in_list(
            &self.non_claims,
            "does not claim WP-07A CSM component supervisor integration",
            "constructability non-claims must preserve WP-07A boundary",
        )?;
        ensure_contains(
            &self.claim_boundary,
            "bounded Runtime v2 constructability anchor validator",
            "constructability claim boundary must stay bounded to this validator",
        )?;
        ensure_contains(
            &self.claim_boundary,
            "later CSM runtime-component hosting",
            "constructability claim boundary must name future CSM hosting",
        )
    }

    pub fn canonicalized(&self) -> Result<Self> {
        self.validate()?;
        let mut canonical = self.clone();
        canonical
            .construction_events
            .sort_by(|a, b| a.event_id.cmp(&b.event_id));
        for event in &mut canonical.construction_events {
            event.anchor_refs.sort();
            event.validator_refs.sort();
        }
        canonical
            .admissible_anchors
            .sort_by(|a, b| a.anchor_id.cmp(&b.anchor_id));
        canonical
            .decisions
            .sort_by(|a, b| a.decision_id.cmp(&b.decision_id));
        for decision in &mut canonical.decisions {
            decision.blocking_reasons.sort();
            decision.blocking_reasons.dedup();
            decision.evidence_refs.sort();
        }
        canonical
            .failure_modes
            .sort_by(|a, b| a.failure_id.cmp(&b.failure_id));
        canonical
            .shared_reality_boundary
            .prohibited_promotions
            .sort();
        canonical
            .shared_reality_boundary
            .prohibited_promotions
            .dedup();
        canonical.validation_commands.sort();
        canonical.validation_commands.dedup();
        canonical.non_claims.sort();
        canonical.non_claims.dedup();
        canonical.validate()?;
        Ok(canonical)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(&self.canonicalized()?)
            .context("serialize Runtime v2 Constructability Anchor Validator packet")
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "create Runtime v2 Constructability output directory {}",
                    parent.display()
                )
            })?;
        }
        fs::write(path, self.pretty_json_bytes()?).with_context(|| {
            format!(
                "write Runtime v2 Constructability Anchor Validator packet to {}",
                path.display()
            )
        })
    }
}

pub fn runtime_v2_constructability_anchor_validator_contract(
) -> Result<RuntimeV2ConstructabilityAnchorValidatorPacket> {
    RuntimeV2ConstructabilityAnchorValidatorPacket::prototype()?.canonicalized()
}

fn prototype_construction_events() -> Vec<RuntimeV2ConstructionEvent> {
    vec![
        RuntimeV2ConstructionEvent {
            event_id: "event-curiosity-proposal-admission".to_string(),
            source_ref: "runtime_v2/curiosity_engine/curiosity_engine.json".to_string(),
            provisional_claim:
                "A bounded curiosity proposal may enter shared review when anchored to retained Runtime v2 evidence."
                    .to_string(),
            requested_publication: RuntimeV2ConstructabilityPublicationScope::ReviewPacket,
            anchor_refs: vec![
                "anchor-curiosity-engine-packet".to_string(),
                "anchor-operator-review-boundary".to_string(),
            ],
            validator_refs: vec![RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF.to_string()],
        },
        RuntimeV2ConstructionEvent {
            event_id: "event-unanchored-promotion-attempt".to_string(),
            source_ref: "runtime_v2/constructability/provisional-claims.jsonl".to_string(),
            provisional_claim:
                "A provisional internal hypothesis can be promoted directly to shared reality."
                    .to_string(),
            requested_publication: RuntimeV2ConstructabilityPublicationScope::SharedReality,
            anchor_refs: vec![],
            validator_refs: vec![RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF.to_string()],
        },
    ]
}

fn prototype_anchors() -> Vec<RuntimeV2ConstructabilityAnchor> {
    vec![
        RuntimeV2ConstructabilityAnchor {
            anchor_id: "anchor-curiosity-engine-packet".to_string(),
            anchor_kind: RuntimeV2ConstructabilityAnchorKind::RetainedArtifact,
            source_ref: "runtime_v2/curiosity_engine/curiosity_engine.json".to_string(),
            admissibility: RuntimeV2ConstructabilityAdmissibility::Admissible,
            summary:
                "Retained Runtime v2 curiosity packet records proposal, gates, and non-claims."
                    .to_string(),
        },
        RuntimeV2ConstructabilityAnchor {
            anchor_id: "anchor-operator-review-boundary".to_string(),
            anchor_kind: RuntimeV2ConstructabilityAnchorKind::OperatorApproval,
            source_ref: RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_FEATURE_DOC.to_string(),
            admissibility: RuntimeV2ConstructabilityAdmissibility::Admissible,
            summary:
                "Feature boundary requires operator or reviewer approval before shared-reality publication."
                    .to_string(),
        },
    ]
}

fn prototype_decisions() -> Vec<RuntimeV2ConstructabilityDecision> {
    vec![
        RuntimeV2ConstructabilityDecision {
            decision_id: "decision-curiosity-proposal-admitted".to_string(),
            event_id: "event-curiosity-proposal-admission".to_string(),
            outcome: RuntimeV2ConstructabilityOutcome::Pass,
            blocking_reasons: vec![],
            evidence_refs: vec![
                "anchor-curiosity-engine-packet".to_string(),
                "anchor-operator-review-boundary".to_string(),
            ],
            reviewer_notes:
                "Review-packet publication is allowed because all cited anchors are admissible and no shared-reality publication is attempted."
                    .to_string(),
        },
        RuntimeV2ConstructabilityDecision {
            decision_id: "decision-unanchored-promotion-rejected".to_string(),
            event_id: "event-unanchored-promotion-attempt".to_string(),
            outcome: RuntimeV2ConstructabilityOutcome::FailClosed,
            blocking_reasons: vec![
                "shared-reality promotion requires at least one admissible anchor".to_string(),
                "operator review is required before external/shared publication".to_string(),
            ],
            evidence_refs: vec![],
            reviewer_notes:
                "The validator rejects direct promotion of a provisional internal hypothesis into shared reality."
                    .to_string(),
        },
    ]
}

fn validate_events(
    events: &[RuntimeV2ConstructionEvent],
    anchors: &[RuntimeV2ConstructabilityAnchor],
) -> Result<()> {
    if events.is_empty() {
        return Err(anyhow!(
            "constructability construction_events must not be empty"
        ));
    }
    let anchor_ids: BTreeSet<_> = anchors
        .iter()
        .map(|anchor| anchor.anchor_id.as_str())
        .collect();
    let mut seen = BTreeSet::new();
    for event in events {
        normalize_id(event.event_id.clone(), "constructability.event_id")?;
        if !seen.insert(event.event_id.clone()) {
            return Err(anyhow!(
                "duplicate constructability event '{}'",
                event.event_id
            ));
        }
        validate_relative_path(&event.source_ref, "constructability.event.source_ref")?;
        validate_nonempty_text(
            &event.provisional_claim,
            "constructability.event.provisional_claim",
        )?;
        let mut seen_anchor_refs = BTreeSet::new();
        for anchor_ref in &event.anchor_refs {
            normalize_id(anchor_ref.clone(), "constructability.event.anchor_refs")?;
            if !seen_anchor_refs.insert(anchor_ref.as_str()) {
                return Err(anyhow!(
                    "constructability event '{}' repeats anchor ref '{}'",
                    event.event_id,
                    anchor_ref
                ));
            }
            if !anchor_ids.contains(anchor_ref.as_str()) {
                return Err(anyhow!(
                    "constructability event '{}' cites missing anchor '{}'",
                    event.event_id,
                    anchor_ref
                ));
            }
        }
        if event.validator_refs.is_empty() {
            return Err(anyhow!(
                "constructability event '{}' must name a validator",
                event.event_id
            ));
        }
        for validator_ref in &event.validator_refs {
            normalize_id(
                validator_ref.clone(),
                "constructability.event.validator_refs",
            )?;
        }
        if event.validator_refs != [RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF.to_string()] {
            return Err(anyhow!(
                "constructability event '{}' must cite exactly canonical validator '{}'",
                event.event_id,
                RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_REF
            ));
        }
    }
    Ok(())
}

fn validate_anchors(anchors: &[RuntimeV2ConstructabilityAnchor]) -> Result<()> {
    if anchors.is_empty() {
        return Err(anyhow!(
            "constructability admissible_anchors must not be empty"
        ));
    }
    let mut seen = BTreeSet::new();
    for anchor in anchors {
        normalize_id(anchor.anchor_id.clone(), "constructability.anchor_id")?;
        if !seen.insert(anchor.anchor_id.clone()) {
            return Err(anyhow!(
                "duplicate constructability anchor '{}'",
                anchor.anchor_id
            ));
        }
        validate_relative_path(&anchor.source_ref, "constructability.anchor.source_ref")?;
        validate_nonempty_text(&anchor.summary, "constructability.anchor.summary")?;
    }
    Ok(())
}

fn validate_decisions(
    decisions: &[RuntimeV2ConstructabilityDecision],
    events: &[RuntimeV2ConstructionEvent],
    anchors: &[RuntimeV2ConstructabilityAnchor],
) -> Result<()> {
    if decisions.is_empty() {
        return Err(anyhow!("constructability decisions must not be empty"));
    }
    let event_ids: BTreeSet<_> = events.iter().map(|event| event.event_id.as_str()).collect();
    let admissible_anchor_ids: BTreeSet<_> = anchors
        .iter()
        .filter(|anchor| anchor.admissibility == RuntimeV2ConstructabilityAdmissibility::Admissible)
        .map(|anchor| anchor.anchor_id.as_str())
        .collect();
    let mut seen = BTreeSet::new();
    let mut decisions_by_event = BTreeSet::new();

    for decision in decisions {
        normalize_id(decision.decision_id.clone(), "constructability.decision_id")?;
        if !seen.insert(decision.decision_id.clone()) {
            return Err(anyhow!(
                "duplicate constructability decision '{}'",
                decision.decision_id
            ));
        }
        if !event_ids.contains(decision.event_id.as_str()) {
            return Err(anyhow!(
                "constructability decision '{}' cites missing event '{}'",
                decision.decision_id,
                decision.event_id
            ));
        }
        if !decisions_by_event.insert(decision.event_id.as_str()) {
            return Err(anyhow!(
                "constructability event '{}' has multiple validator decisions",
                decision.event_id
            ));
        }
        validate_nonempty_text(
            &decision.reviewer_notes,
            "constructability.decision.reviewer_notes",
        )?;
        let event = events
            .iter()
            .find(|event| event.event_id == decision.event_id)
            .expect("decision event was validated above");
        if event.requested_publication == RuntimeV2ConstructabilityPublicationScope::SharedReality
            && event.anchor_refs.is_empty()
            && decision.outcome != RuntimeV2ConstructabilityOutcome::FailClosed
        {
            return Err(anyhow!(
                "unanchored shared-reality event '{}' must fail closed",
                event.event_id
            ));
        }
        match decision.outcome {
            RuntimeV2ConstructabilityOutcome::Pass => {
                if decision.evidence_refs.is_empty() {
                    return Err(anyhow!(
                        "constructability pass decision '{}' must cite evidence refs",
                        decision.decision_id
                    ));
                }
                let mut seen_evidence_refs = BTreeSet::new();
                for evidence_ref in &decision.evidence_refs {
                    if !seen_evidence_refs.insert(evidence_ref.as_str()) {
                        return Err(anyhow!(
                            "constructability pass decision '{}' repeats evidence ref '{}'",
                            decision.decision_id,
                            evidence_ref
                        ));
                    }
                    if !admissible_anchor_ids.contains(evidence_ref.as_str()) {
                        return Err(anyhow!(
                            "constructability pass decision '{}' cites non-admissible anchor '{}'",
                            decision.decision_id,
                            evidence_ref
                        ));
                    }
                    if !event.anchor_refs.contains(evidence_ref) {
                        return Err(anyhow!(
                            "constructability pass decision '{}' cites anchor '{}' not declared by event '{}'",
                            decision.decision_id,
                            evidence_ref,
                            decision.event_id
                        ));
                    }
                }
                let declared_anchors: BTreeSet<_> =
                    event.anchor_refs.iter().map(String::as_str).collect();
                let decision_evidence: BTreeSet<_> =
                    decision.evidence_refs.iter().map(String::as_str).collect();
                if declared_anchors != decision_evidence {
                    return Err(anyhow!(
                        "constructability pass decision '{}' must retain every declared event anchor exactly once",
                        decision.decision_id
                    ));
                }
                if event.requested_publication
                    == RuntimeV2ConstructabilityPublicationScope::SharedReality
                    && !decision.evidence_refs.iter().any(|evidence_ref| {
                        anchors.iter().any(|anchor| {
                            anchor.anchor_id == *evidence_ref
                                && anchor.anchor_kind
                                    == RuntimeV2ConstructabilityAnchorKind::OperatorApproval
                                && anchor.admissibility
                                    == RuntimeV2ConstructabilityAdmissibility::Admissible
                        })
                    })
                {
                    return Err(anyhow!(
                        "constructability shared-reality pass decision '{}' requires an admissible operator-approval anchor",
                        decision.decision_id
                    ));
                }
                if !decision.blocking_reasons.is_empty() {
                    return Err(anyhow!(
                        "constructability pass decision '{}' must not include blocking reasons",
                        decision.decision_id
                    ));
                }
            }
            RuntimeV2ConstructabilityOutcome::FailClosed => {
                if decision.blocking_reasons.is_empty() {
                    return Err(anyhow!(
                        "constructability fail-closed decision '{}' must name blocking reasons",
                        decision.decision_id
                    ));
                }
            }
        }
    }

    for event in events {
        if !decisions_by_event.contains(event.event_id.as_str()) {
            return Err(anyhow!(
                "constructability event '{}' has no validator decision",
                event.event_id
            ));
        }
        if event.requested_publication == RuntimeV2ConstructabilityPublicationScope::SharedReality
            && event.anchor_refs.is_empty()
        {
            let decision = decisions
                .iter()
                .find(|decision| decision.event_id == event.event_id)
                .expect("decision exists for event");
            if decision.outcome != RuntimeV2ConstructabilityOutcome::FailClosed {
                return Err(anyhow!(
                    "unanchored shared-reality event '{}' must fail closed",
                    event.event_id
                ));
            }
        }
    }

    Ok(())
}

fn validate_shared_reality_boundary(boundary: &RuntimeV2SharedRealityBoundary) -> Result<()> {
    if !boundary.promotion_requires_anchor {
        return Err(anyhow!(
            "constructability boundary must require an admissible anchor"
        ));
    }
    if !boundary.promotion_requires_validator_pass {
        return Err(anyhow!(
            "constructability boundary must require validator pass"
        ));
    }
    if !boundary.promotion_requires_operator_review {
        return Err(anyhow!(
            "constructability boundary must require operator review"
        ));
    }
    validate_relative_path(
        &boundary.provisional_storage_path,
        "constructability.boundary.provisional_storage_path",
    )?;
    validate_relative_path(
        &boundary.shared_reality_publication_path,
        "constructability.boundary.shared_reality_publication_path",
    )?;
    ensure_contains_in_list(
        &boundary.prohibited_promotions,
        "unanchored_claim",
        "constructability boundary must prohibit unanchored claims",
    )?;
    ensure_contains_in_list(
        &boundary.prohibited_promotions,
        "private_reasoning_as_public_truth",
        "constructability boundary must prohibit private reasoning as public truth",
    )
}

fn validate_failure_modes(
    failure_modes: &[RuntimeV2ConstructabilityFailureMode],
    events: &[RuntimeV2ConstructionEvent],
    decisions: &[RuntimeV2ConstructabilityDecision],
) -> Result<()> {
    if failure_modes.is_empty() {
        return Err(anyhow!("constructability failure_modes must not be empty"));
    }
    let event_ids: BTreeSet<_> = events.iter().map(|event| event.event_id.as_str()).collect();
    let mut seen = BTreeSet::new();
    for failure in failure_modes {
        normalize_id(
            failure.failure_id.clone(),
            "constructability.failure.failure_id",
        )?;
        if !seen.insert(failure.failure_id.as_str()) {
            return Err(anyhow!(
                "duplicate constructability failure mode '{}'",
                failure.failure_id
            ));
        }
        if !event_ids.contains(failure.rejected_event_id.as_str()) {
            return Err(anyhow!(
                "constructability failure '{}' cites missing rejected event '{}'",
                failure.failure_id,
                failure.rejected_event_id
            ));
        }
        validate_nonempty_text(
            &failure.expected_error,
            "constructability.failure.expected_error",
        )?;
        if !failure.blocks_shared_reality {
            return Err(anyhow!(
                "constructability failure '{}' must block shared reality",
                failure.failure_id
            ));
        }
        let decision = decisions
            .iter()
            .find(|decision| decision.event_id == failure.rejected_event_id)
            .ok_or_else(|| {
                anyhow!(
                    "constructability failure '{}' has no validator decision for rejected event '{}'",
                    failure.failure_id,
                    failure.rejected_event_id
                )
            })?;
        if decision.outcome != RuntimeV2ConstructabilityOutcome::FailClosed {
            return Err(anyhow!(
                "constructability failure '{}' must map to a fail-closed decision",
                failure.failure_id
            ));
        }
        if !decision
            .blocking_reasons
            .iter()
            .any(|reason| reason == &failure.expected_error)
        {
            return Err(anyhow!(
                "constructability failure '{}' expected error is not retained in its decision",
                failure.failure_id
            ));
        }
    }
    Ok(())
}

fn validate_command_list(commands: &[String]) -> Result<()> {
    if commands.is_empty() {
        return Err(anyhow!(
            "constructability validation_commands must not be empty"
        ));
    }
    ensure_contains_in_list(
        commands,
        RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_TEST_MARKER,
        "constructability validation commands must include focused Rust test marker",
    )?;
    ensure_contains_in_list(
        commands,
        "trace_runtime_v2_constructability_anchor_validator",
        "constructability validation commands must include CLI proof marker",
    )?;
    ensure_contains_in_list(
        commands,
        "git diff --check",
        "constructability validation commands must include whitespace/path hygiene",
    )
}

fn ensure_contains_in_list(values: &[String], needle: &str, message: &str) -> Result<()> {
    if values.iter().any(|value| value.contains(needle)) {
        Ok(())
    } else {
        Err(anyhow!("{message}"))
    }
}

fn ensure_contains(value: &str, needle: &str, message: &str) -> Result<()> {
    if value.contains(needle) {
        Ok(())
    } else {
        Err(anyhow!("{message}"))
    }
}

fn require_exact(actual: &str, expected: &str, field: &str) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(anyhow!("{field} must be '{expected}', got '{actual}'"))
    }
}
