# frozen_string_literal: true

module FeatureDecisions5594
  GROUPS = {
    "K" => {
      classification: "kernel_continuity_ingress",
      owner_issues: [5591],
      disposition: "blocked_pending_runtime_v3_parity_or_explicit_non_runtime_disposition",
      basis: "Runtime execution, lifecycle, continuity, replay, or canonical-ingress behavior must be proved on the Runtime v3 kernel path."
    },
    "R" => {
      classification: "reasoning_adaptive_cognition",
      owner_issues: [5592],
      disposition: "blocked_pending_runtime_v3_parity_or_explicit_non_runtime_disposition",
      basis: "Reasoning, memory, cognition, affect, or adaptive behavior must be proved or explicitly dispositioned by Runtime v3 Parity-B."
    },
    "O" => {
      classification: "governed_operations",
      owner_issues: [5589],
      disposition: "blocked_pending_runtime_v3_parity_or_explicit_non_runtime_disposition",
      basis: "Governance, identity, provider, state, time, tool, or operational-service behavior belongs to Runtime v3 Parity-C."
    },
    "A" => {
      classification: "secure_access_observatory",
      owner_issues: [5590],
      disposition: "blocked_pending_runtime_v3_parity_or_explicit_non_runtime_disposition",
      basis: "Secure local or remote access, communications, telemetry, guardian, or Observatory behavior belongs to Runtime v3 Parity-D."
    },
    "C" => {
      classification: "csdlc_v2_acceptance",
      owner_issues: [5358],
      disposition: "external_owner_acceptance_required",
      basis: "This is a C-SDLC authoring, review, validation, quality, or control-plane capability governed by C-SDLC v2 acceptance rather than Runtime v3 parity."
    },
    "L" => {
      classification: "adl_v2_acceptance",
      owner_issues: [5336],
      disposition: "external_owner_acceptance_required",
      basis: "This is an ADL tooling, adapter, or integration capability governed by ADL v2 acceptance rather than Runtime v3 parity."
    },
    "S" => {
      classification: "adl_v2_signing",
      owner_issues: [5342],
      disposition: "external_owner_acceptance_required",
      basis: "Signing, verification, and trust-policy replacement is explicitly owned by ADL v2 WP-07 rather than Runtime v3 governed-operations parity."
    },
    "D" => {
      classification: "retained_or_later_milestone",
      owner_issues: [5347],
      disposition: "deferred_to_canonical_next_target",
      basis: "This feature is retained evidence, a product or demonstration surface, or explicitly owned by its canonical later milestone; WP-17 prevents deletion without disposition."
    }
  }.freeze

  # Explicit, source-line-pinned decisions for every feature row in the canonical
  # matrix. The source digest in the generated artifact makes line drift fail
  # closed; comments keep each decision human-reviewable.
  BY_SOURCE_LINE = {
    209 => "K", # Deterministic workflow execution
    210 => "K", # ExecutionPlan runtime
    211 => "K", # Sequential + fork/join coordination
    212 => "K", # Bounded concurrency and retry/failure controls
    213 => "K", # Run artifacts and replay-oriented inspection
    214 => "S", # Signing, verification, and trust policy
    215 => "A", # Provider and transport substrate
    216 => "A", # Remote execution baseline
    217 => "O", # Human-in-the-loop pause/resume
    218 => "C", # Structured authoring model
    219 => "C", # Structured planning and Structured Review Prompt workflow
    220 => "C", # Control-plane lifecycle
    221 => "C", # Editor and command-adapter surfaces
    222 => "C", # Review and validation surfaces
    223 => "C", # Task-bundle workflow
    224 => "R", # Agency, cognitive loop, and cognitive stack
    225 => "R", # Fast/slow thinking and cognitive arbitration
    226 => "R", # Bounded Godel loop
    227 => "R", # Godel agents and Godel-Hadamard-Bayes algorithm
    228 => "R", # ObsMem indexing, retrieval, and evidence-aware ranking
    229 => "R", # Shared ObsMem foundation
    230 => "O", # Trace validation, trace review, and trace-to-memory ingestion
    231 => "R", # Bounded cognitive path
    232 => "O", # Freedom Gate baseline
    233 => "O", # Freedom Gate v2
    234 => "K", # Trace substrate
    235 => "L", # Operational skills substrate
    236 => "K", # Runtime environment and lifecycle completion
    237 => "K", # Execution boundaries and capability-aware local execution
    238 => "K", # Local runtime resilience and Shepherd preservation
    239 => "O", # Chronosense / temporal substrate
    240 => "O", # Temporal query, retrieval, identity semantics, and continuity hooks
    241 => "O", # Commitments, deadlines, and bounded temporal causality
    242 => "O", # Cost model, accounting primitives, and bounded economics hooks
    243 => "D", # PHI-style integration metrics
    244 => "R", # Instinct and bounded agency
    245 => "D", # Paper Sonata public-facing proof surface
    246 => "D", # Deep-agents comparative proof
    247 => "O", # AEE 1.0 convergence
    248 => "O", # Decision, action, and skill-governance surfaces
    249 => "O", # Delegation, refusal, and coordination contracts
    250 => "A", # Provider-extension packaging and safe extension boundaries
    251 => "A", # Security, posture, and trust-under-adversary package
    252 => "A", # Adversarial runtime, exploit/replay, and self-attack band
    253 => "C", # Demo proof entry points and quality gate
    254 => "D", # Five-agent Hey Jude MIDI demo
    255 => "D", # arXiv paper writer and three-paper program
    256 => "K", # Long-lived supervisor, heartbeat, and cycle artifacts
    257 => "D", # Stock-league long-lived demo family
    258 => "A", # Minimal status/inspection boundary
    259 => "D", # CodeFriend review showcase and architecture-document generation
    260 => "C", # Coverage ratchet, test tracker, and quality tracking
    261 => "C", # Rust refactoring tracker and evidence-driven maintenance
    262 => "L", # Milestone compression and repo visibility prototypes
    263 => "D", # HTML milestone dashboard and compression reporting
    264 => "K", # Runtime v2 foundation prototype
    265 => "A", # CSM Observatory visibility and operator-report surfaces
    266 => "K", # Runtime v2 hardening, recovery, quarantine, and expanded invariants
    267 => "K", # First bounded CSM run
    268 => "C", # Third-party review and review-quality gates
    269 => "D", # ANRM / shepherd-model experiments
    270 => "D", # CSM Shepherd model and Gemma training path
    271 => "D", # Capability-testing evidence and Aptitude Atlas boundary
    272 => "O", # Governed tool calls and capability contracts
    273 => "D", # Cognitive Compression Cost instrumentation
    274 => "D", # Web-based code editor integration
    275 => "R", # Reasoning graph baseline
    276 => "O", # Signed trace and trace query
    277 => "R", # Wellbeing, affect, kindness, moral cognition, humor
    278 => "A", # Secure Agent Communication and Invocation Protocol
    279 => "K", # Inhabited runtime readiness
    280 => "K", # Runtime/polis architecture alignment
    281 => "O", # Agent lifecycle state model
    282 => "A", # CSM Observatory active agent runtime
    283 => "O", # Citizen standing and citizen state follow-on
    284 => "R", # Memory, Theory of Mind, capability testing, intelligence metrics, governed learning, and ANRM/Gemma
    285 => "A", # ACIP hardening and local encryption boundary
    286 => "A", # A2A adapter boundary
    287 => "K", # Runtime inhabitant proof
    288 => "L", # UTS + ACC multi-model benchmark and provider-native tool-call comparison
    289 => "C", # Runtime/test-cycle recovery and coverage ergonomics
    290 => "D", # CodeFriend repo-review product layer
    291 => "C", # Review heuristics and reviewer demo lane
    292 => "L", # Google Workspace CMS bridge and Rust-native adapter boundary
    293 => "D", # Automated repository modernization and external refactoring integration
    294 => "D", # Generic speculative decoding runtime acceleration
    295 => "L", # Repo visibility follow-on
    296 => "D", # Publication packet program and GHB paper lane
    297 => "D", # General-intelligence paper packet
    298 => "C", # Rustdoc/doc cleanup
    299 => "C", # Workflow guardrails
    300 => "C", # Cognitive SDLC first slice and transition manifest
    301 => "C", # Cognitive SDLC default operation and five-minute-sprint repeatability
    302 => "A", # Logging, observability, and OTel-compatible proof-loop readiness
    303 => "K", # Resilience, citizen persistence, and operational sleep/wake
    304 => "C", # Public prompt records export, redaction, validation, and indexing
    305 => "O", # Provider/model reliability and multi-agent readiness
    306 => "A", # Security readiness and Continuous Adversarial Verification
    307 => "R", # Curiosity Engine and Discovery Substrate
    308 => "R", # Constructability Gate for shared ADL reality
    309 => "R", # Reasoning graph, loop runtime, and adl.skill.v1
    310 => "R", # ACP / cognitive profiles runtime surface
    311 => "A", # ACIP binary schema and WebSocket carrier
    312 => "O", # Identity, stable name, and continuity substrate
    313 => "O", # Memory grounding, capability envelope, and birth witnesses/receipt
    314 => "R", # Memory Palace navigable context topology
    315 => "R", # First true Godel-agent birthday
    316 => "O", # Constitutional citizenship, rights/duties, and governance review
    317 => "R", # Bounded Theory of Mind, relationship, reputation, and shared social memory boundary
    318 => "O", # Delegation, upstream delegation, IAM, standing transition, and challenge/appeal governance
    319 => "O", # Guilds and collective organization
    320 => "A", # Enterprise security for the ADL polis
    321 => "A", # Secure execution, policy, identity/auth, isolation, and provider-trust convergence
    322 => "R", # Mental time travel / temporal self-projection
    323 => "D", # Payments, settlement, economic agency, and x402 / Lightning adapters
    324 => "O", # Bounded contract-market and resource-stewardship bridge
    325 => "K", # Distributed execution integration
    326 => "D", # CodeFriend v1 and portable adapter v2
    327 => "D", # Capability-testing evidence consumption / Aptitude Atlas boundary
    328 => "D", # Demo catalog and polished MVP walkthrough
    329 => "C", # Control-plane Rust migration / tooling hardening
    330 => "D"  # Zed integration
  }.freeze
end
