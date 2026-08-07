#![recursion_limit = "256"]

pub mod cards;
pub mod cleanup;
pub mod cutover;
pub mod doctor;
pub mod eligibility;
pub mod error;
pub mod estimation;
pub mod finish;
pub mod git;
pub mod github;
pub mod github_token;
pub mod lifecycle;
pub mod migration;
pub mod model;
pub mod operator;
pub mod proof;
pub mod publication;
pub mod pvf;
pub mod readiness;
pub mod registry;
pub mod review;
pub mod schema;
pub mod soak;
pub mod store;

pub use cards::{
    CardKind, CardStatus, CardValues, InitialCardInput, PlanningCollectionField, PlanningProfile,
    SemanticOperation,
};
pub use cleanup::{
    build_legacy_terminal_index, cleanup_schema_bundle, execute_cleanup, validate_terminal_census,
    CleanupOperation, CleanupRequest, CleanupResult, CleanupStatus, LegacyTerminalEntry,
    LegacyTerminalIndex, LegacyTerminalIndexRequest, TerminalCensusReport,
};
pub use cutover::{run_cutover, CutoverEvidence, CutoverRequest};
pub use doctor::{diagnose, DoctorReport};
pub use eligibility::{
    eligibility_schema_bundle, evaluate_deletion_eligibility, DeletionApproval, DeletionDecision,
    DeletionEligibilityRequest, DeletionEntry, DeletionManifest, DeletionReason, EntryDisposition,
};
pub use error::{ErrorCode, Result, V2Error};
pub use estimation::{
    calibration_report, compare_cycle_time, estimation_schema_bundle, forecast, join_observations,
    terminal_outcome, AcceptedEstimate, Availability, BacktestCase, CalibrationReport,
    ComparableKey, Confidence, CycleTimeCohort, CycleTimeComparison, DriftState,
    EstimateDisposition, EstimateMethod, Forecast, ForecastRange, MetricComparison,
    MetricObservation, Observation, ObservationSource, Provenance, StaticEstimate, TerminalOutcome,
};
pub use finish::MergeMethod;
pub use finish::{
    validate_terminal_estimation_evidence, DerivedTerminalEnvelope, FinishDisposition,
    FinishRequest, FinishResult, IssueTerminalObservation,
};
pub use git::shared_request_path;
pub use github::{
    append_marker, execute_github_action, marker_line, GithubAction, GithubActionRequest,
    GithubActionResult, GithubIssuePacket, PrCheck, PrStatePacket, PrStateRequest,
};
pub use lifecycle::{bind_issue, initialize_native_json, BindRequest, BindResult};
pub use migration::{
    compare_shadow, generate_compatibility_view, import_legacy, migrate_bound_topology,
    migrate_bound_topology_with_crash_for_test, migrate_bound_topology_with_failure_for_test,
    write_compatibility_view_atomic, BoundTopologyDisposition, BoundTopologyMigrationItem,
    BoundTopologyMigrationReport, BoundTopologyMigrationRequest, BoundTopologyMigrationResult,
    ClosedIssueEvidence, ImportReport, LegacyImportRequest, MigrationIssueState, NormalizedOutcome,
    ShadowComparison,
};
pub use model::{
    DesignReview, IssueRecord, LifecyclePhase, MigrationEvidence, NonSubstantiveProof,
    PublicationEvidence, ReadinessEvidence, ReviewAssignment, ReviewEvidence,
    ReviewFindingEvidence, TerminalEvidence, TerminalReceipt,
};
pub use operator::{
    build_and_install_binaries, install_binaries, resolve_operator_generation, verify_coexistence,
    CoexistenceInventory, InstallReceipt, SkillManifest,
};
pub use proof::{run_pre_switch_proof, PreSwitchEvidence, ProofManifest, ProofStep};
pub use publication::{
    prepare_publication, reconcile_action, record_publication, PublicationAction,
    PublicationIntent, PublicationRequest, RemotePullRequest,
};
pub use pvf::{
    classify_schedule, classify_shepherd, execute, finalize, select, ExecutionRequest,
    FinalizeRequest, PvfManifest, ScheduleInput, ShepherdInput,
};
pub use readiness::{
    classify_readiness, CheckConclusion, CheckObservation, CheckRequirement, ConflictState,
    PostPublicationFinding, ReadinessReport, ReadinessRequest, RemoteReviewState,
    TerminalDisposition,
};
pub use review::{
    assign_review, evaluate_publication_review, evaluate_publication_review_in_repo, record_review,
    recover_review, PublicationReviewReport, ReviewAssignmentRequest, ReviewRecordRequest,
    ReviewRecoveryRequest,
};
pub use schema::public_schema_bundle;
pub use soak::{
    decide_cutover, decide_from_evidence, generate_sample_packets, select_generation,
    BudgetEvidence, BudgetKind, CutoverDecision, Generation, GenerationSelector, ParityEvidence,
    SamplePacket, ScenarioEvidence, ScenarioOutcome, SoakDecisionPacket, SoakEvidenceInput,
    SoakScenario,
};
pub use store::{
    approve_design, edit_issue, ApproveDesignRequest, BootstrapRequest, EditRequest, Store,
};
