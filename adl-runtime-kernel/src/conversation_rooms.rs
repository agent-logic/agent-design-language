//! Runtime-owned governed multi-agent room routing.
//!
//! This module is intentionally small and policy-facing: it defines the room
//! turn contract that downstream Observatory code can render without allowing
//! the browser to expand recipients, invent participants, or blur partial
//! delivery into success.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::layer8_authority::{AuthorityScope, Layer8Action};

pub const GOVERNED_ROOM_TURN_SCHEMA: &str = "adl.runtime.governed_room_turn.v1";
pub const GOVERNED_ROOM_MENTION_SCHEMA: &str = "adl.runtime.governed_room_mention.v1";
pub const GOVERNED_ROOM_ROUTE_SCHEMA: &str = "adl.runtime.governed_room_route.v1";
pub const GOVERNED_ROOM_MAX_RECIPIENTS: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedRoomParticipantState {
    Joined,
    Left,
    Revoked,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedRoomParticipant {
    pub participant_id: String,
    pub polis_id: String,
    pub display_name: String,
    pub policy_eligible: bool,
    pub state: GovernedRoomParticipantState,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedRoom {
    pub room_id: String,
    pub polis_id: String,
    pub epoch: u64,
    pub next_turn_sequence: u64,
    pub seen_turn_ids: BTreeSet<String>,
    pub closed: bool,
    pub participants: Vec<GovernedRoomParticipant>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedRoomTurnIntent {
    pub schema: String,
    pub room_id: String,
    pub turn_id: String,
    pub turn_sequence: u64,
    pub sender_id: String,
    pub correlation_id: String,
    pub addressed_recipients: Vec<String>,
    pub message: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedRoomMention {
    pub schema: &'static str,
    pub room_id: String,
    pub turn_id: String,
    pub recipient_id: String,
    pub display_name: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedRoomDeliveryState {
    Accepted,
    Delivered,
    Refused,
    TimedOut,
    Unavailable,
    Revoked,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedRoomRecipientDelivery {
    pub recipient_id: String,
    pub state: GovernedRoomDeliveryState,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedRoomRoute {
    pub schema: &'static str,
    pub status: &'static str,
    pub room_id: String,
    pub turn_id: String,
    pub turn_sequence: u64,
    pub sender_id: String,
    pub correlation_id: String,
    pub room_epoch: u64,
    pub addressed_recipients: Vec<String>,
    pub mentions: Vec<GovernedRoomMention>,
    pub deliveries: Vec<GovernedRoomRecipientDelivery>,
    pub error: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GovernedRoomRoutingError {
    InvalidTurn,
    RoomClosed,
    DuplicateRecipient,
    DuplicateTurn,
    ReorderedTurn,
    SequenceExhausted,
    ImplicitBroadcastDenied,
    UnknownRecipient,
    IneligibleRecipient,
    UnavailableRecipient,
    CrossPolisDenied,
}

impl GovernedRoomRoutingError {
    pub fn code(self) -> &'static str {
        match self {
            GovernedRoomRoutingError::InvalidTurn => "invalid_room_turn",
            GovernedRoomRoutingError::RoomClosed => "room_closed",
            GovernedRoomRoutingError::DuplicateRecipient => "duplicate_room_recipient",
            GovernedRoomRoutingError::DuplicateTurn => "duplicate_room_turn",
            GovernedRoomRoutingError::ReorderedTurn => "reordered_room_turn",
            GovernedRoomRoutingError::SequenceExhausted => "room_turn_sequence_exhausted",
            GovernedRoomRoutingError::ImplicitBroadcastDenied => "implicit_broadcast_denied",
            GovernedRoomRoutingError::UnknownRecipient => "unknown_room_recipient",
            GovernedRoomRoutingError::IneligibleRecipient => "ineligible_room_recipient",
            GovernedRoomRoutingError::UnavailableRecipient => "unavailable_room_recipient",
            GovernedRoomRoutingError::CrossPolisDenied => "cross_polis_denied",
        }
    }
}

impl GovernedRoom {
    pub fn plan_turn(
        &mut self,
        intent: &GovernedRoomTurnIntent,
    ) -> Result<GovernedRoomRoute, GovernedRoomRoutingError> {
        if intent.schema != GOVERNED_ROOM_TURN_SCHEMA
            || !safe_identifier(&self.room_id)
            || !safe_identifier(&self.polis_id)
            || intent.room_id != self.room_id
            || !safe_identifier(&intent.turn_id)
            || !safe_identifier(&intent.sender_id)
            || !safe_correlation(&intent.correlation_id)
            || intent.message.trim().is_empty()
            || intent.message.len() > 4_096
        {
            return Err(GovernedRoomRoutingError::InvalidTurn);
        }
        if self.closed {
            return Err(GovernedRoomRoutingError::RoomClosed);
        }
        if self.seen_turn_ids.contains(&intent.turn_id) {
            return Err(GovernedRoomRoutingError::DuplicateTurn);
        }
        if intent.turn_sequence != self.next_turn_sequence {
            return if intent.turn_sequence < self.next_turn_sequence {
                Err(GovernedRoomRoutingError::DuplicateTurn)
            } else {
                Err(GovernedRoomRoutingError::ReorderedTurn)
            };
        }
        let addressed = explicit_recipient_set(&intent.addressed_recipients)?;
        let participants = self
            .participants
            .iter()
            .map(|participant| (participant.participant_id.as_str(), participant))
            .collect::<BTreeMap<_, _>>();
        let mut mentions = Vec::new();
        for recipient_id in &addressed {
            let Some(participant) = participants.get(recipient_id.as_str()) else {
                return Err(GovernedRoomRoutingError::UnknownRecipient);
            };
            if participant.polis_id != self.polis_id {
                return Err(GovernedRoomRoutingError::CrossPolisDenied);
            }
            if !participant.policy_eligible {
                return Err(GovernedRoomRoutingError::IneligibleRecipient);
            }
            if participant.state != GovernedRoomParticipantState::Joined {
                return Err(GovernedRoomRoutingError::UnavailableRecipient);
            }
            mentions.push(GovernedRoomMention {
                schema: GOVERNED_ROOM_MENTION_SCHEMA,
                room_id: self.room_id.clone(),
                turn_id: intent.turn_id.clone(),
                recipient_id: recipient_id.clone(),
                display_name: participant.display_name.clone(),
            });
        }
        self.seen_turn_ids.insert(intent.turn_id.clone());
        self.next_turn_sequence = self
            .next_turn_sequence
            .checked_add(1)
            .ok_or(GovernedRoomRoutingError::SequenceExhausted)?;
        Ok(GovernedRoomRoute {
            schema: GOVERNED_ROOM_ROUTE_SCHEMA,
            status: "accepted",
            room_id: intent.room_id.clone(),
            turn_id: intent.turn_id.clone(),
            turn_sequence: intent.turn_sequence,
            sender_id: intent.sender_id.clone(),
            correlation_id: intent.correlation_id.clone(),
            room_epoch: self.epoch,
            addressed_recipients: addressed.into_iter().collect(),
            mentions,
            deliveries: Vec::new(),
            error: None,
        })
    }
}

impl GovernedRoomRoute {
    pub fn authority_scope(&self, polis_id: impl Into<String>) -> AuthorityScope {
        AuthorityScope {
            polis_id: polis_id.into(),
            action: Layer8Action::AddressRecipients,
            conversation_id: Some(self.room_id.clone()),
            recipients: self.addressed_recipients.iter().cloned().collect(),
            attachment_id: None,
        }
    }

    pub fn with_delivery_states(
        mut self,
        states: BTreeMap<String, GovernedRoomDeliveryState>,
    ) -> Self {
        let addressed = self
            .addressed_recipients
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        self.deliveries = self
            .addressed_recipients
            .iter()
            .map(|recipient_id| {
                let state = states
                    .get(recipient_id)
                    .copied()
                    .unwrap_or(GovernedRoomDeliveryState::Accepted);
                GovernedRoomRecipientDelivery {
                    recipient_id: recipient_id.clone(),
                    state,
                    error: delivery_error(state).map(str::to_owned),
                }
            })
            .collect();
        let delivered = self
            .deliveries
            .iter()
            .filter(|delivery| matches!(delivery.state, GovernedRoomDeliveryState::Delivered))
            .count();
        let accepted = self
            .deliveries
            .iter()
            .filter(|delivery| matches!(delivery.state, GovernedRoomDeliveryState::Accepted))
            .count();
        self.status = if delivered == addressed.len() {
            "delivered"
        } else if accepted == addressed.len() {
            "accepted"
        } else if delivered == 0 {
            "refused"
        } else {
            "partial_delivery"
        };
        self
    }
}

fn explicit_recipient_set(
    recipients: &[String],
) -> Result<BTreeSet<String>, GovernedRoomRoutingError> {
    if recipients.is_empty() || recipients.len() > GOVERNED_ROOM_MAX_RECIPIENTS {
        return Err(GovernedRoomRoutingError::ImplicitBroadcastDenied);
    }
    let mut unique = BTreeSet::new();
    for recipient in recipients {
        if recipient == "*" || recipient.eq_ignore_ascii_case("all") || !safe_identifier(recipient)
        {
            return Err(GovernedRoomRoutingError::ImplicitBroadcastDenied);
        }
        if !unique.insert(recipient.clone()) {
            return Err(GovernedRoomRoutingError::DuplicateRecipient);
        }
    }
    Ok(unique)
}

fn delivery_error(state: GovernedRoomDeliveryState) -> Option<&'static str> {
    match state {
        GovernedRoomDeliveryState::Accepted => None,
        GovernedRoomDeliveryState::Delivered => None,
        GovernedRoomDeliveryState::Refused => Some("recipient_refused_delivery"),
        GovernedRoomDeliveryState::TimedOut => Some("recipient_delivery_timed_out"),
        GovernedRoomDeliveryState::Unavailable => Some("recipient_unavailable"),
        GovernedRoomDeliveryState::Revoked => Some("recipient_revoked"),
    }
}

fn safe_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

fn safe_correlation(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn participant(id: &str) -> GovernedRoomParticipant {
        GovernedRoomParticipant {
            participant_id: id.to_owned(),
            polis_id: "polis-local".to_owned(),
            display_name: id.to_owned(),
            policy_eligible: true,
            state: GovernedRoomParticipantState::Joined,
        }
    }

    fn room() -> GovernedRoom {
        GovernedRoom {
            room_id: "room-1".to_owned(),
            polis_id: "polis-local".to_owned(),
            epoch: 7,
            next_turn_sequence: 1,
            seen_turn_ids: BTreeSet::new(),
            closed: false,
            participants: vec![participant("shepherd"), participant("scribe")],
        }
    }

    fn intent(recipients: Vec<&str>) -> GovernedRoomTurnIntent {
        GovernedRoomTurnIntent {
            schema: GOVERNED_ROOM_TURN_SCHEMA.to_owned(),
            room_id: "room-1".to_owned(),
            turn_id: "turn-1".to_owned(),
            turn_sequence: 1,
            sender_id: "operator".to_owned(),
            correlation_id: "corr:room:1".to_owned(),
            addressed_recipients: recipients.into_iter().map(str::to_owned).collect(),
            message: "hello room".to_owned(),
        }
    }

    #[test]
    fn room_turn_requires_explicit_stable_recipient_set() {
        let route = room()
            .plan_turn(&intent(vec!["scribe", "shepherd"]))
            .expect("explicit participant set accepted");
        assert_eq!(route.status, "accepted");
        assert_eq!(route.room_epoch, 7);
        assert_eq!(route.turn_sequence, 1);
        assert_eq!(route.addressed_recipients, vec!["scribe", "shepherd"]);
        assert_eq!(
            route.mentions,
            vec![
                GovernedRoomMention {
                    schema: GOVERNED_ROOM_MENTION_SCHEMA,
                    room_id: "room-1".to_owned(),
                    turn_id: "turn-1".to_owned(),
                    recipient_id: "scribe".to_owned(),
                    display_name: "scribe".to_owned(),
                },
                GovernedRoomMention {
                    schema: GOVERNED_ROOM_MENTION_SCHEMA,
                    room_id: "room-1".to_owned(),
                    turn_id: "turn-1".to_owned(),
                    recipient_id: "shepherd".to_owned(),
                    display_name: "shepherd".to_owned(),
                },
            ]
        );
        assert_eq!(route.deliveries, Vec::new());
    }

    #[test]
    fn implicit_broadcast_and_duplicate_recipients_fail_closed() {
        assert_eq!(
            room().plan_turn(&intent(vec![])).unwrap_err(),
            GovernedRoomRoutingError::ImplicitBroadcastDenied
        );
        assert_eq!(
            room().plan_turn(&intent(vec!["all"])).unwrap_err(),
            GovernedRoomRoutingError::ImplicitBroadcastDenied
        );
        assert_eq!(
            room()
                .plan_turn(&intent(vec!["shepherd", "shepherd"]))
                .unwrap_err(),
            GovernedRoomRoutingError::DuplicateRecipient
        );
    }

    #[test]
    fn unknown_ineligible_and_cross_polis_recipients_fail_closed() {
        assert_eq!(
            room().plan_turn(&intent(vec!["unknown"])).unwrap_err(),
            GovernedRoomRoutingError::UnknownRecipient
        );
        let mut ineligible = room();
        ineligible.participants[0].policy_eligible = false;
        assert_eq!(
            ineligible.plan_turn(&intent(vec!["shepherd"])).unwrap_err(),
            GovernedRoomRoutingError::IneligibleRecipient
        );
        let mut left = room();
        left.participants[0].state = GovernedRoomParticipantState::Left;
        assert_eq!(
            left.plan_turn(&intent(vec!["shepherd"])).unwrap_err(),
            GovernedRoomRoutingError::UnavailableRecipient
        );
        let mut revoked = room();
        revoked.participants[0].state = GovernedRoomParticipantState::Revoked;
        assert_eq!(
            revoked.plan_turn(&intent(vec!["shepherd"])).unwrap_err(),
            GovernedRoomRoutingError::UnavailableRecipient
        );
        let mut foreign = room();
        foreign.participants[0].polis_id = "polis-remote".to_owned();
        assert_eq!(
            foreign.plan_turn(&intent(vec!["shepherd"])).unwrap_err(),
            GovernedRoomRoutingError::CrossPolisDenied
        );
    }

    #[test]
    fn delivery_states_preserve_partial_failure_without_hiding_recipient_identity() {
        let route = room()
            .plan_turn(&intent(vec!["shepherd", "scribe"]))
            .expect("accepted");
        let accepted_route = route.clone().with_delivery_states(BTreeMap::from([
            ("shepherd".to_owned(), GovernedRoomDeliveryState::Accepted),
            ("scribe".to_owned(), GovernedRoomDeliveryState::Accepted),
        ]));
        assert_eq!(accepted_route.status, "accepted");
        assert!(accepted_route
            .deliveries
            .iter()
            .all(|delivery| delivery.error.is_none()));

        let route = route.with_delivery_states(BTreeMap::from([
            ("shepherd".to_owned(), GovernedRoomDeliveryState::Delivered),
            ("scribe".to_owned(), GovernedRoomDeliveryState::TimedOut),
        ]));
        assert_eq!(route.status, "partial_delivery");
        assert_eq!(route.deliveries.len(), 2);
        assert_eq!(route.deliveries[0].recipient_id, "scribe");
        assert_eq!(
            route.deliveries[0].error.as_deref(),
            Some("recipient_delivery_timed_out")
        );
        assert_eq!(route.deliveries[1].recipient_id, "shepherd");
        assert_eq!(route.deliveries[1].error, None);
    }

    #[test]
    fn room_route_reuses_layer8_address_recipients_authority_scope() {
        let route = room()
            .plan_turn(&intent(vec!["shepherd", "scribe"]))
            .expect("accepted");
        let scope = route.authority_scope("polis-local");
        assert_eq!(scope.action, Layer8Action::AddressRecipients);
        assert_eq!(scope.conversation_id.as_deref(), Some("room-1"));
        assert_eq!(
            scope.recipients,
            BTreeSet::from(["shepherd".to_owned(), "scribe".to_owned()])
        );
        assert_eq!(scope.attachment_id, None);
    }

    #[test]
    fn room_turn_ordering_and_replay_are_deterministic() {
        let mut room = room();
        let first = intent(vec!["shepherd"]);
        room.plan_turn(&first).expect("first turn accepted");
        assert_eq!(room.next_turn_sequence, 2);

        assert_eq!(
            room.plan_turn(&first).unwrap_err(),
            GovernedRoomRoutingError::DuplicateTurn
        );

        let mut future = intent(vec!["shepherd"]);
        future.turn_id = "turn-3".to_owned();
        future.turn_sequence = 3;
        assert_eq!(
            room.plan_turn(&future).unwrap_err(),
            GovernedRoomRoutingError::ReorderedTurn
        );

        let mut second = intent(vec!["shepherd"]);
        second.turn_id = "turn-2".to_owned();
        second.turn_sequence = 2;
        room.plan_turn(&second).expect("second turn accepted");
        assert_eq!(room.next_turn_sequence, 3);
    }
}
