//! Minimal non-authoritative C-SDLC v3 contract boundary.
//!
//! This crate is introduced by ADL issue #500 as a construction-decision and
//! validation surface only. It intentionally does not expose lifecycle
//! execution, GitHub mutation, worktree binding, finish, cleanup, or authority
//! cutover APIs.

/// The predecessor issues retained by the V3-A contract.
pub const PREDECESSOR_DENOMINATOR: [u64; 3] = [161, 162, 163];

/// Requirement-level retained predecessor denominator for V3-A.
pub const PREDECESSOR_REQUIREMENTS: [&str; 19] = [
    "command-tree",
    "state-output-contracts",
    "capability-matrix",
    "reviewer-independence",
    "publication-linkage",
    "finish-cleanup-migration",
    "supported-platform-matrix",
    "state-size-guard",
    "output-filter-subset",
    "four-layer-rust-shape",
    "dependency-thresholds",
    "parser-template-boundaries",
    "github-client-capability-gaps",
    "commit-primitive-recommendation",
    "promote-or-discard-disposition",
    "platform-commit-matrix",
    "durability-semantics",
    "windows-fail-closed-posture",
    "operator-decision",
];

/// Lifecycle surfaces classified by the V3-A proportional-lifecycle decision.
pub const PROPORTIONAL_SURFACES: [&str; 15] = [
    "sip",
    "stp",
    "spp",
    "vpp",
    "srp",
    "sor",
    "design_review",
    "readiness",
    "bind",
    "implementation_review",
    "publication",
    "finish",
    "cleanup",
    "sprint_umbrella_review",
    "generation_digest_cas",
];

/// Returns true only for the explicit retained V3-A predecessor denominator.
pub fn is_v3a_predecessor(issue: u64) -> bool {
    PREDECESSOR_DENOMINATOR.contains(&issue)
}

/// Returns true only for surfaces in the V3-A proportional-lifecycle denominator.
pub fn is_proportional_surface(surface: &str) -> bool {
    PROPORTIONAL_SURFACES.contains(&surface)
}

/// V3 is not operational authority during V3-A.
pub fn operational_authority() -> &'static str {
    "csdlc-v2"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("manifest has parent")
            .to_path_buf()
    }

    fn read_repo(path: &str) -> String {
        fs::read_to_string(repo_root().join(path)).expect(path)
    }

    #[test]
    fn contract_schema() {
        let contract = read_repo("docs/csdlc-v3/CONTRACT.md");
        assert!(contract.contains("C-SDLC v3 Contract"));
        assert!(contract.contains("v2 remains the sole operational authority"));
        assert!(contract.contains("Authority and compatibility"));
        assert!(contract.contains("Retained predecessor contract"));
        assert!(contract.contains("Construction decision"));
        assert!(contract.contains("Proportional lifecycle contract"));
        assert!(contract.contains("Rollback and fail-closed behavior"));
        assert_eq!(operational_authority(), "csdlc-v2");
    }

    #[test]
    fn predecessor_coverage() {
        let coverage = read_repo("docs/csdlc-v3/predecessor-coverage.json");
        for issue in PREDECESSOR_DENOMINATOR {
            assert!(coverage.contains(&format!("\"issue\": {issue}")));
            assert!(is_v3a_predecessor(issue));
        }
        assert!(!is_v3a_predecessor(160));
        assert!(!is_v3a_predecessor(164));
        assert!(coverage.contains("\"denominator\": [161, 162, 163]"));
        assert!(!coverage.contains("\"requirement_ids\""));
        for requirement in PREDECESSOR_REQUIREMENTS {
            assert!(coverage.contains(&format!("\"id\": \"{requirement}\"")));
            assert!(coverage.contains(&format!("\"disposition\": \"retained\"")));
            assert!(coverage.contains("\"maps_to\""));
        }
        assert_eq!(
            PREDECESSOR_REQUIREMENTS
                .into_iter()
                .collect::<BTreeSet<_>>()
                .len(),
            PREDECESSOR_REQUIREMENTS.len()
        );
        assert_eq!(
            PREDECESSOR_DENOMINATOR.into_iter().collect::<BTreeSet<_>>(),
            BTreeSet::from([161, 162, 163])
        );
    }

    #[test]
    fn architecture_boundary() {
        let contract = read_repo("docs/csdlc-v3/CONTRACT.md");
        let forbidden_claims = [
            "v3 is the sole operational authority",
            "v3 becomes the sole operational authority",
            "v3 has operational authority",
            "v3 approves v2 retirement",
            "v3 authorizes v2 retirement",
            "v3 completes v2 retirement",
        ];
        let contract_lower = contract.to_lowercase();
        for claim in forbidden_claims {
            assert!(
                !contract_lower.contains(claim),
                "forbidden authority claim present: {claim}"
            );
        }
        assert!(contract.contains("does not make v3 operational"));
        assert!(contract.contains("v2 remains the rollback target"));
        assert!(contract.contains("Windows mutation remains fail-closed/read-only"));
    }

    #[test]
    fn proportional_lifecycle() {
        let matrix = read_repo("docs/csdlc-v3/proportional-lifecycle.json");
        for surface in PROPORTIONAL_SURFACES {
            assert!(matrix.contains(&format!("\"id\": \"{surface}\"")));
            assert!(is_proportional_surface(surface));
        }
        assert!(!is_proportional_surface("umbrella_re_review"));
        assert!(matrix.contains("\"design_gates\": 1"));
        assert!(matrix.contains("\"validation\": \"focused\""));
        assert!(matrix.contains("\"implementation_reviews\": 1"));
        assert!(matrix.contains("\"closeouts\": 1"));
        assert!(matrix.contains("\"three_issue_ready_minutes_max\": 3"));
        assert!(matrix.contains("\"duplicate_authority\": \"forbidden\""));
        assert!(matrix.contains("\"umbrella_repeats_child_proof\": false"));
        assert_eq!(
            PROPORTIONAL_SURFACES
                .into_iter()
                .collect::<BTreeSet<_>>()
                .len(),
            PROPORTIONAL_SURFACES.len()
        );
    }
}
