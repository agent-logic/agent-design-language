#!/usr/bin/env ruby
# frozen_string_literal: true

require "set"

ROOT = File.expand_path("../../../..", __dir__)
FACADE = File.join(ROOT, "adl/src/resilience.rs")
MODULE_DIR = File.join(ROOT, "adl/src/resilience")

EXPECTED_PUBLIC = %w[
  RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1
  RESILIENCE_CITIZEN_HEALTH_SCHEMA_V1
  RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1
  RESILIENCE_CHECKPOINT_SCHEMA_V1
  RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1
  RESILIENCE_RETRY_ATTEMPT_SCHEMA_V1
  RESILIENCE_RETRY_EXECUTION_TRACE_SCHEMA_V1
  RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1
  RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1
  RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1
  RESILIENCE_RATE_LIMIT_EXECUTION_TRACE_SCHEMA_V1
  RESILIENCE_RATE_LIMIT_STATE_SCHEMA_V1
  RESILIENCE_BULKHEAD_EXECUTION_TRACE_SCHEMA_V1
  RESILIENCE_BULKHEAD_STATE_SCHEMA_V1
  RESILIENCE_FALLBACK_EXECUTION_TRACE_SCHEMA_V1
  RESILIENCE_POLICY_SCHEMA_V1
  RESILIENCE_SUBSTRATE_SCHEMA_V1
  RUNTIME_RESILIENCE_TRACE_SCHEMA_V1
  RUNTIME_CORRELATION_FIELDS_SCHEMA_V1
  RUNTIME_HEALTH_STATUS_SCHEMA_V1
  RuntimeHealthStateV1
  RuntimeResilienceDispositionV1
  RuntimeResilienceTraceV1
  RuntimeCorrelationFieldsV1
  RuntimeHealthStatusV1
  remote_exec_health_payload
  ResilienceSurfaceV1
  ResilienceFaultClassV1
  ResilienceFaultDispositionV1
  ResilienceFaultClassificationV1
  CitizenHealthStateV1
  CitizenHealthRecordV1
  RecoveryDispositionV1
  RecoveryArtifactV1
  CheckpointKindV1
  CheckpointRecordV1
  TelemetryEventKindV1
  ResilienceTelemetryEventV1
  RetryPolicyV1
  RetryTerminalReasonV1
  RetryAttemptRecordV1
  RetryExecutionFinalStatusV1
  RetryExecutionTraceV1
  RetryExecution
  TimeoutPolicyV1
  TimeoutBreachKindV1
  TimeoutExecutionFinalStatusV1
  TimeoutObservation
  TimeoutExecutionTraceV1
  TimeoutExecution
  CircuitBreakerStateKindV1
  CircuitBreakerFinalStatusV1
  CircuitBreakerStateV1
  CircuitBreakerExecutionTraceV1
  CircuitBreakerExecution
  CircuitBreakerPolicyV1
  RateLimitPolicyV1
  RateLimitFinalStatusV1
  RateLimitStateV1
  RateLimitExecutionTraceV1
  RateLimitExecution
  BulkheadFinalStatusV1
  BulkheadStateV1
  BulkheadExecutionTraceV1
  BulkheadExecution
  FallbackExecutionFinalStatusV1
  FallbackOutcomeKindV1
  FallbackExecutionTraceV1
  FallbackExecution
  BulkheadPolicyV1
  FallbackPolicyV1
  ResiliencePolicyV1
  ResilienceSubstrateManifestV1
  execute_retry_policy
  execute_timeout_policy
  circuit_breaker_initial_state
  execute_circuit_breaker_policy
  rate_limit_initial_state
  bulkhead_initial_state
  execute_bulkhead_policy
  execute_rate_limit_policy
  resilience_schema_smoke
  execute_fallback_policy
].to_set

EXPECTED_PUBLIC_METHODS = %w[
  RuntimeResilienceDispositionV1::as_str
  RuntimeCorrelationFieldsV1::new
  RuntimeCorrelationFieldsV1::from_trace_event
  RuntimeCorrelationFieldsV1::field_contract
  RuntimeHealthStatusV1::healthy_runtime_component
  RuntimeHealthStatusV1::to_json_value
  ResilienceFaultClassificationV1::provider
  ResiliencePolicyV1::provider_attempt_policy
  ResilienceSubstrateManifestV1::phase1_foundation
].to_set

paths = [FACADE] + Dir[File.join(MODULE_DIR, "*.rs")].reject { |path| path.end_with?("/tests.rs") }
declared = paths.each_with_object(Set.new) do |path, names|
  current_impl = nil
  File.readlines(path).each do |line|
    current_impl = Regexp.last_match(1) if line =~ /^impl\s+([A-Za-z0-9_]+)/
    current_impl = nil if line =~ /^}/
    names << Regexp.last_match(1) if line =~ /^pub\s+(?:const|struct|enum|fn)\s+([A-Za-z0-9_]+)/
  end
end
declared_methods = paths.each_with_object(Set.new) do |path, names|
  current_impl = nil
  File.readlines(path).each do |line|
    current_impl = Regexp.last_match(1) if line =~ /^impl\s+([A-Za-z0-9_]+)/
    names << "#{current_impl}::#{Regexp.last_match(1)}" if current_impl && line =~ /^\s+pub\s+fn\s+([A-Za-z0-9_]+)/
    current_impl = nil if line =~ /^}/
  end
end

facade_exports = File.read(FACADE).scan(/\b[A-Za-z][A-Za-z0-9_]*\b/).to_set
missing_declarations = EXPECTED_PUBLIC - declared
missing_facade_exports = EXPECTED_PUBLIC - facade_exports
unexpected_public = declared - EXPECTED_PUBLIC
missing_methods = EXPECTED_PUBLIC_METHODS - declared_methods
unexpected_public_methods = declared_methods - EXPECTED_PUBLIC_METHODS

unless missing_declarations.empty? && missing_facade_exports.empty? && unexpected_public.empty? && missing_methods.empty? && unexpected_public_methods.empty?
  warn "RUST-01 api parity failed"
  warn "missing declarations: #{missing_declarations.to_a.sort.join(", ")}" unless missing_declarations.empty?
  warn "missing facade exports: #{missing_facade_exports.to_a.sort.join(", ")}" unless missing_facade_exports.empty?
  warn "unexpected public declarations: #{unexpected_public.to_a.sort.join(", ")}" unless unexpected_public.empty?
  warn "missing public methods: #{missing_methods.to_a.sort.join(", ")}" unless missing_methods.empty?
  warn "unexpected public methods: #{unexpected_public_methods.to_a.sort.join(", ")}" unless unexpected_public_methods.empty?
  exit 1
end

puts "RUST-01 api parity passed: #{EXPECTED_PUBLIC.size} public resilience declarations and #{EXPECTED_PUBLIC_METHODS.size} public inherent methods preserved"
