use std::{
    collections::BTreeMap,
    fmt,
    sync::{Arc, Mutex},
};

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

use super::certificates::{AuthorityCertificate, CertificatePurpose, DistributedCertificateStore};

pub const CAPABILITY_ADVERTISEMENT_SCHEMA: &str = "adl.distributed.capability_advertisement.v1";
const SIGNING_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-CAPABILITY-ADVERTISEMENT-V1\0";
const SIGNATURE_LEN: usize = 64;
const MAX_TEXT_LEN: usize = 128;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityEvidence {
    pub capability: String,
    pub observed_units: u32,
}

impl CapabilityEvidence {
    pub fn new(capability: impl Into<String>, observed_units: u32) -> Self {
        Self {
            capability: capability.into(),
            observed_units,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityAdvertisementBody {
    pub schema: String,
    pub trust_domain: String,
    pub issuer_id: String,
    pub certificate_generation: u64,
    pub sequence: u64,
    pub measured_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
    pub capabilities: Vec<CapabilityEvidence>,
}

impl CapabilityAdvertisementBody {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trust_domain: impl Into<String>,
        issuer_id: impl Into<String>,
        certificate_generation: u64,
        sequence: u64,
        measured_at_unix_secs: u64,
        expires_at_unix_secs: u64,
        capabilities: impl IntoIterator<Item = CapabilityEvidence>,
        policy: &CapabilityAdvertisementPolicy,
    ) -> AdvertisementResult<Self> {
        let body = Self {
            schema: CAPABILITY_ADVERTISEMENT_SCHEMA.to_owned(),
            trust_domain: trust_domain.into(),
            issuer_id: issuer_id.into(),
            certificate_generation,
            sequence,
            measured_at_unix_secs,
            expires_at_unix_secs,
            capabilities: canonicalize(capabilities, policy)?,
        };
        policy.validate_body(&body)?;
        Ok(body)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedCapabilityAdvertisement {
    pub body: CapabilityAdvertisementBody,
    pub authority_certificate: AuthorityCertificate,
    pub signature: Vec<u8>,
}

impl SignedCapabilityAdvertisement {
    pub fn issue(
        body: CapabilityAdvertisementBody,
        authority_certificate: AuthorityCertificate,
        signing_key: &SigningKey,
        policy: &CapabilityAdvertisementPolicy,
    ) -> AdvertisementResult<Self> {
        policy.validate_body(&body)?;
        validate_certificate_binding(&body, &authority_certificate, signing_key.verifying_key())?;
        let certificate_id = authority_certificate
            .certificate_id()
            .map_err(|_| AdvertisementError::CertificateAuthorization)?;
        let signature = signing_key
            .sign(&signing_bytes(&body, &certificate_id)?)
            .to_bytes()
            .to_vec();
        let advertisement = Self {
            body,
            authority_certificate,
            signature,
        };
        if canonical_bytes(&advertisement)?.len() > policy.max_encoded_bytes {
            return Err(AdvertisementError::Oversized);
        }
        Ok(advertisement)
    }

    pub fn encode(&self, policy: &CapabilityAdvertisementPolicy) -> AdvertisementResult<Vec<u8>> {
        let bytes = canonical_bytes(self)?;
        if bytes.len() > policy.max_encoded_bytes {
            return Err(AdvertisementError::Oversized);
        }
        Ok(bytes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCapabilityAdvertisement {
    pub trust_domain: String,
    pub issuer_id: String,
    pub certificate_id: String,
    pub certificate_generation: u64,
    pub sequence: u64,
    pub measured_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
    pub verification_deadline_unix_secs: u64,
    pub capabilities: Vec<CapabilityEvidence>,
}

impl VerifiedCapabilityAdvertisement {
    /// Returns verified evidence only. This type intentionally exposes no lease,
    /// fencing token, placement command, or scheduling authority.
    pub fn evidence(&self) -> &[CapabilityEvidence] {
        &self.capabilities
    }
}

#[derive(Clone, Debug)]
pub struct CapabilityAdvertisementPolicy {
    trust_domain: String,
    max_entries: usize,
    max_units_per_entry: u32,
    max_total_units: u64,
    max_lifetime_secs: u64,
    max_age_secs: u64,
    max_future_skew_secs: u64,
    max_encoded_bytes: usize,
    max_tracked_certificates: usize,
}

impl CapabilityAdvertisementPolicy {
    pub fn new(trust_domain: impl Into<String>) -> AdvertisementResult<Self> {
        let policy = Self {
            trust_domain: trust_domain.into(),
            max_entries: 256,
            max_units_per_entry: 1_000_000,
            max_total_units: 10_000_000,
            max_lifetime_secs: 300,
            max_age_secs: 120,
            max_future_skew_secs: 5,
            max_encoded_bytes: 64 * 1024,
            max_tracked_certificates: 4096,
        };
        validate_text(&policy.trust_domain)?;
        Ok(policy)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_bounds(
        mut self,
        max_entries: usize,
        max_units_per_entry: u32,
        max_total_units: u64,
        max_lifetime_secs: u64,
        max_age_secs: u64,
        max_future_skew_secs: u64,
        max_encoded_bytes: usize,
        max_tracked_certificates: usize,
    ) -> AdvertisementResult<Self> {
        if max_entries == 0
            || max_units_per_entry == 0
            || max_total_units == 0
            || max_lifetime_secs == 0
            || max_age_secs == 0
            || max_encoded_bytes == 0
            || max_tracked_certificates == 0
        {
            return Err(AdvertisementError::InvalidPolicy);
        }
        self.max_entries = max_entries;
        self.max_units_per_entry = max_units_per_entry;
        self.max_total_units = max_total_units;
        self.max_lifetime_secs = max_lifetime_secs;
        self.max_age_secs = max_age_secs;
        self.max_future_skew_secs = max_future_skew_secs;
        self.max_encoded_bytes = max_encoded_bytes;
        self.max_tracked_certificates = max_tracked_certificates;
        Ok(self)
    }

    fn validate_body(&self, body: &CapabilityAdvertisementBody) -> AdvertisementResult<()> {
        if body.schema != CAPABILITY_ADVERTISEMENT_SCHEMA
            || body.trust_domain != self.trust_domain
            || body.certificate_generation == 0
            || body.sequence == 0
        {
            return Err(AdvertisementError::InvalidAdvertisement);
        }
        validate_text(&body.trust_domain)?;
        validate_text(&body.issuer_id)?;
        if body.capabilities.is_empty() || body.capabilities.len() > self.max_entries {
            return Err(AdvertisementError::CapacityExceeded);
        }
        let canonical = canonicalize(body.capabilities.clone(), self)?;
        if canonical != body.capabilities {
            return Err(AdvertisementError::NonCanonical);
        }
        let lifetime = body
            .expires_at_unix_secs
            .checked_sub(body.measured_at_unix_secs)
            .ok_or(AdvertisementError::InvalidLifetime)?;
        if lifetime == 0 || lifetime > self.max_lifetime_secs {
            return Err(AdvertisementError::InvalidLifetime);
        }
        Ok(())
    }

    fn validate_time(
        &self,
        body: &CapabilityAdvertisementBody,
        now_unix_secs: u64,
    ) -> AdvertisementResult<()> {
        let future_limit = now_unix_secs
            .checked_add(self.max_future_skew_secs)
            .ok_or(AdvertisementError::InvalidLifetime)?;
        if body.measured_at_unix_secs > future_limit {
            return Err(AdvertisementError::NotYetValid);
        }
        if now_unix_secs >= body.expires_at_unix_secs {
            return Err(AdvertisementError::Expired);
        }
        if now_unix_secs.saturating_sub(body.measured_at_unix_secs) > self.max_age_secs {
            return Err(AdvertisementError::Stale);
        }
        Ok(())
    }
}

pub struct CapabilityAdvertisementVerifier {
    certificate_store: Arc<DistributedCertificateStore>,
    policy: CapabilityAdvertisementPolicy,
    replay_high_water: Mutex<BTreeMap<String, ReplayState>>,
}

#[derive(Clone, Copy)]
struct ReplayState {
    highest_sequence: u64,
    expires_at_unix_secs: u64,
}

impl CapabilityAdvertisementVerifier {
    pub fn new(
        certificate_store: Arc<DistributedCertificateStore>,
        policy: CapabilityAdvertisementPolicy,
    ) -> Self {
        Self {
            certificate_store,
            policy,
            replay_high_water: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn decode_and_verify(
        &self,
        bytes: &[u8],
        now_unix_secs: u64,
    ) -> AdvertisementResult<VerifiedCapabilityAdvertisement> {
        if bytes.is_empty() || bytes.len() > self.policy.max_encoded_bytes {
            return Err(AdvertisementError::Oversized);
        }
        let advertisement: SignedCapabilityAdvertisement =
            serde_json::from_slice(bytes).map_err(|_| AdvertisementError::Malformed)?;
        if canonical_bytes(&advertisement)? != bytes {
            return Err(AdvertisementError::NonCanonical);
        }
        self.verify(&advertisement, now_unix_secs)
    }

    pub fn verify(
        &self,
        advertisement: &SignedCapabilityAdvertisement,
        now_unix_secs: u64,
    ) -> AdvertisementResult<VerifiedCapabilityAdvertisement> {
        if canonical_bytes(advertisement)?.len() > self.policy.max_encoded_bytes {
            return Err(AdvertisementError::Oversized);
        }
        self.policy.validate_body(&advertisement.body)?;
        self.policy
            .validate_time(&advertisement.body, now_unix_secs)?;
        validate_certificate_body(&advertisement.body, &advertisement.authority_certificate)?;

        let certificate_id = advertisement
            .authority_certificate
            .certificate_id()
            .map_err(|_| AdvertisementError::CertificateAuthorization)?;
        let authorized = self
            .certificate_store
            .authorize(
                &advertisement.body.issuer_id,
                CertificatePurpose::AdvertisementSigning,
                advertisement.body.certificate_generation,
                now_unix_secs,
            )
            .map_err(|_| AdvertisementError::CertificateAuthorization)?;
        if authorized.certificate_id != certificate_id
            || authorized.holder_id != advertisement.body.issuer_id
            || authorized.purpose != CertificatePurpose::AdvertisementSigning
            || authorized.generation != advertisement.body.certificate_generation
        {
            return Err(AdvertisementError::CertificateAuthorization);
        }
        let verification_deadline_unix_secs = advertisement
            .body
            .expires_at_unix_secs
            .min(authorized.authorization_deadline_unix_secs);

        let verifying_key =
            VerifyingKey::from_bytes(&advertisement.authority_certificate.body.subject_public_key)
                .map_err(|_| AdvertisementError::WrongSigner)?;
        let signature = Signature::from_slice(&advertisement.signature)
            .map_err(|_| AdvertisementError::MalformedSignature)?;
        if advertisement.signature.len() != SIGNATURE_LEN
            || verifying_key
                .verify_strict(
                    &signing_bytes(&advertisement.body, &certificate_id)?,
                    &signature,
                )
                .is_err()
        {
            return Err(AdvertisementError::WrongSigner);
        }

        let mut replay = self
            .replay_high_water
            .lock()
            .map_err(|_| AdvertisementError::StateUnavailable)?;
        replay.retain(|_, state| now_unix_secs < state.expires_at_unix_secs);
        if let Some(state) = replay.get_mut(&certificate_id) {
            if advertisement.body.sequence <= state.highest_sequence {
                return Err(AdvertisementError::Replay);
            }
            state.highest_sequence = advertisement.body.sequence;
            state.expires_at_unix_secs = verification_deadline_unix_secs;
        } else {
            if replay.len() >= self.policy.max_tracked_certificates {
                return Err(AdvertisementError::CapacityExceeded);
            }
            replay.insert(
                certificate_id.clone(),
                ReplayState {
                    highest_sequence: advertisement.body.sequence,
                    expires_at_unix_secs: verification_deadline_unix_secs,
                },
            );
        }

        Ok(VerifiedCapabilityAdvertisement {
            trust_domain: advertisement.body.trust_domain.clone(),
            issuer_id: advertisement.body.issuer_id.clone(),
            certificate_id,
            certificate_generation: advertisement.body.certificate_generation,
            sequence: advertisement.body.sequence,
            measured_at_unix_secs: advertisement.body.measured_at_unix_secs,
            expires_at_unix_secs: advertisement.body.expires_at_unix_secs,
            verification_deadline_unix_secs,
            capabilities: advertisement.body.capabilities.clone(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdvertisementError {
    InvalidPolicy,
    InvalidAdvertisement,
    InvalidText,
    InvalidLifetime,
    NotYetValid,
    Expired,
    Stale,
    Replay,
    CapacityExceeded,
    NonCanonical,
    Oversized,
    Malformed,
    MalformedSignature,
    WrongSigner,
    CertificateAuthorization,
    StateUnavailable,
    Encoding,
}

impl AdvertisementError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPolicy => "invalid_policy",
            Self::InvalidAdvertisement => "invalid_advertisement",
            Self::InvalidText => "invalid_text",
            Self::InvalidLifetime => "invalid_lifetime",
            Self::NotYetValid => "not_yet_valid",
            Self::Expired => "expired",
            Self::Stale => "stale",
            Self::Replay => "replay",
            Self::CapacityExceeded => "capacity_exceeded",
            Self::NonCanonical => "non_canonical",
            Self::Oversized => "oversized",
            Self::Malformed => "malformed",
            Self::MalformedSignature => "malformed_signature",
            Self::WrongSigner => "wrong_signer",
            Self::CertificateAuthorization => "certificate_authorization_failed",
            Self::StateUnavailable => "state_unavailable",
            Self::Encoding => "encoding_failed",
        }
    }
}

impl fmt::Display for AdvertisementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for AdvertisementError {}

pub type AdvertisementResult<T> = Result<T, AdvertisementError>;

fn canonicalize(
    capabilities: impl IntoIterator<Item = CapabilityEvidence>,
    policy: &CapabilityAdvertisementPolicy,
) -> AdvertisementResult<Vec<CapabilityEvidence>> {
    let mut canonical = BTreeMap::<String, u32>::new();
    let mut total = 0_u64;
    for evidence in capabilities {
        validate_text(&evidence.capability)?;
        if evidence.observed_units == 0 || evidence.observed_units > policy.max_units_per_entry {
            return Err(AdvertisementError::CapacityExceeded);
        }
        if let Some(existing) = canonical.get(&evidence.capability) {
            if *existing != evidence.observed_units {
                return Err(AdvertisementError::NonCanonical);
            }
            continue;
        }
        if canonical.len() >= policy.max_entries {
            return Err(AdvertisementError::CapacityExceeded);
        }
        total = total
            .checked_add(u64::from(evidence.observed_units))
            .ok_or(AdvertisementError::CapacityExceeded)?;
        if total > policy.max_total_units {
            return Err(AdvertisementError::CapacityExceeded);
        }
        canonical.insert(evidence.capability, evidence.observed_units);
    }
    if canonical.is_empty() {
        return Err(AdvertisementError::CapacityExceeded);
    }
    Ok(canonical
        .into_iter()
        .map(|(capability, observed_units)| CapabilityEvidence {
            capability,
            observed_units,
        })
        .collect())
}

fn validate_certificate_binding(
    body: &CapabilityAdvertisementBody,
    certificate: &AuthorityCertificate,
    signing_key: VerifyingKey,
) -> AdvertisementResult<()> {
    validate_certificate_body(body, certificate)?;
    if certificate.body.subject_public_key != signing_key.to_bytes() {
        return Err(AdvertisementError::WrongSigner);
    }
    Ok(())
}

fn validate_certificate_body(
    body: &CapabilityAdvertisementBody,
    certificate: &AuthorityCertificate,
) -> AdvertisementResult<()> {
    let certificate_body = &certificate.body;
    if certificate_body.purpose != CertificatePurpose::AdvertisementSigning
        || certificate_body.trust_domain != body.trust_domain
        || certificate_body.holder_id != body.issuer_id
        || certificate_body.generation != body.certificate_generation
        || body.measured_at_unix_secs < certificate_body.issued_at_unix_secs
        || body.expires_at_unix_secs > certificate_body.expires_at_unix_secs
    {
        return Err(AdvertisementError::CertificateAuthorization);
    }
    Ok(())
}

#[derive(Serialize)]
struct SigningProjection<'a> {
    certificate_id: &'a str,
    body: &'a CapabilityAdvertisementBody,
}

fn signing_bytes(
    body: &CapabilityAdvertisementBody,
    certificate_id: &str,
) -> AdvertisementResult<Vec<u8>> {
    let projection = serde_jcs::to_vec(&SigningProjection {
        certificate_id,
        body,
    })
    .map_err(|_| AdvertisementError::Encoding)?;
    let mut bytes = Vec::with_capacity(SIGNING_DOMAIN.len() + projection.len());
    bytes.extend_from_slice(SIGNING_DOMAIN);
    bytes.extend_from_slice(&projection);
    Ok(bytes)
}

fn canonical_bytes<T: Serialize>(value: &T) -> AdvertisementResult<Vec<u8>> {
    serde_jcs::to_vec(value).map_err(|_| AdvertisementError::Encoding)
}

fn validate_text(value: &str) -> AdvertisementResult<()> {
    if value.is_empty()
        || value.len() > MAX_TEXT_LEN
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b':'))
    {
        return Err(AdvertisementError::InvalidText);
    }
    Ok(())
}
