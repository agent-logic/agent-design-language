use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier};
use serde::{Deserialize, Serialize};

use super::{
    CommunicationKeyDescriptor, CommunicationVerifyingDescriptor, CommunicationVerifyingIdentity,
    RefusalReason, ACIP_IDENTITY_MESSAGE_SCHEMA,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityMessageKind {
    Request,
    Acknowledgement,
}

/// One identity-bound contract for operator-to-agent and direct agent-to-agent delivery.
/// Private keys are deliberately absent; callers inject externally held signing authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedIdentityMessage {
    pub schema: String,
    pub message_kind: IdentityMessageKind,
    pub message_id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub polis_id: String,
    pub conversation_id: String,
    pub correlation_id: String,
    pub causation_id: String,
    pub replay_id: String,
    pub monotonic_sequence: u64,
    pub issued_at_epoch_secs: u64,
    pub expires_at_epoch_secs: u64,
    pub payload_json: String,
    pub signing_key_id: String,
    pub credential_generation: u64,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationSigningProfile {
    pub sender: CommunicationKeyDescriptor,
    pub recipients: Vec<CommunicationVerifyingDescriptor>,
}

#[derive(Debug)]
struct CommunicationSigningIdentity {
    descriptor: CommunicationKeyDescriptor,
    signing_key: SigningKey,
}

#[derive(Debug)]
pub struct Layer8SignedExchange {
    sender: CommunicationSigningIdentity,
    recipients: BTreeMap<String, CommunicationVerifyingIdentity>,
    sequence: AtomicU64,
}

pub fn sign_recipient_acknowledgement(
    request: &SignedIdentityMessage,
    descriptor: &CommunicationKeyDescriptor,
    payload_json: String,
    now: u64,
) -> Result<SignedIdentityMessage, RefusalReason> {
    if descriptor.principal_id != request.recipient_id || descriptor.polis_id != request.polis_id {
        return Err(RefusalReason::IdentityUnavailable);
    }
    if now < descriptor.not_before_epoch_secs || now >= descriptor.expires_at_epoch_secs {
        return Err(RefusalReason::IdentityExpired);
    }
    require_canonical_payload(&payload_json)?;
    let encoded = std::fs::read_to_string(&descriptor.private_key_file)
        .map_err(|_| RefusalReason::IdentityUnavailable)?;
    let bytes = hex::decode(encoded.trim()).map_err(|_| RefusalReason::IdentityUnavailable)?;
    let secret: [u8; 32] = bytes
        .try_into()
        .map_err(|_| RefusalReason::IdentityUnavailable)?;
    let signing_key = SigningKey::from_bytes(&secret);
    let mut acknowledgement = SignedIdentityMessage {
        schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_owned(),
        message_kind: IdentityMessageKind::Acknowledgement,
        message_id: format!(
            "{}-ack-{}",
            descriptor.signing_key_id, request.monotonic_sequence
        ),
        sender_id: descriptor.principal_id.clone(),
        recipient_id: request.sender_id.clone(),
        polis_id: descriptor.polis_id.clone(),
        conversation_id: request.conversation_id.clone(),
        correlation_id: request.correlation_id.clone(),
        causation_id: request.message_id.clone(),
        replay_id: request.replay_id.clone(),
        monotonic_sequence: request.monotonic_sequence,
        issued_at_epoch_secs: now,
        expires_at_epoch_secs: now.saturating_add(60),
        payload_json,
        signing_key_id: descriptor.signing_key_id.clone(),
        credential_generation: descriptor.credential_generation,
        signature: String::new(),
    };
    acknowledgement.signature = hex::encode(
        signing_key
            .sign(&acknowledgement.signing_bytes()?)
            .to_bytes(),
    );
    Ok(acknowledgement)
}

impl Layer8SignedExchange {
    pub fn load(profile: ConversationSigningProfile) -> Result<Self, RefusalReason> {
        let load = |descriptor: CommunicationKeyDescriptor| {
            let encoded = std::fs::read_to_string(&descriptor.private_key_file)
                .map_err(|_| RefusalReason::IdentityUnavailable)?;
            let bytes =
                hex::decode(encoded.trim()).map_err(|_| RefusalReason::IdentityUnavailable)?;
            let secret: [u8; 32] = bytes
                .try_into()
                .map_err(|_| RefusalReason::IdentityUnavailable)?;
            Ok(CommunicationSigningIdentity {
                descriptor,
                signing_key: SigningKey::from_bytes(&secret),
            })
        };
        let sender = load(profile.sender)?;
        let mut recipients = BTreeMap::new();
        for descriptor in profile.recipients {
            let bytes = hex::decode(&descriptor.verifying_key_hex)
                .map_err(|_| RefusalReason::IdentityUnavailable)?;
            let key: [u8; 32] = bytes
                .try_into()
                .map_err(|_| RefusalReason::IdentityUnavailable)?;
            let identity = CommunicationVerifyingIdentity {
                principal_id: descriptor.principal_id,
                polis_id: descriptor.polis_id,
                signing_key_id: descriptor.signing_key_id,
                credential_generation: descriptor.credential_generation,
                verifying_key: ed25519_dalek::VerifyingKey::from_bytes(&key)
                    .map_err(|_| RefusalReason::IdentityUnavailable)?,
                revoked: descriptor.revoked,
                not_before_epoch_secs: descriptor.not_before_epoch_secs,
                expires_at_epoch_secs: descriptor.expires_at_epoch_secs,
            };
            if recipients
                .insert(identity.principal_id.clone(), identity)
                .is_some()
            {
                return Err(RefusalReason::InvalidRequest);
            }
        }
        if recipients.is_empty() {
            return Err(RefusalReason::InvalidRequest);
        }
        Ok(Self {
            sender,
            recipients,
            sequence: AtomicU64::new(0),
        })
    }

    pub fn signed_request(
        &self,
        recipient_id: &str,
        conversation_id: &str,
        correlation_id: &str,
        replay_id: &str,
        payload_json: String,
        now: u64,
    ) -> Result<SignedIdentityMessage, RefusalReason> {
        let recipient = self.active_recipient_verifying_identity(recipient_id, now)?;
        if recipient.polis_id != self.sender.descriptor.polis_id {
            return Err(RefusalReason::InvalidRequest);
        }
        self.sign(
            &self.sender,
            IdentityMessageKind::Request,
            recipient_id,
            conversation_id,
            correlation_id,
            "",
            replay_id,
            payload_json,
            now,
        )
    }

    pub fn verify_request_and_acknowledgement(
        &self,
        request: &SignedIdentityMessage,
        acknowledgement: &SignedIdentityMessage,
        now: u64,
    ) -> Result<(), RefusalReason> {
        self.verify_request(request, now)?;
        let recipient = self
            .recipients
            .get(&request.recipient_id)
            .ok_or(RefusalReason::IdentityUnavailable)?;
        verify_recipient_acknowledgement(request, acknowledgement, recipient, now)
    }

    pub fn verify_request(
        &self,
        request: &SignedIdentityMessage,
        now: u64,
    ) -> Result<(), RefusalReason> {
        if request.message_kind != IdentityMessageKind::Request {
            return Err(RefusalReason::InvalidRequest);
        }
        let recipient = self.active_recipient_verifying_identity(&request.recipient_id, now)?;
        if recipient.polis_id != self.sender.descriptor.polis_id {
            return Err(RefusalReason::InvalidRequest);
        }
        verify_signed_identity_message(
            request,
            &self.verifying(&self.sender),
            &request.recipient_id,
            now,
        )
    }

    pub fn sender_verifying_identity(&self) -> CommunicationVerifyingIdentity {
        self.verifying(&self.sender)
    }

    pub fn recipient_verifying_identity(
        &self,
        recipient_id: &str,
    ) -> Result<CommunicationVerifyingIdentity, RefusalReason> {
        self.recipients
            .get(recipient_id)
            .cloned()
            .ok_or(RefusalReason::IdentityUnavailable)
    }

    pub fn active_recipient_verifying_identity(
        &self,
        recipient_id: &str,
        now: u64,
    ) -> Result<CommunicationVerifyingIdentity, RefusalReason> {
        let identity = self.recipient_verifying_identity(recipient_id)?;
        identity.ensure_valid_at(now)?;
        Ok(identity)
    }

    fn verifying(&self, identity: &CommunicationSigningIdentity) -> CommunicationVerifyingIdentity {
        CommunicationVerifyingIdentity {
            principal_id: identity.descriptor.principal_id.clone(),
            polis_id: identity.descriptor.polis_id.clone(),
            signing_key_id: identity.descriptor.signing_key_id.clone(),
            credential_generation: identity.descriptor.credential_generation,
            verifying_key: identity.signing_key.verifying_key(),
            revoked: false,
            not_before_epoch_secs: identity.descriptor.not_before_epoch_secs,
            expires_at_epoch_secs: identity.descriptor.expires_at_epoch_secs,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn sign(
        &self,
        identity: &CommunicationSigningIdentity,
        message_kind: IdentityMessageKind,
        recipient_id: &str,
        conversation_id: &str,
        correlation_id: &str,
        causation_id: &str,
        replay_id: &str,
        payload_json: String,
        now: u64,
    ) -> Result<SignedIdentityMessage, RefusalReason> {
        identity.descriptor_identity_valid_at(now)?;
        require_canonical_payload(&payload_json)?;
        let sequence = self
            .sequence
            .fetch_add(1, Ordering::SeqCst)
            .checked_add(1)
            .ok_or(RefusalReason::InvalidRequest)?;
        let mut message = SignedIdentityMessage {
            schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_owned(),
            message_kind,
            message_id: format!("{}-{sequence}", identity.descriptor.signing_key_id),
            sender_id: identity.descriptor.principal_id.clone(),
            recipient_id: recipient_id.to_owned(),
            polis_id: identity.descriptor.polis_id.clone(),
            conversation_id: conversation_id.to_owned(),
            correlation_id: correlation_id.to_owned(),
            causation_id: causation_id.to_owned(),
            replay_id: replay_id.to_owned(),
            monotonic_sequence: sequence,
            issued_at_epoch_secs: now,
            expires_at_epoch_secs: now.saturating_add(60),
            payload_json,
            signing_key_id: identity.descriptor.signing_key_id.clone(),
            credential_generation: identity.descriptor.credential_generation,
            signature: String::new(),
        };
        message.signature = hex::encode(
            identity
                .signing_key
                .sign(&message.signing_bytes()?)
                .to_bytes(),
        );
        Ok(message)
    }
}

impl SignedIdentityMessage {
    pub fn signing_bytes(&self) -> Result<Vec<u8>, RefusalReason> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        serde_jcs::to_vec(&unsigned).map_err(|_| RefusalReason::InvalidRequest)
    }
}

trait CommunicationSigningDescriptorValidation {
    fn descriptor_identity_valid_at(&self, now_epoch_secs: u64) -> Result<(), RefusalReason>;
}

impl CommunicationSigningDescriptorValidation for CommunicationSigningIdentity {
    fn descriptor_identity_valid_at(&self, now_epoch_secs: u64) -> Result<(), RefusalReason> {
        if now_epoch_secs < self.descriptor.not_before_epoch_secs
            || now_epoch_secs >= self.descriptor.expires_at_epoch_secs
        {
            return Err(RefusalReason::IdentityExpired);
        }
        Ok(())
    }
}

fn require_canonical_payload(payload_json: &str) -> Result<(), RefusalReason> {
    let payload: serde_json::Value =
        serde_json::from_str(payload_json).map_err(|_| RefusalReason::InvalidRequest)?;
    if serde_jcs::to_string(&payload).map_err(|_| RefusalReason::InvalidRequest)? != payload_json {
        return Err(RefusalReason::InvalidRequest);
    }
    Ok(())
}

pub fn verify_signed_identity_message(
    message: &SignedIdentityMessage,
    identity: &CommunicationVerifyingIdentity,
    expected_recipient: &str,
    now_epoch_secs: u64,
) -> Result<(), RefusalReason> {
    if message.schema != ACIP_IDENTITY_MESSAGE_SCHEMA
        || message.message_id.trim().is_empty()
        || message.sender_id.trim().is_empty()
        || message.recipient_id.trim().is_empty()
        || message.polis_id.trim().is_empty()
        || message.conversation_id.trim().is_empty()
        || message.correlation_id.trim().is_empty()
        || message.replay_id.trim().is_empty()
        || message.monotonic_sequence == 0
        || message.credential_generation == 0
        || message.recipient_id != expected_recipient
    {
        return Err(RefusalReason::InvalidRequest);
    }
    identity.ensure_valid_at(now_epoch_secs)?;
    if now_epoch_secs < message.issued_at_epoch_secs
        || now_epoch_secs >= message.expires_at_epoch_secs
    {
        return Err(RefusalReason::IdentityExpired);
    }
    if message.sender_id != identity.principal_id
        || message.polis_id != identity.polis_id
        || message.signing_key_id != identity.signing_key_id
        || message.credential_generation != identity.credential_generation
    {
        return Err(RefusalReason::IdentityUnavailable);
    }
    require_canonical_payload(&message.payload_json)?;
    let signature_bytes =
        hex::decode(&message.signature).map_err(|_| RefusalReason::InvalidRequest)?;
    let signature =
        Signature::from_slice(&signature_bytes).map_err(|_| RefusalReason::InvalidRequest)?;
    identity
        .verifying_key
        .verify(&message.signing_bytes()?, &signature)
        .map_err(|_| RefusalReason::IdentityUnavailable)
}

pub fn verify_recipient_acknowledgement(
    request: &SignedIdentityMessage,
    acknowledgement: &SignedIdentityMessage,
    recipient_identity: &CommunicationVerifyingIdentity,
    now_epoch_secs: u64,
) -> Result<(), RefusalReason> {
    if request.message_kind != IdentityMessageKind::Request
        || acknowledgement.message_kind != IdentityMessageKind::Acknowledgement
        || acknowledgement.sender_id != request.recipient_id
        || acknowledgement.recipient_id != request.sender_id
        || acknowledgement.polis_id != request.polis_id
        || acknowledgement.conversation_id != request.conversation_id
        || acknowledgement.correlation_id != request.correlation_id
        || acknowledgement.causation_id != request.message_id
        || acknowledgement.replay_id != request.replay_id
    {
        return Err(RefusalReason::InvalidRequest);
    }
    verify_signed_identity_message(
        acknowledgement,
        recipient_identity,
        &request.sender_id,
        now_epoch_secs,
    )
}
