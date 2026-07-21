use crate::model::{
    CancelRequest, CompletionOutcome, EngineEffect, EngineError, EngineErrorCode, EngineEvent,
    EngineLimits, EnginePolicy, EngineSnapshot, EventKind, FailureClass, JoinPolicy, NodeSnapshot,
    NodeState, PortCompletion, PortFailure, PortKind, ProviderRequest, ToolRequest, TurnInput,
    TurnOutput, CHECKPOINT_CONTRACT_VERSION, ENGINE_CONTRACT_VERSION,
};
use adl_compiler::{canonical_plan_bytes, ExecutionPlan, PlanNode, EXECUTION_PLAN_VERSION};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const PLAN_DIGEST_DOMAIN: &[u8] = b"adl.engine.plan.v1\0";
const POLICY_DIGEST_DOMAIN: &[u8] = b"adl.engine.policy.v1\0";
const EDGE_DIGEST_DOMAIN: &[u8] = b"adl.engine.edge.v1\0";
const REQUEST_ID_DOMAIN: &[u8] = b"adl.engine.request.v1\0";
const IDEMPOTENCY_DOMAIN: &[u8] = b"adl.engine.idempotency.v1\0";
const CANCEL_IDEMPOTENCY_DOMAIN: &[u8] = b"adl.engine.cancel.v1\0";
const COMPLETION_DIGEST_DOMAIN: &[u8] = b"adl.engine.completion.v1\0";

#[derive(Debug, Clone)]
pub struct Engine {
    plan: ExecutionPlan,
    policy: EnginePolicy,
    predecessors: BTreeMap<String, Vec<String>>,
    nodes: BTreeMap<String, PlanNode>,
    snapshot: EngineSnapshot,
}

impl Engine {
    pub fn new(
        plan: ExecutionPlan,
        policy: EnginePolicy,
        limits: EngineLimits,
    ) -> Result<Self, EngineError> {
        validate_limits(&limits)?;
        let (nodes, predecessors, node_ids, edge_ids) = validate_plan(&plan, &limits)?;
        validate_policy(&plan, &policy, &predecessors, &limits)?;

        let plan_bytes = canonical_plan_bytes(&plan).map_err(|error| {
            EngineError::new(EngineErrorCode::Serialization, "plan", &error.to_string())
        })?;
        let policy_bytes = encode(&policy, "policy")?;
        let plan_digest = hash_parts(PLAN_DIGEST_DOMAIN, &[&plan_bytes]);
        let policy_digest = hash_parts(POLICY_DIGEST_DOMAIN, &[&policy_bytes]);

        validate_request_envelopes(&plan, &policy, &nodes, &plan_digest, &limits)?;

        let mut node_states = BTreeMap::new();
        for node_id in &node_ids {
            node_states.insert(
                node_id.clone(),
                NodeSnapshot {
                    state: NodeState::Pending,
                    attempts: 0,
                },
            );
        }
        let snapshot = EngineSnapshot {
            checkpoint_contract: String::from(CHECKPOINT_CONTRACT_VERSION),
            engine_contract: String::from(ENGINE_CONTRACT_VERSION),
            plan_contract: plan.contract.clone(),
            plan_source_digest: plan.source_digest.clone(),
            plan_digest,
            policy_digest,
            node_ids,
            edge_ids,
            limits,
            logical_tick: 0,
            logical_turns: 0,
            attempts_consumed: 0,
            output_bytes: 0,
            event_count: 0,
            next_event_sequence: 0,
            next_request_sequence: 0,
            nodes: node_states,
            consumed_completion_digests: BTreeMap::new(),
        };
        ensure_snapshot_bound(&snapshot)?;
        Ok(Self {
            plan,
            policy,
            predecessors,
            nodes,
            snapshot,
        })
    }

    pub fn resume(
        plan: ExecutionPlan,
        policy: EnginePolicy,
        limits: EngineLimits,
        checkpoint: &[u8],
    ) -> Result<Self, EngineError> {
        if count_u64(checkpoint.len(), "checkpoint")? > limits.max_checkpoint_bytes {
            return Err(EngineError::new(
                EngineErrorCode::ResourceLimit,
                "checkpoint",
                "checkpoint byte limit exceeded",
            ));
        }
        let expected = Self::new(plan, policy, limits)?;
        let snapshot: EngineSnapshot = serde_json::from_slice(checkpoint).map_err(|error| {
            EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint",
                &error.to_string(),
            )
        })?;
        let canonical = encode(&snapshot, "checkpoint")?;
        if canonical != checkpoint {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint",
                "checkpoint encoding is not canonical",
            ));
        }
        validate_resumed_snapshot(&snapshot, &expected.snapshot)?;
        let mut resumed = expected;
        resumed.snapshot = snapshot;
        Ok(resumed)
    }

    pub fn snapshot(&self) -> &EngineSnapshot {
        &self.snapshot
    }

    pub fn is_quiescent(&self) -> bool {
        self.snapshot
            .nodes
            .values()
            .all(|node| !node.state.is_in_flight())
    }

    pub fn is_terminal(&self) -> bool {
        self.snapshot
            .nodes
            .values()
            .all(|node| node.state.is_terminal())
    }

    pub fn checkpoint(&self) -> Result<Vec<u8>, EngineError> {
        if !self.is_quiescent() {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointNotQuiescent,
                "checkpoint",
                "checkpoint requires a quiescent engine",
            ));
        }
        let bytes = encode(&self.snapshot, "checkpoint")?;
        if count_u64(bytes.len(), "checkpoint")? > self.snapshot.limits.max_checkpoint_bytes {
            return Err(EngineError::new(
                EngineErrorCode::ResourceLimit,
                "checkpoint",
                "checkpoint byte limit exceeded",
            ));
        }
        Ok(bytes)
    }

    pub fn turn(&mut self, mut input: TurnInput) -> Result<TurnOutput, EngineError> {
        if input.logical_tick <= self.snapshot.logical_tick {
            return Err(EngineError::new(
                EngineErrorCode::Protocol,
                "turn.logical_tick",
                "logical tick must increase",
            ));
        }
        if self.snapshot.logical_turns >= self.snapshot.limits.max_logical_turns {
            return Err(EngineError::new(
                EngineErrorCode::ResourceLimit,
                "turn",
                "logical turn limit exhausted",
            ));
        }

        let mut working = self.snapshot.clone();
        working.logical_tick = input.logical_tick;
        working.logical_turns += 1;
        let mut effects = Vec::new();
        let mut events = Vec::new();
        let mut completed_nodes = BTreeSet::new();

        input
            .completions
            .sort_by(|left, right| left.request_id().cmp(right.request_id()));
        for completion in &input.completions {
            if self.apply_completion(&mut working, completion, &mut events, &mut completed_nodes)? {
                completed_nodes.insert(String::from(completion.identity().0));
            }
        }

        input.cancellations.sort();
        input.cancellations.dedup();
        for node_id in &input.cancellations {
            if completed_nodes.contains(node_id) {
                continue;
            }
            self.apply_cancellation(&mut working, node_id, &mut effects, &mut events)?;
        }

        self.promote_ready(&mut working, &mut events)?;
        self.dispatch(&mut working, &mut effects, &mut events)?;
        ensure_snapshot_bound(&working)?;

        self.snapshot = working.clone();
        Ok(TurnOutput {
            snapshot: working,
            effects,
            events,
        })
    }

    fn apply_completion(
        &self,
        snapshot: &mut EngineSnapshot,
        completion: &PortCompletion,
        events: &mut Vec<EngineEvent>,
        completed_nodes: &mut BTreeSet<String>,
    ) -> Result<bool, EngineError> {
        let encoded = encode(completion, "completion")?;
        let digest = hash_parts(COMPLETION_DIGEST_DOMAIN, &[&encoded]);
        let request_id = completion.request_id();
        if let Some(previous) = snapshot.consumed_completion_digests.get(request_id) {
            if previous == &digest {
                return Ok(false);
            }
            return Err(EngineError::new(
                EngineErrorCode::Protocol,
                "completion.request_id",
                "non-identical duplicate completion",
            ));
        }

        let mut active_node = None;
        for (node_id, node) in &snapshot.nodes {
            let active = match &node.state {
                NodeState::Dispatched {
                    request_id: active, ..
                }
                | NodeState::Cancelling {
                    request_id: active, ..
                } => active == request_id,
                NodeState::Pending
                | NodeState::Ready
                | NodeState::RetryWait { .. }
                | NodeState::Succeeded { .. }
                | NodeState::Failed { .. }
                | NodeState::Cancelled => false,
            };
            if active {
                active_node = Some(node_id.clone());
                break;
            }
        }
        let node_id = active_node.ok_or_else(|| {
            EngineError::new(
                EngineErrorCode::Protocol,
                "completion.request_id",
                "unknown completion request identity",
            )
        })?;
        let (declared_node, declared_attempt) = completion.identity();
        if declared_node != node_id {
            return Err(EngineError::new(
                EngineErrorCode::Protocol,
                "completion.node_id",
                "completion node identity mismatch",
            ));
        }

        let current = snapshot.nodes.get(&node_id).ok_or_else(|| {
            EngineError::new(
                EngineErrorCode::InvalidPlan,
                "snapshot.nodes",
                "active node is absent",
            )
        })?;
        let (active_attempt, cancelling) = match &current.state {
            NodeState::Dispatched { attempt, .. } => (*attempt, false),
            NodeState::Cancelling { attempt, .. } => (*attempt, true),
            NodeState::Pending
            | NodeState::Ready
            | NodeState::RetryWait { .. }
            | NodeState::Succeeded { .. }
            | NodeState::Failed { .. }
            | NodeState::Cancelled => {
                return Err(EngineError::new(
                    EngineErrorCode::Protocol,
                    "completion",
                    "completion targets a non-active node",
                ));
            }
        };
        if declared_attempt != active_attempt || current.attempts != active_attempt {
            return Err(EngineError::new(
                EngineErrorCode::Protocol,
                "completion.attempt",
                "completion attempt mismatch",
            ));
        }

        let node_policy = self.policy.nodes.get(&node_id).ok_or_else(|| {
            EngineError::new(
                EngineErrorCode::InvalidPolicy,
                "policy.nodes",
                "node policy is absent",
            )
        })?;
        let outcome = match completion {
            PortCompletion::Provider(value) => {
                if node_policy.port != PortKind::Provider {
                    return Err(EngineError::new(
                        EngineErrorCode::Protocol,
                        "completion",
                        "provider completion targets a tool request",
                    ));
                }
                Some(value.outcome.clone())
            }
            PortCompletion::Tool(value) => {
                match &node_policy.port {
                    PortKind::Tool { .. } => {}
                    PortKind::Provider => {
                        return Err(EngineError::new(
                            EngineErrorCode::Protocol,
                            "completion",
                            "tool completion targets a provider request",
                        ));
                    }
                }
                Some(value.outcome.clone())
            }
            PortCompletion::Cancel(value) => {
                if !cancelling {
                    return Err(EngineError::new(
                        EngineErrorCode::Protocol,
                        "completion",
                        "cancel acknowledgement targets a dispatched request",
                    ));
                }
                if !value.acknowledged {
                    return Err(EngineError::new(
                        EngineErrorCode::Protocol,
                        "completion.acknowledged",
                        "cancel acknowledgement was rejected",
                    ));
                }
                None
            }
        };

        snapshot
            .consumed_completion_digests
            .insert(String::from(request_id), digest);
        if let Some(value) = outcome {
            self.apply_outcome(snapshot, &node_id, value, events)?;
        } else {
            let node = snapshot.nodes.get_mut(&node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "snapshot.nodes",
                    "cancelled node is absent",
                )
            })?;
            node.state = NodeState::Cancelled;
            emit(
                snapshot,
                events,
                Some(node_id.clone()),
                EventKind::NodeCancelled,
            )?;
        }
        completed_nodes.insert(node_id);
        Ok(true)
    }

    fn apply_outcome(
        &self,
        snapshot: &mut EngineSnapshot,
        node_id: &str,
        outcome: CompletionOutcome,
        events: &mut Vec<EngineEvent>,
    ) -> Result<(), EngineError> {
        match outcome {
            CompletionOutcome::Success(output) => {
                let new_total = snapshot
                    .output_bytes
                    .checked_add(count_u64(output.bytes.len(), "completion.output")?)
                    .ok_or_else(|| {
                        EngineError::new(
                            EngineErrorCode::ResourceLimit,
                            "completion.output",
                            "output byte accounting overflow",
                        )
                    })?;
                if new_total > snapshot.limits.max_output_bytes {
                    return Err(EngineError::new(
                        EngineErrorCode::ResourceLimit,
                        "completion.output",
                        "retained output byte limit exceeded",
                    ));
                }
                snapshot.output_bytes = new_total;
                let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "snapshot.nodes",
                        "completed node is absent",
                    )
                })?;
                node.state = NodeState::Succeeded { output };
                emit(
                    snapshot,
                    events,
                    Some(String::from(node_id)),
                    EventKind::NodeSucceeded,
                )?;
            }
            CompletionOutcome::Failure(failure) => {
                let node_policy = self.policy.nodes.get(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPolicy,
                        "policy.nodes",
                        "node policy is absent",
                    )
                })?;
                let attempts = snapshot
                    .nodes
                    .get(node_id)
                    .ok_or_else(|| {
                        EngineError::new(
                            EngineErrorCode::InvalidPlan,
                            "snapshot.nodes",
                            "failed node is absent",
                        )
                    })?
                    .attempts;
                if node_policy.retry.retryable.contains(&failure.class)
                    && attempts < node_policy.retry.max_attempts
                {
                    let index = usize::try_from(attempts - 1).map_err(|error| {
                        EngineError::new(
                            EngineErrorCode::ResourceLimit,
                            "policy.retry.delay_ticks",
                            &error.to_string(),
                        )
                    })?;
                    let delay = node_policy.retry.delay_ticks[index];
                    let ready_at_tick =
                        snapshot.logical_tick.checked_add(delay).ok_or_else(|| {
                            EngineError::new(
                                EngineErrorCode::ResourceLimit,
                                "policy.retry.delay_ticks",
                                "retry logical tick overflow",
                            )
                        })?;
                    let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
                        EngineError::new(
                            EngineErrorCode::InvalidPlan,
                            "snapshot.nodes",
                            "retry node is absent",
                        )
                    })?;
                    node.state = NodeState::RetryWait { ready_at_tick };
                    emit(
                        snapshot,
                        events,
                        Some(String::from(node_id)),
                        EventKind::RetryScheduled { ready_at_tick },
                    )?;
                } else {
                    let terminal = if node_policy.retry.retryable.contains(&failure.class) {
                        PortFailure::new(FailureClass::RetryExhausted, "retry attempts exhausted")
                    } else {
                        failure
                    };
                    fail_node(snapshot, events, node_id, terminal)?;
                }
            }
        }
        Ok(())
    }

    fn apply_cancellation(
        &self,
        snapshot: &mut EngineSnapshot,
        node_id: &str,
        effects: &mut Vec<EngineEffect>,
        events: &mut Vec<EngineEvent>,
    ) -> Result<(), EngineError> {
        let current = snapshot.nodes.get(node_id).ok_or_else(|| {
            EngineError::new(
                EngineErrorCode::Protocol,
                "turn.cancellations",
                "cancellation targets an unknown node",
            )
        })?;
        match current.state.clone() {
            NodeState::Pending | NodeState::Ready | NodeState::RetryWait { .. } => {
                let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "snapshot.nodes",
                        "cancelled node is absent",
                    )
                })?;
                node.state = NodeState::Cancelled;
                emit(
                    snapshot,
                    events,
                    Some(String::from(node_id)),
                    EventKind::NodeCancelled,
                )?;
            }
            NodeState::Dispatched {
                request_id,
                attempt,
                sequence,
            } => {
                let cancel_key = hash_parts(
                    CANCEL_IDEMPOTENCY_DOMAIN,
                    &[request_id.as_bytes(), node_id.as_bytes()],
                );
                let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "snapshot.nodes",
                        "cancelling node is absent",
                    )
                })?;
                node.state = NodeState::Cancelling {
                    request_id: request_id.clone(),
                    attempt,
                    sequence,
                };
                effects.push(EngineEffect::Cancel(CancelRequest {
                    request_id: request_id.clone(),
                    idempotency_key: cancel_key,
                    node_id: String::from(node_id),
                    attempt,
                }));
                emit(
                    snapshot,
                    events,
                    Some(String::from(node_id)),
                    EventKind::CancellationRequested { request_id },
                )?;
            }
            NodeState::Cancelling { .. }
            | NodeState::Succeeded { .. }
            | NodeState::Failed { .. }
            | NodeState::Cancelled => {}
        }
        Ok(())
    }

    fn promote_ready(
        &self,
        snapshot: &mut EngineSnapshot,
        events: &mut Vec<EngineEvent>,
    ) -> Result<(), EngineError> {
        for node in snapshot.nodes.values_mut() {
            if let NodeState::RetryWait { ready_at_tick } = node.state {
                if ready_at_tick <= snapshot.logical_tick {
                    node.state = NodeState::Pending;
                }
            }
        }

        loop {
            let mut failures = Vec::new();
            for node_id in &snapshot.node_ids {
                let node = snapshot.nodes.get(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "snapshot.nodes",
                        "planned node is absent",
                    )
                })?;
                if node.state != NodeState::Pending {
                    continue;
                }
                let predecessors = self.predecessors.get(node_id).ok_or_else(|| {
                    EngineError::new(
                        EngineErrorCode::InvalidPlan,
                        "plan.edges",
                        "predecessor set is absent",
                    )
                })?;
                if dependency_decision(snapshot, predecessors, &self.policy.nodes[node_id])
                    == DependencyDecision::Fail
                {
                    failures.push(node_id.clone());
                }
            }
            if failures.is_empty() {
                break;
            }
            for node_id in failures {
                fail_node(
                    snapshot,
                    events,
                    &node_id,
                    PortFailure::new(FailureClass::Dependency, "join condition became impossible"),
                )?;
            }
        }

        let mut eligible = Vec::new();
        for node_id in &snapshot.node_ids {
            let node = snapshot.nodes.get(node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "snapshot.nodes",
                    "planned node is absent",
                )
            })?;
            if node.state != NodeState::Pending {
                continue;
            }
            let predecessors = self.predecessors.get(node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.edges",
                    "predecessor set is absent",
                )
            })?;
            if dependency_decision(snapshot, predecessors, &self.policy.nodes[node_id])
                == DependencyDecision::Ready
            {
                eligible.push(node_id.clone());
            }
        }
        eligible.sort();
        let ready_count = snapshot
            .nodes
            .values()
            .filter(|node| node.state == NodeState::Ready)
            .count();
        let ready_limit = limit_usize(snapshot.limits.max_ready_nodes, "limits.max_ready_nodes")?;
        let capacity = ready_limit.saturating_sub(ready_count);
        let promote_count = capacity.min(eligible.len());
        for node_id in eligible.iter().take(promote_count) {
            let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "snapshot.nodes",
                    "ready node is absent",
                )
            })?;
            node.state = NodeState::Ready;
            emit(
                snapshot,
                events,
                Some(node_id.clone()),
                EventKind::NodeReady,
            )?;
        }
        if eligible.len() > promote_count {
            emit(
                snapshot,
                events,
                None,
                EventKind::Backpressure {
                    queued: count_u64(
                        eligible.len() - promote_count,
                        "events.backpressure.queued",
                    )?,
                },
            )?;
        }
        Ok(())
    }

    fn dispatch(
        &self,
        snapshot: &mut EngineSnapshot,
        effects: &mut Vec<EngineEffect>,
        events: &mut Vec<EngineEvent>,
    ) -> Result<(), EngineError> {
        let in_flight = snapshot
            .nodes
            .values()
            .filter(|node| node.state.is_in_flight())
            .count();
        let in_flight_limit = limit_usize(snapshot.limits.max_in_flight, "limits.max_in_flight")?;
        let mut available = in_flight_limit.saturating_sub(in_flight);
        let ready = snapshot
            .nodes
            .iter()
            .filter_map(|(node_id, node)| {
                if node.state == NodeState::Ready {
                    Some(node_id.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if ready.len() > available {
            emit(
                snapshot,
                events,
                None,
                EventKind::Backpressure {
                    queued: count_u64(ready.len() - available, "events.backpressure.queued")?,
                },
            )?;
        }
        for node_id in ready {
            if available == 0 {
                break;
            }
            let attempts = snapshot.nodes[&node_id].attempts;
            if attempts >= snapshot.limits.max_attempts_per_node
                || snapshot.attempts_consumed >= snapshot.limits.max_total_attempts
            {
                fail_node(
                    snapshot,
                    events,
                    &node_id,
                    PortFailure::new(FailureClass::RetryExhausted, "attempt budget exhausted"),
                )?;
                continue;
            }
            let attempt = attempts + 1;
            let sequence = snapshot.next_request_sequence;
            let node = self.nodes.get(&node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.nodes",
                    "dispatch node is absent",
                )
            })?;
            let node_policy = self.policy.nodes.get(&node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPolicy,
                    "policy.nodes",
                    "dispatch policy is absent",
                )
            })?;
            let effect = make_effect(
                &self.plan,
                node,
                node_policy,
                &snapshot.plan_digest,
                attempt,
                sequence,
            );
            let effect_bytes = encode(&effect, "effect")?;
            if count_u64(effect_bytes.len(), "effect")? > snapshot.limits.max_request_bytes {
                return Err(EngineError::new(
                    EngineErrorCode::ResourceLimit,
                    "effect",
                    "request byte limit exceeded",
                ));
            }
            let request_id = effect_request_id(&effect);
            let state = snapshot.nodes.get_mut(&node_id).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "snapshot.nodes",
                    "dispatch state is absent",
                )
            })?;
            state.attempts = attempt;
            state.state = NodeState::Dispatched {
                request_id: request_id.clone(),
                attempt,
                sequence,
            };
            snapshot.attempts_consumed += 1;
            snapshot.next_request_sequence += 1;
            effects.push(effect);
            emit(
                snapshot,
                events,
                Some(node_id),
                EventKind::RequestDispatched {
                    request_id,
                    attempt,
                },
            )?;
            available -= 1;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DependencyDecision {
    Wait,
    Ready,
    Fail,
}

fn dependency_decision(
    snapshot: &EngineSnapshot,
    predecessors: &[String],
    policy: &crate::model::NodePolicy,
) -> DependencyDecision {
    if predecessors.is_empty() {
        return DependencyDecision::Ready;
    }
    let mut succeeded = 0;
    let mut terminal = 0;
    let mut failed = 0;
    for predecessor in predecessors {
        let state = &snapshot.nodes[predecessor].state;
        match state {
            NodeState::Succeeded { .. } => {
                succeeded += 1;
                terminal += 1;
            }
            NodeState::Failed { .. } | NodeState::Cancelled => {
                failed += 1;
                terminal += 1;
            }
            NodeState::Pending
            | NodeState::Ready
            | NodeState::Dispatched { .. }
            | NodeState::RetryWait { .. }
            | NodeState::Cancelling { .. } => {}
        }
    }
    match policy.join {
        JoinPolicy::All => {
            if succeeded == predecessors.len() {
                DependencyDecision::Ready
            } else if terminal == predecessors.len() {
                DependencyDecision::Fail
            } else {
                DependencyDecision::Wait
            }
        }
        JoinPolicy::FailFast => {
            if failed > 0 {
                DependencyDecision::Fail
            } else if succeeded == predecessors.len() {
                DependencyDecision::Ready
            } else {
                DependencyDecision::Wait
            }
        }
        JoinPolicy::AtLeast { required } => {
            if succeeded >= required {
                DependencyDecision::Ready
            } else if succeeded + (predecessors.len() - terminal) < required {
                DependencyDecision::Fail
            } else {
                DependencyDecision::Wait
            }
        }
    }
}

fn validate_limits(limits: &EngineLimits) -> Result<(), EngineError> {
    if limits.max_plan_nodes == 0
        || limits.max_dependency_edges == 0
        || limits.max_ready_nodes == 0
        || limits.max_in_flight == 0
        || limits.max_total_attempts == 0
        || limits.max_attempts_per_node == 0
        || limits.max_request_bytes == 0
        || limits.max_output_bytes == 0
        || limits.max_events == 0
        || limits.max_checkpoint_bytes == 0
        || limits.max_logical_turns == 0
    {
        return Err(EngineError::new(
            EngineErrorCode::InvalidLimits,
            "limits",
            "all engine limits must be nonzero",
        ));
    }
    if limits.max_in_flight > limits.max_ready_nodes
        || limits.max_ready_nodes > limits.max_plan_nodes
        || u64::from(limits.max_attempts_per_node) > limits.max_total_attempts
    {
        return Err(EngineError::new(
            EngineErrorCode::InvalidLimits,
            "limits",
            "engine limits are contradictory",
        ));
    }
    limit_usize(limits.max_plan_nodes, "limits.max_plan_nodes")?;
    limit_usize(limits.max_dependency_edges, "limits.max_dependency_edges")?;
    limit_usize(limits.max_ready_nodes, "limits.max_ready_nodes")?;
    limit_usize(limits.max_in_flight, "limits.max_in_flight")?;
    limit_usize(limits.max_request_bytes, "limits.max_request_bytes")?;
    limit_usize(limits.max_output_bytes, "limits.max_output_bytes")?;
    limit_usize(limits.max_checkpoint_bytes, "limits.max_checkpoint_bytes")?;
    Ok(())
}

type PlanIndex = (
    BTreeMap<String, PlanNode>,
    BTreeMap<String, Vec<String>>,
    Vec<String>,
    Vec<String>,
);

fn validate_plan(plan: &ExecutionPlan, limits: &EngineLimits) -> Result<PlanIndex, EngineError> {
    if plan.contract != EXECUTION_PLAN_VERSION {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPlan,
            "plan.contract",
            "execution plan contract version mismatch",
        ));
    }
    if !is_hex_digest(&plan.source_digest) {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPlan,
            "plan.source_digest",
            "plan source digest is not canonical SHA-256 hex",
        ));
    }
    if plan.nodes.is_empty() || count_u64(plan.nodes.len(), "plan.nodes")? > limits.max_plan_nodes {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPlan,
            "plan.nodes",
            "plan node admission limit violated",
        ));
    }
    if count_u64(plan.edges.len(), "plan.edges")? > limits.max_dependency_edges {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPlan,
            "plan.edges",
            "plan edge admission limit violated",
        ));
    }
    if u64::try_from(plan.nodes.len()).map_err(|error| {
        EngineError::new(
            EngineErrorCode::InvalidLimits,
            "limits.max_total_attempts",
            &error.to_string(),
        )
    })? > limits.max_total_attempts
    {
        return Err(EngineError::new(
            EngineErrorCode::InvalidLimits,
            "limits.max_total_attempts",
            "total attempt limit cannot admit every plan node",
        ));
    }

    let mut nodes = BTreeMap::new();
    for node in &plan.nodes {
        if node.id.is_empty() || nodes.insert(node.id.clone(), node.clone()).is_some() {
            return Err(EngineError::new(
                EngineErrorCode::InvalidPlan,
                "plan.nodes",
                "plan node identities must be nonempty and unique",
            ));
        }
    }
    let node_ids = nodes.keys().cloned().collect::<Vec<_>>();
    let mut predecessor_sets = node_ids
        .iter()
        .map(|node_id| (node_id.clone(), BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    let mut successor_sets = predecessor_sets.clone();
    let mut edge_encodings = BTreeSet::new();
    let mut edge_ids = Vec::new();
    for edge in &plan.edges {
        if edge.from == edge.to || !nodes.contains_key(&edge.from) || !nodes.contains_key(&edge.to)
        {
            return Err(EngineError::new(
                EngineErrorCode::InvalidPlan,
                "plan.edges",
                "plan edge has an unknown or self-referential endpoint",
            ));
        }
        let encoded = encode(edge, "plan.edges")?;
        if !edge_encodings.insert(encoded.clone()) {
            return Err(EngineError::new(
                EngineErrorCode::InvalidPlan,
                "plan.edges",
                "duplicate plan edge",
            ));
        }
        edge_ids.push(hash_parts(EDGE_DIGEST_DOMAIN, &[&encoded]));
        predecessor_sets
            .get_mut(&edge.to)
            .ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.edges",
                    "edge target is absent",
                )
            })?
            .insert(edge.from.clone());
        successor_sets
            .get_mut(&edge.from)
            .ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.edges",
                    "edge source is absent",
                )
            })?
            .insert(edge.to.clone());
    }
    edge_ids.sort();
    validate_acyclic(&node_ids, &predecessor_sets, &successor_sets)?;
    let predecessors = predecessor_sets
        .into_iter()
        .map(|(node_id, values)| (node_id, values.into_iter().collect()))
        .collect();
    Ok((nodes, predecessors, node_ids, edge_ids))
}

fn validate_acyclic(
    node_ids: &[String],
    predecessors: &BTreeMap<String, BTreeSet<String>>,
    successors: &BTreeMap<String, BTreeSet<String>>,
) -> Result<(), EngineError> {
    let mut indegree = predecessors
        .iter()
        .map(|(node_id, values)| (node_id.clone(), values.len()))
        .collect::<BTreeMap<_, _>>();
    let mut ready = node_ids
        .iter()
        .filter(|node_id| indegree[*node_id] == 0)
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut visited = 0;
    while let Some(node_id) = ready.pop_first() {
        visited += 1;
        for successor in &successors[&node_id] {
            let count = indegree.get_mut(successor).ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::InvalidPlan,
                    "plan.edges",
                    "successor is absent",
                )
            })?;
            *count -= 1;
            if *count == 0 {
                ready.insert(successor.clone());
            }
        }
    }
    if visited != node_ids.len() {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPlan,
            "plan.edges",
            "execution plan contains a dependency cycle",
        ));
    }
    Ok(())
}

fn validate_policy(
    plan: &ExecutionPlan,
    policy: &EnginePolicy,
    predecessors: &BTreeMap<String, Vec<String>>,
    limits: &EngineLimits,
) -> Result<(), EngineError> {
    let plan_ids = plan
        .nodes
        .iter()
        .map(|node| node.id.clone())
        .collect::<BTreeSet<_>>();
    let policy_ids = policy.nodes.keys().cloned().collect::<BTreeSet<_>>();
    if plan_ids != policy_ids {
        return Err(EngineError::new(
            EngineErrorCode::InvalidPolicy,
            "policy.nodes",
            "policy node identities must exactly match the plan",
        ));
    }
    for node in &plan.nodes {
        let node_policy = &policy.nodes[&node.id];
        if node_policy.timeout_ticks == 0
            || node_policy.retry.max_attempts == 0
            || node_policy.retry.max_attempts > limits.max_attempts_per_node
        {
            return Err(EngineError::new(
                EngineErrorCode::InvalidPolicy,
                "policy.nodes",
                "node retry or timeout bounds are invalid",
            ));
        }
        let expected_delays =
            usize::try_from(node_policy.retry.max_attempts - 1).map_err(|error| {
                EngineError::new(
                    EngineErrorCode::InvalidPolicy,
                    "policy.nodes.retry.delay_ticks",
                    &error.to_string(),
                )
            })?;
        if node_policy.retry.delay_ticks.len() != expected_delays
            || node_policy.retry.delay_ticks.contains(&0)
        {
            return Err(EngineError::new(
                EngineErrorCode::InvalidPolicy,
                "policy.nodes.retry.delay_ticks",
                "retry delay schedule must be positive and exact",
            ));
        }
        match &node_policy.port {
            PortKind::Provider => {
                if node.provider_ref.is_empty() {
                    return Err(EngineError::new(
                        EngineErrorCode::InvalidPolicy,
                        "policy.nodes.port",
                        "provider request has no provider identity",
                    ));
                }
            }
            PortKind::Tool { name } => {
                if name.is_empty() || !node.tools.contains(name) {
                    return Err(EngineError::new(
                        EngineErrorCode::InvalidPolicy,
                        "policy.nodes.port",
                        "tool policy is not allowed by the plan node",
                    ));
                }
            }
        }
        let incoming = predecessors[&node.id].len();
        if let JoinPolicy::AtLeast { required } = node_policy.join {
            if required == 0 || required > incoming {
                return Err(EngineError::new(
                    EngineErrorCode::InvalidPolicy,
                    "policy.nodes.join",
                    "at-least join threshold is impossible",
                ));
            }
        }
    }
    Ok(())
}

fn validate_request_envelopes(
    plan: &ExecutionPlan,
    policy: &EnginePolicy,
    nodes: &BTreeMap<String, PlanNode>,
    plan_digest: &str,
    limits: &EngineLimits,
) -> Result<(), EngineError> {
    for (node_id, node) in nodes {
        let node_policy = &policy.nodes[node_id];
        let effect = make_effect(
            plan,
            node,
            node_policy,
            plan_digest,
            node_policy.retry.max_attempts,
            limits.max_total_attempts,
        );
        if count_u64(encode(&effect, "effect")?.len(), "effect")? > limits.max_request_bytes {
            return Err(EngineError::new(
                EngineErrorCode::InvalidLimits,
                "limits.max_request_bytes",
                "request byte limit cannot admit a plan node",
            ));
        }
    }
    Ok(())
}

fn make_effect(
    plan: &ExecutionPlan,
    node: &PlanNode,
    policy: &crate::model::NodePolicy,
    plan_digest: &str,
    attempt: u32,
    sequence: u64,
) -> EngineEffect {
    let attempt_bytes = attempt.to_be_bytes();
    let sequence_bytes = sequence.to_be_bytes();
    let request_id = hash_parts(
        REQUEST_ID_DOMAIN,
        &[
            plan_digest.as_bytes(),
            node.id.as_bytes(),
            &attempt_bytes,
            &sequence_bytes,
        ],
    );
    let idempotency_key = hash_parts(
        IDEMPOTENCY_DOMAIN,
        &[request_id.as_bytes(), plan.source_digest.as_bytes()],
    );
    match &policy.port {
        PortKind::Provider => EngineEffect::Provider(Box::new(ProviderRequest {
            request_id,
            idempotency_key,
            sequence,
            node_id: node.id.clone(),
            attempt,
            provider_ref: node.provider_ref.clone(),
            model: node.model.clone(),
            prompt: node.prompt.clone(),
            inputs: node.inputs.clone(),
            timeout_ticks: policy.timeout_ticks,
        })),
        PortKind::Tool { name } => EngineEffect::Tool(Box::new(ToolRequest {
            request_id,
            idempotency_key,
            sequence,
            node_id: node.id.clone(),
            attempt,
            tool: name.clone(),
            run: plan.run.clone(),
            inputs: node.inputs.clone(),
            timeout_ticks: policy.timeout_ticks,
        })),
    }
}

fn effect_request_id(effect: &EngineEffect) -> String {
    match effect {
        EngineEffect::Provider(request) => request.request_id.clone(),
        EngineEffect::Tool(request) => request.request_id.clone(),
        EngineEffect::Cancel(request) => request.request_id.clone(),
    }
}

fn fail_node(
    snapshot: &mut EngineSnapshot,
    events: &mut Vec<EngineEvent>,
    node_id: &str,
    failure: PortFailure,
) -> Result<(), EngineError> {
    let node = snapshot.nodes.get_mut(node_id).ok_or_else(|| {
        EngineError::new(
            EngineErrorCode::InvalidPlan,
            "snapshot.nodes",
            "failed node is absent",
        )
    })?;
    node.state = NodeState::Failed {
        failure: failure.clone(),
    };
    emit(
        snapshot,
        events,
        Some(String::from(node_id)),
        EventKind::NodeFailed { failure },
    )
}

fn emit(
    snapshot: &mut EngineSnapshot,
    events: &mut Vec<EngineEvent>,
    node_id: Option<String>,
    kind: EventKind,
) -> Result<(), EngineError> {
    if snapshot.event_count >= snapshot.limits.max_events {
        return Err(EngineError::new(
            EngineErrorCode::ResourceLimit,
            "events",
            "event budget exhausted",
        ));
    }
    let sequence = snapshot.next_event_sequence;
    snapshot.event_count += 1;
    snapshot.next_event_sequence += 1;
    events.push(EngineEvent {
        sequence,
        node_id,
        kind,
    });
    Ok(())
}

fn ensure_snapshot_bound(snapshot: &EngineSnapshot) -> Result<(), EngineError> {
    if count_u64(encode(snapshot, "checkpoint")?.len(), "checkpoint")?
        > snapshot.limits.max_checkpoint_bytes
    {
        return Err(EngineError::new(
            EngineErrorCode::ResourceLimit,
            "checkpoint",
            "checkpoint byte limit exceeded",
        ));
    }
    Ok(())
}

fn validate_resumed_snapshot(
    snapshot: &EngineSnapshot,
    expected: &EngineSnapshot,
) -> Result<(), EngineError> {
    if snapshot.checkpoint_contract != expected.checkpoint_contract
        || snapshot.engine_contract != expected.engine_contract
        || snapshot.plan_contract != expected.plan_contract
        || snapshot.plan_source_digest != expected.plan_source_digest
        || snapshot.plan_digest != expected.plan_digest
        || snapshot.policy_digest != expected.policy_digest
        || snapshot.node_ids != expected.node_ids
        || snapshot.edge_ids != expected.edge_ids
        || snapshot.limits != expected.limits
    {
        return Err(EngineError::new(
            EngineErrorCode::CheckpointIncompatible,
            "checkpoint",
            "checkpoint plan, policy, limits, or contract mismatch",
        ));
    }
    let observed_ids = snapshot.nodes.keys().cloned().collect::<Vec<_>>();
    if observed_ids != snapshot.node_ids
        || snapshot.logical_turns > snapshot.limits.max_logical_turns
        || snapshot.event_count > snapshot.limits.max_events
        || snapshot.event_count != snapshot.next_event_sequence
        || snapshot.attempts_consumed > snapshot.limits.max_total_attempts
        || snapshot.attempts_consumed != snapshot.next_request_sequence
        || snapshot.output_bytes > snapshot.limits.max_output_bytes
    {
        return Err(EngineError::new(
            EngineErrorCode::CheckpointIncompatible,
            "checkpoint",
            "checkpoint counters or identity set are invalid",
        ));
    }
    let mut attempts = 0_u64;
    let mut output_bytes = 0_u64;
    for node in snapshot.nodes.values() {
        if node.attempts > snapshot.limits.max_attempts_per_node {
            return Err(EngineError::new(
                EngineErrorCode::CheckpointIncompatible,
                "checkpoint.nodes",
                "checkpoint node attempt counter exceeds its limit",
            ));
        }
        attempts = attempts
            .checked_add(u64::from(node.attempts))
            .ok_or_else(|| {
                EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "checkpoint attempt counter overflow",
                )
            })?;
        match &node.state {
            NodeState::Dispatched { .. } | NodeState::Cancelling { .. } => {
                return Err(EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "checkpoint contains an in-flight request",
                ));
            }
            NodeState::RetryWait { .. } if node.attempts == 0 => {
                return Err(EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "retry wait has no consumed attempt",
                ));
            }
            NodeState::Succeeded { .. } if node.attempts == 0 => {
                return Err(EngineError::new(
                    EngineErrorCode::CheckpointIncompatible,
                    "checkpoint.nodes",
                    "successful node has no consumed attempt",
                ));
            }
            NodeState::Succeeded { output } => {
                output_bytes = output_bytes
                    .checked_add(count_u64(output.bytes.len(), "checkpoint.nodes.output")?)
                    .ok_or_else(|| {
                        EngineError::new(
                            EngineErrorCode::CheckpointIncompatible,
                            "checkpoint.nodes",
                            "checkpoint output counter overflow",
                        )
                    })?;
            }
            NodeState::Pending
            | NodeState::Ready
            | NodeState::RetryWait { .. }
            | NodeState::Failed { .. }
            | NodeState::Cancelled => {}
        }
    }
    if attempts != snapshot.attempts_consumed || output_bytes != snapshot.output_bytes {
        return Err(EngineError::new(
            EngineErrorCode::CheckpointIncompatible,
            "checkpoint",
            "checkpoint accounting does not match node state",
        ));
    }
    if count_u64(
        snapshot.consumed_completion_digests.len(),
        "checkpoint.completions",
    )? > snapshot.attempts_consumed
        || snapshot
            .consumed_completion_digests
            .iter()
            .any(|(request_id, digest)| !is_hex_digest(request_id) || !is_hex_digest(digest))
    {
        return Err(EngineError::new(
            EngineErrorCode::CheckpointIncompatible,
            "checkpoint.completions",
            "checkpoint completion digest set is invalid",
        ));
    }
    ensure_snapshot_bound(snapshot)
}

fn count_u64(value: usize, path: &str) -> Result<u64, EngineError> {
    u64::try_from(value)
        .map_err(|error| EngineError::new(EngineErrorCode::ResourceLimit, path, &error.to_string()))
}

fn limit_usize(value: u64, path: &str) -> Result<usize, EngineError> {
    usize::try_from(value)
        .map_err(|error| EngineError::new(EngineErrorCode::InvalidLimits, path, &error.to_string()))
}

fn encode<T: Serialize>(value: &T, path: &str) -> Result<Vec<u8>, EngineError> {
    serde_json::to_vec(value)
        .map_err(|error| EngineError::new(EngineErrorCode::Serialization, path, &error.to_string()))
}

fn hash_parts(domain: &[u8], parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    for part in parts {
        let length = u64::try_from(part.len()).unwrap_or(u64::MAX).to_be_bytes();
        hasher.update(length);
        hasher.update(part);
    }
    hex::encode(hasher.finalize())
}

fn is_hex_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
