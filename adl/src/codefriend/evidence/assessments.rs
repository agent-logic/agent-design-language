//! Mechanically verified source locations and typed assessment intent.
//! Exact source support is not proof of semantic relevance, causality or severity.
use super::{
    contracts::{Confidence, Finding, Severity, CONTRACT, REVIEW_LANES},
    hash, valid_digest, Admission,
};
use crate::codefriend::ingestion::{digest, unsafe_content};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    io::{self, Write},
};

pub const SCHEMA: &str = "codefriend.assessment_set.v1";
pub const MAX_LANE_BYTES: usize = 1024 * 1024;
pub const MAX_REVIEW_BYTES: usize = 4 * 1024 * 1024;
const MAX_QUOTE: usize = 2048;
const MAX_LANE_QUOTES: usize = 64 * 1024;

/// Counts serialization before allocating a full serialized copy or hashing it.
pub fn bounded<T: Serialize>(value: &T, limit: usize) -> Result<()> {
    struct Counter {
        left: usize,
    }
    impl Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if bytes.len() > self.left {
                return Err(io::Error::other("assessment_byte_limit"));
            }
            self.left -= bytes.len();
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    serde_json::to_writer(Counter { left: limit }, value)
        .map_err(|_| anyhow::anyhow!("assessment_byte_limit"))
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentKind {
    DefectCandidate,
    PositiveObservation,
    UnresolvedQuestion,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DefectDetails {
    pub severity: Severity,
    pub observed_behavior: String,
    pub expected_behavior: String,
    pub concrete_trigger: String,
    pub impact: String,
    pub proposed_remedy_or_verification: String,
}
fn defect_inference(defect: &DefectDetails) -> String {
    format!(
        "Impact: {}\nProposed remedy or verification: {}",
        defect.impact, defect.proposed_remedy_or_verification
    )
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProviderCitation {
    pub evidence_id: String,
    /// Legacy provider offsets are decoded for compatibility, never trusted.
    #[serde(default, skip_serializing)]
    pub start_byte: u64,
    #[serde(default, skip_serializing)]
    pub end_byte: u64,
    pub quote: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProviderAssessment {
    pub kind: AssessmentKind,
    pub summary: String,
    pub explanation: String,
    pub citations: Vec<ProviderCitation>,
    pub limitations: Vec<String>,
    pub defect: Option<DefectDetails>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProviderAssessmentOutput {
    pub assessments: Vec<ProviderAssessment>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VerifiedCitation {
    pub evidence_id: String,
    pub start_byte: u64,
    pub end_byte: u64,
    pub quote_digest: String,
}
impl VerifiedCitation {
    /// Resolves exclusively through the immutable admitted object; never opens a checkout.
    pub fn quote<'a>(&self, admission: &'a Admission) -> Result<&'a str> {
        ensure!(
            valid_digest(&self.evidence_id) && valid_digest(&self.quote_digest),
            "assessment_citation_identity"
        );
        let mut evidence = admission
            .evidence
            .iter()
            .filter(|e| e.id == self.evidence_id);
        let e = evidence
            .next()
            .ok_or_else(|| anyhow::anyhow!("assessment_evidence_unavailable"))?;
        ensure!(evidence.next().is_none(), "assessment_evidence_ambiguous");
        let object = admission
            .packet
            .objects
            .iter()
            .find(|o| o.path == e.path)
            .ok_or_else(|| anyhow::anyhow!("assessment_evidence_unavailable"))?;
        let content = object
            .content
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("assessment_evidence_unavailable"))?;
        ensure!(
            object.content_digest.as_deref() == Some(e.content_digest.as_str())
                && digest(content.as_bytes()) == e.content_digest,
            "assessment_source_digest"
        );
        let start = usize::try_from(self.start_byte)?;
        let end = usize::try_from(self.end_byte)?;
        ensure!(
            start < end && end.saturating_sub(start) <= MAX_QUOTE,
            "assessment_quote_bounds"
        );
        let quote = content
            .get(start..end)
            .ok_or_else(|| anyhow::anyhow!("assessment_utf8_span"))?;
        ensure!(
            digest(quote.as_bytes()) == self.quote_digest,
            "assessment_quote_mismatch"
        );
        Ok(quote)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Assessment {
    pub id: String,
    pub lane: String,
    pub kind: AssessmentKind,
    pub summary: String,
    pub explanation: String,
    pub citations: Vec<VerifiedCitation>,
    pub limitations: Vec<String>,
    pub defect: Option<DefectDetails>,
}
fn text(value: &str) -> Result<()> {
    ensure!(
        !value.trim().is_empty() && value.len() <= 8192 && !unsafe_content("", value),
        "assessment_text_invalid"
    );
    Ok(())
}
impl Assessment {
    fn identity(&self, admission: &Admission) -> Result<String> {
        let mut value = self.clone();
        value.id.clear();
        bounded(&value, MAX_LANE_BYTES)?;
        hash(&(SCHEMA, &admission.digest, value))
    }
    fn validate(&self, admission: &Admission) -> Result<()> {
        ensure!(
            REVIEW_LANES.contains(&self.lane.as_str()),
            "assessment_lane_invalid"
        );
        text(&self.summary)?;
        text(&self.explanation)?;
        ensure!(self.limitations.len() <= 16, "assessment_limitation_limit");
        for limitation in &self.limitations {
            text(limitation)?;
        }
        ensure!(
            !self.citations.is_empty() && self.citations.len() <= 4,
            "assessment_citation_count"
        );
        let mut previous = None;
        for citation in &self.citations {
            citation.quote(admission)?;
            let key = (
                &citation.evidence_id,
                citation.start_byte,
                citation.end_byte,
            );
            ensure!(
                previous.is_none_or(|old| old < key),
                "assessment_citations_not_canonical"
            );
            previous = Some(key);
        }
        match (&self.kind, &self.defect) {
            (AssessmentKind::DefectCandidate, Some(defect)) => {
                for value in [
                    &defect.observed_behavior,
                    &defect.expected_behavior,
                    &defect.concrete_trigger,
                    &defect.impact,
                    &defect.proposed_remedy_or_verification,
                ] {
                    text(value)?;
                }
                text(&defect_inference(defect))?;
            }
            (AssessmentKind::PositiveObservation | AssessmentKind::UnresolvedQuestion, None) => {}
            _ => anyhow::bail!("assessment_actionability_mismatch"),
        }
        ensure!(
            self.id == self.identity(admission)?,
            "assessment_identity_mismatch"
        );
        Ok(())
    }
    pub fn finding(&self, admission: &Admission) -> Result<Option<Finding>> {
        self.validate(admission)?;
        let Some(defect) = &self.defect else {
            return Ok(None);
        };
        let mut evidence: Vec<_> = self
            .citations
            .iter()
            .map(|c| c.evidence_id.clone())
            .collect();
        evidence.sort();
        evidence.dedup();
        // Logical matching deliberately excludes the content receipt, revision,
        // evidence IDs, source offsets and assessment/severity wording. Citations
        // still bind those exact bytes through the separately validated receipt.
        let mut paths: Vec<_> = self
            .citations
            .iter()
            .map(|citation| {
                admission
                    .evidence
                    .iter()
                    .find(|e| e.id == citation.evidence_id)
                    .map(|e| e.path.clone())
                    .ok_or_else(|| anyhow::anyhow!("assessment_evidence_unavailable"))
            })
            .collect::<Result<_>>()?;
        paths.sort();
        paths.dedup();
        let logical_anchor = hash(&(
            "codefriend.assessment_logical_anchor.v1",
            &self.lane,
            paths,
            &defect.observed_behavior,
            &defect.expected_behavior,
            &defect.concrete_trigger,
        ))?;
        let mut finding = Finding {
            schema: CONTRACT.into(),
            id: String::new(),
            repository: admission.packet.repository.clone(),
            perspective: self.lane.clone(),
            rule: format!("{}.assessment", self.lane),
            semantic_anchor: logical_anchor,
            title: self.summary.clone(),
            severity: defect.severity.clone(),
            rationale: self.explanation.clone(),
            confidence: Confidence::Unknown,
            evidence,
            inference: defect_inference(defect),
            scope_digest: admission.packet.scope_digest.clone(),
            limitations: self.limitations.clone(),
        };
        finding.id = finding.identity()?;
        Ok(Some(finding))
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssessmentSet {
    pub schema: String,
    pub assessments: Vec<Assessment>,
    pub digest: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct AssessmentCounts {
    pub defect_candidates: usize,
    pub positive_observations: usize,
    pub unresolved_questions: usize,
}
impl AssessmentSet {
    pub fn new(admission: &Admission, mut assessments: Vec<Assessment>) -> Result<Self> {
        assessments.sort_by(|a, b| a.id.cmp(&b.id));
        let mut set = Self {
            schema: SCHEMA.into(),
            assessments,
            digest: String::new(),
        };
        bounded(&set, MAX_REVIEW_BYTES)?;
        set.digest = hash(&(SCHEMA, &admission.digest, &set.assessments))?;
        set.validate(admission)?;
        Ok(set)
    }
    pub fn validate(&self, admission: &Admission) -> Result<()> {
        admission.validate()?;
        bounded(self, MAX_REVIEW_BYTES)?;
        ensure!(
            self.schema == SCHEMA && self.assessments.len() <= 400,
            "assessment_set_bounds"
        );
        let mut counts = std::collections::BTreeMap::<&str, (usize, usize)>::new();
        let mut previous = None;
        let mut projected = BTreeSet::new();
        for assessment in &self.assessments {
            ensure!(
                previous.is_none_or(|id: &String| id < &assessment.id),
                "assessment_set_not_canonical"
            );
            previous = Some(&assessment.id);
            assessment.validate(admission)?;
            if let Some(finding) = assessment.finding(admission)? {
                ensure!(
                    projected.insert(finding.id),
                    "assessment_logical_identity_collision"
                );
            }
            let (count, bytes) = counts.entry(&assessment.lane).or_default();
            *count += 1;
            for citation in &assessment.citations {
                *bytes += usize::try_from(citation.end_byte - citation.start_byte)?;
            }
            ensure!(
                *count <= 100 && *bytes <= MAX_LANE_QUOTES,
                "assessment_lane_bounds"
            );
        }
        ensure!(
            self.digest == hash(&(SCHEMA, &admission.digest, &self.assessments))?,
            "assessment_set_digest"
        );
        Ok(())
    }
    pub fn findings(&self, admission: &Admission) -> Result<Vec<Finding>> {
        self.validate(admission)?;
        let mut findings = self
            .assessments
            .iter()
            .filter_map(|a| a.finding(admission).transpose())
            .collect::<Result<Vec<_>>>()?;
        findings.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(findings)
    }
    pub fn counts(&self) -> AssessmentCounts {
        let mut counts = AssessmentCounts::default();
        for a in &self.assessments {
            match a.kind {
                AssessmentKind::DefectCandidate => counts.defect_candidates += 1,
                AssessmentKind::PositiveObservation => counts.positive_observations += 1,
                AssessmentKind::UnresolvedQuestion => counts.unresolved_questions += 1,
            }
        }
        counts
    }
}
/// A provider claim that could not establish all required support. Never a finding.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssessmentGap {
    pub assessment_index: usize,
    pub summary: String,
    pub reason: String,
}
/// Incomplete assessment support, bound to the actual four lane result receipts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssessmentCoverage {
    pub gaps: std::collections::BTreeMap<String, Vec<AssessmentGap>>,
    pub lane_result_digests: std::collections::BTreeMap<String, String>,
}
impl AssessmentCoverage {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            !self.gaps.is_empty() && self.gaps.len() <= 4,
            "assessment_gap_lanes"
        );
        ensure!(
            self.lane_result_digests.len() == 4
                && REVIEW_LANES.iter().all(|lane| self
                    .lane_result_digests
                    .get(*lane)
                    .is_some_and(|d| valid_digest(d))),
            "assessment_gap_receipts"
        );
        for (lane, gaps) in &self.gaps {
            ensure!(
                REVIEW_LANES.contains(&lane.as_str()) && !gaps.is_empty() && gaps.len() <= 100,
                "assessment_gap_lanes"
            );
            let mut previous = None;
            for gap in gaps {
                ensure!(
                    gap.assessment_index < 100 && previous.is_none_or(|i| i < gap.assessment_index),
                    "assessment_gap_order"
                );
                text(&gap.summary)?;
                text(&gap.reason)?;
                previous = Some(gap.assessment_index);
            }
        }
        bounded(self, MAX_REVIEW_BYTES)
    }
    pub fn descriptions(&self) -> Vec<String> {
        self.gaps
            .iter()
            .flat_map(|(lane, gaps)| {
                gaps.iter().map(move |gap| {
                    format!(
                        "Unverified {lane} assessment {}: {} ({})",
                        gap.assessment_index + 1,
                        gap.summary,
                        gap.reason
                    )
                })
            })
            .collect()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedAssessments {
    pub assessments: Vec<Assessment>,
    pub gaps: Vec<AssessmentGap>,
}

fn provider_json(raw: &str) -> Result<&str> {
    let raw = raw.trim();
    if !raw.starts_with("```") {
        return Ok(raw);
    }
    let (opening, rest) = raw
        .split_once('\n')
        .ok_or_else(|| anyhow::anyhow!("assessment_json_invalid"))?;
    ensure!(
        matches!(opening.trim_end_matches('\r'), "```" | "```json"),
        "assessment_json_invalid"
    );
    let (body, closing) = rest
        .rsplit_once('\n')
        .ok_or_else(|| anyhow::anyhow!("assessment_json_invalid"))?;
    ensure!(closing == "```", "assessment_json_invalid");
    Ok(body)
}

/// Bounded wire decoding only; callers must verify support against admission.
pub fn decode_provider_output(raw: &str) -> Result<ProviderAssessmentOutput> {
    ensure!(raw.len() <= MAX_LANE_BYTES, "assessment_lane_byte_limit");
    let output: ProviderAssessmentOutput = serde_json::from_str(provider_json(raw)?)
        .map_err(|_| anyhow::anyhow!("assessment_json_invalid"))?;
    ensure!(output.assessments.len() <= 100, "assessment_lane_count");
    Ok(output)
}

fn resolve_quote(c: &ProviderCitation, admission: &Admission) -> Result<VerifiedCitation> {
    let mut evidence = admission.evidence.iter().filter(|e| e.id == c.evidence_id);
    let e = evidence
        .next()
        .ok_or_else(|| anyhow::anyhow!("assessment_evidence_unavailable"))?;
    ensure!(evidence.next().is_none(), "assessment_evidence_ambiguous");
    let object = admission
        .packet
        .objects
        .iter()
        .find(|o| o.path == e.path)
        .ok_or_else(|| anyhow::anyhow!("assessment_evidence_unavailable"))?;
    let content = object
        .content
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("assessment_evidence_unavailable"))?;
    ensure!(
        object.content_digest.as_deref() == Some(e.content_digest.as_str())
            && digest(content.as_bytes()) == e.content_digest,
        "assessment_source_digest"
    );
    // Byte windows include overlapping occurrences. Neither offsets nor another
    // file can disambiguate provider evidence; exact source bytes are authority.
    let mut matches = content
        .as_bytes()
        .windows(c.quote.len())
        .enumerate()
        .filter(|(_, bytes)| *bytes == c.quote.as_bytes())
        .map(|(i, _)| i);
    let start = matches
        .next()
        .ok_or_else(|| anyhow::anyhow!("assessment_quote_mismatch"))?;
    ensure!(matches.next().is_none(), "assessment_quote_ambiguous");
    let citation = VerifiedCitation {
        evidence_id: c.evidence_id.clone(),
        start_byte: start as u64,
        end_byte: (start + c.quote.len()) as u64,
        quote_digest: digest(c.quote.as_bytes()),
    };
    ensure!(
        citation.quote(admission)? == c.quote,
        "assessment_quote_mismatch"
    );
    Ok(citation)
}

/// Strict caller compatibility: gaps cannot masquerade as a complete lane.
pub fn parse_lane(lane: &str, raw: &str, admission: &Admission) -> Result<Vec<Assessment>> {
    let parsed = parse_lane_with_gaps(lane, raw, admission)?;
    if let Some(gap) = parsed.gaps.first() {
        anyhow::bail!("{}", gap.reason);
    }
    Ok(parsed.assessments)
}

/// Preserve independently supported siblings, and explicit gaps for rejected claims.
pub fn parse_lane_with_gaps(
    lane: &str,
    raw: &str,
    admission: &Admission,
) -> Result<ParsedAssessments> {
    ensure!(raw.len() <= MAX_LANE_BYTES, "assessment_lane_byte_limit");
    admission.validate()?;
    let output = decode_provider_output(raw)?;
    ensure!(output.assessments.len() <= 100, "assessment_lane_count");
    // Enforce aggregate bounds even on quotes in rejected assessments.
    let quoted: usize = output
        .assessments
        .iter()
        .flat_map(|a| &a.citations)
        .map(|c| c.quote.len())
        .sum();
    ensure!(quoted <= MAX_LANE_QUOTES, "assessment_lane_quote_limit");
    let mut result = Vec::new();
    let mut gaps = Vec::new();
    for (assessment_index, input) in output.assessments.into_iter().enumerate() {
        let summary = if text(&input.summary).is_ok() {
            input.summary.clone()
        } else {
            "Unverified assessment".into()
        };
        let parsed: Result<Assessment> = (|| {
            ensure!(
                !input.citations.is_empty() && input.citations.len() <= 4,
                "assessment_citation_count"
            );
            let mut citations = Vec::new();
            let mut seen = BTreeSet::new();
            for c in input.citations {
                ensure!(
                    !c.quote.is_empty() && c.quote.len() <= MAX_QUOTE,
                    "assessment_quote_bounds"
                );

                let citation = resolve_quote(&c, admission)?;
                ensure!(
                    seen.insert((
                        citation.evidence_id.clone(),
                        citation.start_byte,
                        citation.end_byte
                    )),
                    "assessment_duplicate_citation"
                );
                citations.push(citation);
            }
            citations.sort_by(|a, b| {
                (&a.evidence_id, a.start_byte, a.end_byte).cmp(&(
                    &b.evidence_id,
                    b.start_byte,
                    b.end_byte,
                ))
            });
            let mut assessment = Assessment {
                id: String::new(),
                lane: lane.into(),
                kind: input.kind,
                summary: input.summary,
                explanation: input.explanation,
                citations,
                limitations: input.limitations,
                defect: input.defect,
            };
            assessment.id = assessment.identity(admission)?;
            assessment.validate(admission)?;
            Ok(assessment)
        })();
        match parsed {
            Ok(assessment) => result.push(assessment),
            Err(error) => gaps.push(AssessmentGap {
                assessment_index,
                summary,
                reason: error.to_string(),
            }),
        }
    }
    AssessmentSet::new(admission, result.clone())?;
    Ok(ParsedAssessments {
        assessments: result,
        gaps,
    })
}
