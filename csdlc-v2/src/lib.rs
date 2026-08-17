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
pub mod output;
pub mod projection_cleanup;
pub mod projection_recovery;
pub mod proof;
pub mod publication;
pub mod pvf;
pub mod readiness;
pub mod registry;
pub mod review;
pub mod runner_preflight;
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
    artifact_reference, calibration_report, compare_cycle_time, estimation_schema_bundle, forecast,
    join_observations, load_cycle_time_evidence, load_observation_manifest, load_verified_json,
    terminal_outcome, validate_artifact_reference, validate_reference, verified_calibration,
    AcceptedEstimate, ArtifactReference, Availability, BacktestCase, CalibrationCaseArtifacts,
    CalibrationManifest, CalibrationReport, ComparableKey, Confidence, CycleTimeCohort,
    CycleTimeComparison, CycleTimeComparisonStatus, CycleTimeEvidence, DriftState,
    EstimateDisposition, EstimateMethod, Forecast, ForecastRange, MetricComparison,
    MetricObservation, Observation, ObservationManifest, ObservationSource, Provenance,
    StaticEstimate, TerminalOutcome, ValidationSourceManifest, VerifiedCalibration,
};
pub use finish::MergeMethod;
pub use finish::{
    retain_terminal_estimation_outcome, validate_terminal_estimation_evidence,
    DerivedTerminalEnvelope, FinishDisposition, FinishRequest, FinishResult,
    HistoricalFinishRequest, IssueTerminalObservation, TerminalEstimationResult,
    TerminalEstimationStatus,
};
pub use git::shared_request_path;
pub use github::{
    append_marker, execute_github_action, marker_line, ClosingPullRequestIdentity, GithubAction,
    GithubActionRequest, GithubActionResult, GithubIssuePacket, PrCheck, PrStatePacket,
    PrStateRequest,
};
pub use lifecycle::{bind_issue, initialize_native_json, BindRequest, BindResult};
pub use migration::{
    compare_shadow, generate_compatibility_view, import_legacy, migrate_bound_issue_identity,
    migrate_bound_topology, migrate_bound_topology_with_crash_for_test,
    migrate_bound_topology_with_failure_for_test, migrate_code_repository,
    migrate_initialized_code_repository, write_compatibility_view_atomic,
    BoundIssueIdentityMigrationEvidence, BoundIssueIdentityMigrationReport,
    BoundIssueIdentityMigrationRequest, BoundTopologyDisposition, BoundTopologyMigrationItem,
    BoundTopologyMigrationReport, BoundTopologyMigrationRequest, BoundTopologyMigrationResult,
    ClosedIssueEvidence, CodeRepositoryMigrationEvidence, CodeRepositoryMigrationReport,
    CodeRepositoryMigrationRequest, ImportReport, InitializedCanonicalCollisionDisposition,
    InitializedCodeRepositoryCollisionEvidence, InitializedCodeRepositoryMigrationEvidence,
    InitializedCodeRepositoryMigrationReport, InitializedCodeRepositoryMigrationRequest,
    LegacyImportRequest, MigrationIssueState, NormalizedOutcome, ShadowComparison,
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
pub use output::write_json_stdout;
pub use projection_cleanup::{
    execute_archived_projection_cleanup, ArchivedProjectionCleanupRequest,
    ArchivedProjectionCleanupResult, ArchivedProjectionCleanupStatus, ArchivedProjectionNode,
    CleanupNodeIdentity, CleanupNodeType,
};
pub use projection_recovery::{
    build_archived_projection_cleanup_request_from_recovery, classify_preserved_projection,
    recover_preserved_projection, CandidateObservation, FailedOperationLineage, ManifestEntry,
    NodeIdentity, ProjectionCasAnchor, ProjectionClassification, ProjectionClassifyRequest,
    ProjectionRecoverRequest, ProjectionRecoveryCleanupBridgeRequest,
    ProjectionRecoveryCleanupBridgeResult, ProjectionRecoveryResult,
};
pub use proof::{run_pre_switch_proof, PreSwitchEvidence, ProofManifest, ProofStep};
pub use publication::{
    prepare_publication, reconcile_action, record_publication, PublicationAction,
    PublicationIntent, PublicationLinkageMode, PublicationRequest, RemotePullRequest,
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
pub use runner_preflight::{
    inspect_runner_eligibility, CapacityState, DispatchState, OverallState, PolicyState,
    RunnerPreflightPacket, RunnerPreflightRequest, WorkflowRefObservation,
};
pub use schema::public_schema_bundle;
pub use soak::{
    decide_cutover, decide_from_evidence, generate_sample_packets, select_generation,
    BudgetEvidence, BudgetKind, CutoverDecision, Generation, GenerationSelector, ParityEvidence,
    SamplePacket, ScenarioEvidence, ScenarioOutcome, SoakDecisionPacket, SoakEvidenceInput,
    SoakScenario,
};
pub use store::{
    approve_design, edit_issue, recover_design_review, recover_initialized_decomposition,
    recover_initialized_design_envelope, ApproveDesignRequest, BootstrapRequest,
    DecompositionGraphEdge, DecompositionGraphInput, DecompositionGraphNode,
    DesignReviewRecoveryTruth, EditRequest, InitializedDecompositionRecoveryReplacement,
    InitializedDecompositionRecoveryRequest, InitializedDecompositionRecoveryResult,
    InitializedRecoveryFailurePoint, PreservedAuthoredArtifact, RecoverDesignReviewRequest,
    RecoverInitializedDesignEnvelopeRequest, Store,
};
pub use store::{recover_initialized_design_envelope_with_hook, DesignRecoveryFailpoint};
