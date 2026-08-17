//! Authenticated, advisory-only resource-weather evidence.
//!
//! This module intentionally remains unregistered until integration issue #5878. Only verified,
//! bounded observations cross this boundary; the module does not grant scheduling authority or
//! mutate scheduler state.

use std::{
    fmt, fs,
    path::{Component, Path, PathBuf},
};

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use redb::{
    Database, Durability, ReadableDatabase, ReadableTable, ReadableTableMetadata, TableDefinition,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[cfg(not(test))]
use super::authority_store_adapters::AuthorityBoundCertificateStore;
use super::certificates::{AuthorityCertificate, CertificatePurpose, VerifiedCertificate};
#[cfg(test)]
use super::certificates::{DistributedCertificateStore, AUTHORITY_BOUND_CERTIFICATE_ACCESS};

mod certificate_authority_seal {
    pub trait Sealed {}
}

pub trait ResourceWeatherCertificateAuthority: certificate_authority_seal::Sealed {
    fn authorize_weather(
        &self,
        holder_id: &str,
        purpose: CertificatePurpose,
        generation: u64,
        now_unix_secs: u64,
    ) -> Result<VerifiedCertificate, CertificateAuthorityUnavailable>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertificateAuthorityUnavailable;

#[cfg(not(test))]
impl certificate_authority_seal::Sealed for AuthorityBoundCertificateStore {}
#[cfg(not(test))]
impl ResourceWeatherCertificateAuthority for AuthorityBoundCertificateStore {
    fn authorize_weather(
        &self,
        holder_id: &str,
        purpose: CertificatePurpose,
        generation: u64,
        now_unix_secs: u64,
    ) -> Result<VerifiedCertificate, CertificateAuthorityUnavailable> {
        self.authorize(holder_id, purpose, generation, now_unix_secs)
            .map_err(|_| CertificateAuthorityUnavailable)
    }
}

#[cfg(test)]
impl certificate_authority_seal::Sealed for DistributedCertificateStore {}
#[cfg(test)]
impl ResourceWeatherCertificateAuthority for DistributedCertificateStore {
    fn authorize_weather(
        &self,
        holder_id: &str,
        purpose: CertificatePurpose,
        generation: u64,
        now_unix_secs: u64,
    ) -> Result<VerifiedCertificate, CertificateAuthorityUnavailable> {
        self.authorize(
            &AUTHORITY_BOUND_CERTIFICATE_ACCESS,
            holder_id,
            purpose,
            generation,
            now_unix_secs,
        )
        .map_err(|_| CertificateAuthorityUnavailable)
    }
}

#[cfg(test)]
impl certificate_authority_seal::Sealed for std::sync::Arc<DistributedCertificateStore> {}
#[cfg(test)]
impl ResourceWeatherCertificateAuthority for std::sync::Arc<DistributedCertificateStore> {
    fn authorize_weather(
        &self,
        holder_id: &str,
        purpose: CertificatePurpose,
        generation: u64,
        now_unix_secs: u64,
    ) -> Result<VerifiedCertificate, CertificateAuthorityUnavailable> {
        self.as_ref()
            .authorize_weather(holder_id, purpose, generation, now_unix_secs)
    }
}

pub const RESOURCE_WEATHER_SCHEMA: &str = "adl.distributed.resource_weather.v1";
const SIGNING_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-RESOURCE-WEATHER-V1\0";
const SIGNATURE_LEN: usize = 64;
const MAX_TEXT_LEN: usize = 128;
pub const MAX_OBSERVATION_LIFETIME_SECS: u64 = 600;
pub const MAX_FUTURE_SKEW_SECS: u64 = 30;
pub const MAX_PAYLOAD_BYTES: usize = 64 * 1024;
const MAX_DURABLE_RECORD_BYTES: usize = MAX_PAYLOAD_BYTES + 1024;
pub const MAX_HOLDERS: u64 = 16_384;
pub const MAX_AVAILABLE_SLOTS: u16 = 16_384;
pub const MAX_UTILIZATION_PERMILLE: u16 = 1_000;
const WEATHER: TableDefinition<&str, &[u8]> =
    TableDefinition::new("distributed_resource_weather_v1");

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricValue {
    Observed(u16),
    Unavailable,
}

impl MetricValue {
    fn observed(self) -> Option<u16> {
        match self {
            Self::Observed(value) => Some(value),
            Self::Unavailable => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RawResourceMetrics {
    pub cpu_utilization_percent: Option<f64>,
    pub memory_total_bytes: Option<u64>,
    pub memory_used_bytes: Option<u64>,
    pub disk_total_bytes: Option<u64>,
    pub disk_used_bytes: Option<u64>,
    pub gpu_utilization_percent: Option<f64>,
    pub available_slots: Option<u16>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NormalizedResourceMetrics {
    pub cpu_utilization_permille: MetricValue,
    pub memory_utilization_permille: MetricValue,
    pub disk_utilization_permille: MetricValue,
    pub gpu_utilization_permille: MetricValue,
    pub available_slots: Option<u16>,
}

impl NormalizedResourceMetrics {
    pub fn from_raw(raw: RawResourceMetrics, max_available_slots: u16) -> WeatherResult<Self> {
        if raw
            .available_slots
            .is_some_and(|slots| slots > max_available_slots)
        {
            return Err(WeatherError::MetricOutOfBounds);
        }
        Ok(Self {
            cpu_utilization_permille: normalize_percent(raw.cpu_utilization_percent)?,
            memory_utilization_permille: normalize_ratio(
                raw.memory_used_bytes,
                raw.memory_total_bytes,
            )?,
            disk_utilization_permille: normalize_ratio(raw.disk_used_bytes, raw.disk_total_bytes)?,
            gpu_utilization_permille: normalize_percent(raw.gpu_utilization_percent)?,
            available_slots: raw.available_slots,
        })
    }

    fn validate(&self, max_available_slots: u16) -> WeatherResult<()> {
        for metric in [
            self.cpu_utilization_permille,
            self.memory_utilization_permille,
            self.disk_utilization_permille,
            self.gpu_utilization_permille,
        ] {
            if metric
                .observed()
                .is_some_and(|value| value > MAX_UTILIZATION_PERMILLE)
            {
                return Err(WeatherError::MetricOutOfBounds);
            }
        }
        if self
            .available_slots
            .is_some_and(|slots| slots > max_available_slots)
        {
            return Err(WeatherError::MetricOutOfBounds);
        }
        Ok(())
    }

    fn observed_values(&self) -> impl Iterator<Item = u16> + '_ {
        [
            self.cpu_utilization_permille.observed(),
            self.memory_utilization_permille.observed(),
            self.disk_utilization_permille.observed(),
            self.gpu_utilization_permille.observed(),
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action", content = "metrics")]
pub enum WeatherAction {
    Observe(NormalizedResourceMetrics),
    Withdraw,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceWeatherClaims {
    pub schema: String,
    pub trust_domain: String,
    pub holder_id: String,
    pub certificate_id: String,
    pub certificate_generation: u64,
    pub sequence: u64,
    pub sampled_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
    pub action: WeatherAction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationWindow {
    pub sampled_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
}

impl ResourceWeatherClaims {
    pub fn new(
        trust_domain: impl Into<String>,
        holder_id: impl Into<String>,
        certificate_id: impl Into<String>,
        certificate_generation: u64,
        sequence: u64,
        window: ObservationWindow,
        action: WeatherAction,
    ) -> Self {
        Self {
            schema: RESOURCE_WEATHER_SCHEMA.to_owned(),
            trust_domain: trust_domain.into(),
            holder_id: holder_id.into(),
            certificate_id: certificate_id.into(),
            certificate_generation,
            sequence,
            sampled_at_unix_secs: window.sampled_at_unix_secs,
            expires_at_unix_secs: window.expires_at_unix_secs,
            action,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignedResourceWeather {
    pub claims: ResourceWeatherClaims,
    pub certificate: AuthorityCertificate,
    pub signature: Vec<u8>,
}

impl SignedResourceWeather {
    pub fn sign(
        claims: ResourceWeatherClaims,
        certificate: AuthorityCertificate,
        signer: &SigningKey,
    ) -> WeatherResult<Self> {
        let certificate_id = certificate
            .certificate_id()
            .map_err(|_| WeatherError::CertificateRejected)?;
        if certificate_id != claims.certificate_id
            || certificate.body.trust_domain != claims.trust_domain
            || certificate.body.holder_id != claims.holder_id
            || certificate.body.generation != claims.certificate_generation
            || certificate.body.purpose != CertificatePurpose::AdvertisementSigning
            || certificate.body.subject_public_key != signer.verifying_key().to_bytes()
        {
            return Err(WeatherError::CertificateMismatch);
        }
        let signature = signer.sign(&signing_bytes(&claims)?).to_bytes().to_vec();
        Ok(Self {
            claims,
            certificate,
            signature,
        })
    }

    pub fn observation_id(&self) -> WeatherResult<String> {
        let bytes = serde_jcs::to_vec(self).map_err(encoding_error)?;
        Ok(format!("weather_{}", hex::encode(Sha256::digest(bytes))))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherAvailability {
    Available,
    Partial,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlacementWeather {
    pub holder_id: String,
    pub certificate_generation: u64,
    pub sequence: u64,
    pub sampled_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
    pub availability: WeatherAvailability,
    pub pressure_permille: Option<u16>,
    pub available_slots: Option<u16>,
    pub advisory_only: bool,
}

impl PlacementWeather {
    pub fn no_data(holder_id: impl Into<String>) -> Self {
        Self {
            holder_id: holder_id.into(),
            certificate_generation: 0,
            sequence: 0,
            sampled_at_unix_secs: 0,
            expires_at_unix_secs: 0,
            availability: WeatherAvailability::Unavailable,
            pressure_permille: None,
            available_slots: None,
            advisory_only: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WeatherError {
    RelativeDatabasePath,
    DatabasePathIsSymlink,
    InvalidSchema,
    InvalidTrustDomain,
    InvalidHolder,
    InvalidCertificateIdentity,
    InvalidSequence,
    InvalidLifetime,
    NotYetValid,
    Expired,
    WrongTrustDomain,
    WrongCertificatePurpose,
    CertificateMismatch,
    CertificateRejected,
    MalformedSignature,
    InvalidSignature,
    MetricOutOfBounds,
    PayloadTooLarge,
    ReplayRefused,
    ResourceExhausted,
    DurableStateCorrupt,
    Storage(String),
    Encoding(String),
}

impl WeatherError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::RelativeDatabasePath => "relative_database_path",
            Self::DatabasePathIsSymlink => "database_path_is_symlink",
            Self::InvalidSchema => "invalid_schema",
            Self::InvalidTrustDomain => "invalid_trust_domain",
            Self::InvalidHolder => "invalid_holder",
            Self::InvalidCertificateIdentity => "invalid_certificate_identity",
            Self::InvalidSequence => "invalid_sequence",
            Self::InvalidLifetime => "invalid_lifetime",
            Self::NotYetValid => "not_yet_valid",
            Self::Expired => "expired",
            Self::WrongTrustDomain => "wrong_trust_domain",
            Self::WrongCertificatePurpose => "wrong_certificate_purpose",
            Self::CertificateMismatch => "certificate_mismatch",
            Self::CertificateRejected => "certificate_rejected",
            Self::MalformedSignature => "malformed_signature",
            Self::InvalidSignature => "invalid_signature",
            Self::MetricOutOfBounds => "metric_out_of_bounds",
            Self::PayloadTooLarge => "payload_too_large",
            Self::ReplayRefused => "replay_refused",
            Self::ResourceExhausted => "resource_exhausted",
            Self::DurableStateCorrupt => "durable_state_corrupt",
            Self::Storage(_) => "storage_error",
            Self::Encoding(_) => "encoding_error",
        }
    }
}

impl fmt::Display for WeatherError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage(detail) | Self::Encoding(detail) => {
                write!(formatter, "{}: {detail}", self.code())
            }
            _ => formatter.write_str(self.code()),
        }
    }
}

impl std::error::Error for WeatherError {}

pub type WeatherResult<T> = Result<T, WeatherError>;

#[derive(Clone)]
pub struct ResourceWeatherPolicy {
    trust_domain: String,
    max_observation_lifetime_secs: u64,
    max_future_skew_secs: u64,
    max_payload_bytes: usize,
    max_holders: u64,
    max_available_slots: u16,
}

impl ResourceWeatherPolicy {
    pub fn new(trust_domain: impl Into<String>) -> WeatherResult<Self> {
        let trust_domain = trust_domain.into();
        validate_text(&trust_domain, WeatherError::InvalidTrustDomain)?;
        Ok(Self {
            trust_domain,
            max_observation_lifetime_secs: 300,
            max_future_skew_secs: 5,
            max_payload_bytes: 8 * 1024,
            max_holders: 4_096,
            max_available_slots: 4_096,
        })
    }

    pub fn with_bounds(
        mut self,
        max_observation_lifetime_secs: u64,
        max_future_skew_secs: u64,
        max_payload_bytes: usize,
        max_holders: u64,
        max_available_slots: u16,
    ) -> WeatherResult<Self> {
        if max_observation_lifetime_secs == 0
            || max_payload_bytes == 0
            || max_holders == 0
            || max_available_slots == 0
            || max_observation_lifetime_secs > MAX_OBSERVATION_LIFETIME_SECS
            || max_future_skew_secs > MAX_FUTURE_SKEW_SECS
            || max_payload_bytes > MAX_PAYLOAD_BYTES
            || max_holders > MAX_HOLDERS
            || max_available_slots > MAX_AVAILABLE_SLOTS
        {
            return Err(WeatherError::ResourceExhausted);
        }
        self.max_observation_lifetime_secs = max_observation_lifetime_secs;
        self.max_future_skew_secs = max_future_skew_secs;
        self.max_payload_bytes = max_payload_bytes;
        self.max_holders = max_holders;
        self.max_available_slots = max_available_slots;
        Ok(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct DurableWeatherRecord {
    holder_id: String,
    certificate_generation: u64,
    sequence: u64,
    authorization_deadline_unix_secs: u64,
    advertisement_digest: String,
    advertisement: SignedResourceWeather,
}

pub struct ResourceWeatherStore {
    database_path: PathBuf,
    database: Database,
    policy: ResourceWeatherPolicy,
}

impl ResourceWeatherStore {
    pub fn open(
        database_path: impl AsRef<Path>,
        policy: ResourceWeatherPolicy,
    ) -> WeatherResult<Self> {
        let database_path = database_path.as_ref();
        if !database_path.is_absolute() {
            return Err(WeatherError::RelativeDatabasePath);
        }
        reject_symlink_components(database_path)?;
        fs::create_dir_all(
            database_path
                .parent()
                .ok_or(WeatherError::RelativeDatabasePath)?,
        )
        .map_err(storage_error)?;
        reject_symlink_components(database_path)?;
        let database = Database::create(database_path).map_err(storage_error)?;
        {
            let write = database.begin_write().map_err(storage_error)?;
            write.open_table(WEATHER).map_err(storage_error)?;
            write.commit().map_err(storage_error)?;
        }
        let store = Self {
            database_path: database_path.to_path_buf(),
            database,
            policy,
        };
        store.validate_durable_state()?;
        Ok(store)
    }

    pub fn database_path(&self) -> &Path {
        &self.database_path
    }

    pub fn admit<C: ResourceWeatherCertificateAuthority>(
        &self,
        advertisement: SignedResourceWeather,
        certificates: &C,
        now_unix_secs: u64,
    ) -> WeatherResult<PlacementWeather> {
        let authorization_deadline = self.verify(&advertisement, certificates, now_unix_secs)?;
        let holder_id = advertisement.claims.holder_id.clone();
        let generation = advertisement.claims.certificate_generation;
        let sequence = advertisement.claims.sequence;

        let mut write = self.database.begin_write().map_err(storage_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        let mut table = write.open_table(WEATHER).map_err(storage_error)?;
        let existing = table
            .get(holder_id.as_str())
            .map_err(storage_error)?
            .map(|value| decode_record(value.value()))
            .transpose()?;
        if existing.as_ref().is_some_and(|record| {
            generation < record.certificate_generation
                || (generation == record.certificate_generation && sequence <= record.sequence)
        }) {
            return Err(WeatherError::ReplayRefused);
        }
        if existing.is_none() && table.len().map_err(storage_error)? >= self.policy.max_holders {
            return Err(WeatherError::ResourceExhausted);
        }
        let projection = project(&advertisement.claims);
        let advertisement_digest = advertisement.observation_id()?;
        let stored = DurableWeatherRecord {
            holder_id: holder_id.clone(),
            certificate_generation: generation,
            sequence,
            authorization_deadline_unix_secs: authorization_deadline,
            advertisement_digest,
            advertisement,
        };
        let encoded = serde_jcs::to_vec(&stored).map_err(encoding_error)?;
        if encoded.len() > MAX_DURABLE_RECORD_BYTES {
            return Err(WeatherError::PayloadTooLarge);
        }
        table
            .insert(holder_id.as_str(), encoded.as_slice())
            .map_err(storage_error)?;
        drop(table);
        write.commit().map_err(storage_error)?;
        Ok(projection)
    }

    pub fn weather_for<C: ResourceWeatherCertificateAuthority>(
        &self,
        holder_id: &str,
        certificates: &C,
        now_unix_secs: u64,
    ) -> WeatherResult<PlacementWeather> {
        validate_text(holder_id, WeatherError::InvalidHolder)?;
        let read = self.database.begin_read().map_err(storage_error)?;
        let table = read.open_table(WEATHER).map_err(storage_error)?;
        let record = table
            .get(holder_id)
            .map_err(storage_error)?
            .map(|value| decode_record(value.value()))
            .transpose()?;
        let Some(record) = record else {
            return Ok(PlacementWeather::no_data(holder_id));
        };
        self.validate_record(&record, holder_id)?;
        let advertisement = record.advertisement;
        if now_unix_secs >= record.authorization_deadline_unix_secs
            || self
                .verify(&advertisement, certificates, now_unix_secs)
                .is_err()
        {
            return Ok(PlacementWeather::no_data(holder_id));
        }
        Ok(project(&advertisement.claims))
    }

    pub fn snapshot<C: ResourceWeatherCertificateAuthority>(
        &self,
        certificates: &C,
        now_unix_secs: u64,
    ) -> WeatherResult<Vec<PlacementWeather>> {
        let read = self.database.begin_read().map_err(storage_error)?;
        let table = read.open_table(WEATHER).map_err(storage_error)?;
        let mut rows = Vec::new();
        for entry in table.iter().map_err(storage_error)? {
            let (holder, value) = entry.map_err(storage_error)?;
            let holder = holder.value().to_owned();
            let record = decode_record(value.value())?;
            self.validate_record(&record, &holder)?;
            let projection = match record.advertisement {
                advertisement
                    if now_unix_secs < record.authorization_deadline_unix_secs
                        && self
                            .verify(&advertisement, certificates, now_unix_secs)
                            .is_ok() =>
                {
                    project(&advertisement.claims)
                }
                _ => PlacementWeather::no_data(holder),
            };
            rows.push(projection);
        }
        Ok(rows)
    }

    fn verify<C: ResourceWeatherCertificateAuthority>(
        &self,
        advertisement: &SignedResourceWeather,
        certificates: &C,
        now_unix_secs: u64,
    ) -> WeatherResult<u64> {
        let encoded = serde_jcs::to_vec(advertisement).map_err(encoding_error)?;
        if encoded.len() > self.policy.max_payload_bytes {
            return Err(WeatherError::PayloadTooLarge);
        }
        let claims = &advertisement.claims;
        if claims.schema != RESOURCE_WEATHER_SCHEMA {
            return Err(WeatherError::InvalidSchema);
        }
        validate_text(&claims.trust_domain, WeatherError::InvalidTrustDomain)?;
        validate_text(&claims.holder_id, WeatherError::InvalidHolder)?;
        validate_text(
            &claims.certificate_id,
            WeatherError::InvalidCertificateIdentity,
        )?;
        if claims.trust_domain != self.policy.trust_domain {
            return Err(WeatherError::WrongTrustDomain);
        }
        if claims.sequence == 0 || claims.certificate_generation == 0 {
            return Err(WeatherError::InvalidSequence);
        }
        if claims.expires_at_unix_secs <= claims.sampled_at_unix_secs
            || claims.expires_at_unix_secs - claims.sampled_at_unix_secs
                > self.policy.max_observation_lifetime_secs
        {
            return Err(WeatherError::InvalidLifetime);
        }
        if claims.sampled_at_unix_secs
            > now_unix_secs.saturating_add(self.policy.max_future_skew_secs)
        {
            return Err(WeatherError::NotYetValid);
        }
        if now_unix_secs >= claims.expires_at_unix_secs {
            return Err(WeatherError::Expired);
        }
        if let WeatherAction::Observe(metrics) = &claims.action {
            metrics.validate(self.policy.max_available_slots)?;
        }

        let body = &advertisement.certificate.body;
        if body.purpose != CertificatePurpose::AdvertisementSigning {
            return Err(WeatherError::WrongCertificatePurpose);
        }
        let certificate_id = advertisement
            .certificate
            .certificate_id()
            .map_err(|_| WeatherError::CertificateRejected)?;
        if certificate_id != claims.certificate_id
            || body.trust_domain != claims.trust_domain
            || body.holder_id != claims.holder_id
            || body.generation != claims.certificate_generation
        {
            return Err(WeatherError::CertificateMismatch);
        }
        if claims.sampled_at_unix_secs < body.issued_at_unix_secs
            || claims.expires_at_unix_secs > body.expires_at_unix_secs
        {
            return Err(WeatherError::InvalidLifetime);
        }
        let authorized = certificates
            .authorize_weather(
                &claims.holder_id,
                CertificatePurpose::AdvertisementSigning,
                claims.certificate_generation,
                now_unix_secs,
            )
            .map_err(|_| WeatherError::CertificateRejected)?;
        if authorized.certificate_id != claims.certificate_id {
            return Err(WeatherError::CertificateMismatch);
        }
        if advertisement.signature.len() != SIGNATURE_LEN {
            return Err(WeatherError::MalformedSignature);
        }
        let signature = Signature::from_slice(&advertisement.signature)
            .map_err(|_| WeatherError::MalformedSignature)?;
        let key = VerifyingKey::from_bytes(&body.subject_public_key)
            .map_err(|_| WeatherError::CertificateRejected)?;
        key.verify_strict(&signing_bytes(claims)?, &signature)
            .map_err(|_| WeatherError::InvalidSignature)?;
        Ok(authorized
            .authorization_deadline_unix_secs
            .min(claims.expires_at_unix_secs)
            .min(body.expires_at_unix_secs))
    }

    fn validate_durable_state(&self) -> WeatherResult<()> {
        let read = self.database.begin_read().map_err(storage_error)?;
        let table = read.open_table(WEATHER).map_err(storage_error)?;
        if table.len().map_err(storage_error)? > self.policy.max_holders {
            return Err(WeatherError::ResourceExhausted);
        }
        for entry in table.iter().map_err(storage_error)? {
            let (holder, value) = entry.map_err(storage_error)?;
            if value.value().len() > MAX_DURABLE_RECORD_BYTES {
                return Err(WeatherError::DurableStateCorrupt);
            }
            let record = decode_record(value.value())?;
            self.validate_record(&record, holder.value())?;
        }
        Ok(())
    }

    fn validate_record(&self, record: &DurableWeatherRecord, holder: &str) -> WeatherResult<()> {
        if holder != record_holder(record)
            || record.certificate_generation == 0
            || record.sequence == 0
        {
            return Err(WeatherError::DurableStateCorrupt);
        }
        let advertisement = &record.advertisement;
        let claims = &advertisement.claims;
        let body = &advertisement.certificate.body;
        let digest = advertisement
            .observation_id()
            .map_err(|_| WeatherError::DurableStateCorrupt)?;
        let certificate_id = advertisement
            .certificate
            .certificate_id()
            .map_err(|_| WeatherError::DurableStateCorrupt)?;
        let deadline = record.authorization_deadline_unix_secs;
        if claims.schema != RESOURCE_WEATHER_SCHEMA
            || claims.trust_domain != self.policy.trust_domain
            || claims.holder_id != record.holder_id
            || claims.certificate_generation != record.certificate_generation
            || claims.sequence != record.sequence
            || claims.sequence == 0
            || claims.expires_at_unix_secs <= claims.sampled_at_unix_secs
            || claims.expires_at_unix_secs - claims.sampled_at_unix_secs
                > self.policy.max_observation_lifetime_secs
            || body.purpose != CertificatePurpose::AdvertisementSigning
            || body.trust_domain != claims.trust_domain
            || body.holder_id != claims.holder_id
            || body.generation != claims.certificate_generation
            || claims.certificate_id != certificate_id
            || claims.sampled_at_unix_secs < body.issued_at_unix_secs
            || claims.expires_at_unix_secs > body.expires_at_unix_secs
            || deadline > claims.expires_at_unix_secs
            || deadline > body.expires_at_unix_secs
            || record.advertisement_digest != digest
            || serde_jcs::to_vec(advertisement)
                .map_err(|_| WeatherError::DurableStateCorrupt)?
                .len()
                > self.policy.max_payload_bytes
        {
            return Err(WeatherError::DurableStateCorrupt);
        }
        if let WeatherAction::Observe(metrics) = &claims.action {
            metrics
                .validate(self.policy.max_available_slots)
                .map_err(|_| WeatherError::DurableStateCorrupt)?;
        }
        if advertisement.signature.len() != SIGNATURE_LEN {
            return Err(WeatherError::DurableStateCorrupt);
        }
        let signature = Signature::from_slice(&advertisement.signature)
            .map_err(|_| WeatherError::DurableStateCorrupt)?;
        let key = VerifyingKey::from_bytes(&body.subject_public_key)
            .map_err(|_| WeatherError::DurableStateCorrupt)?;
        key.verify_strict(
            &signing_bytes(claims).map_err(|_| WeatherError::DurableStateCorrupt)?,
            &signature,
        )
        .map_err(|_| WeatherError::DurableStateCorrupt)
    }
}

fn project(claims: &ResourceWeatherClaims) -> PlacementWeather {
    let WeatherAction::Observe(metrics) = &claims.action else {
        return PlacementWeather::no_data(&claims.holder_id);
    };
    let observed = metrics.observed_values().collect::<Vec<_>>();
    let availability = if observed.is_empty() && metrics.available_slots.is_none() {
        WeatherAvailability::Unavailable
    } else if observed.len() < 4 || metrics.available_slots.is_none() {
        WeatherAvailability::Partial
    } else {
        WeatherAvailability::Available
    };
    PlacementWeather {
        holder_id: claims.holder_id.clone(),
        certificate_generation: claims.certificate_generation,
        sequence: claims.sequence,
        sampled_at_unix_secs: claims.sampled_at_unix_secs,
        expires_at_unix_secs: claims.expires_at_unix_secs,
        availability,
        pressure_permille: observed.into_iter().max(),
        available_slots: metrics.available_slots,
        advisory_only: true,
    }
}

fn normalize_percent(value: Option<f64>) -> WeatherResult<MetricValue> {
    let Some(value) = value else {
        return Ok(MetricValue::Unavailable);
    };
    if !value.is_finite() || !(0.0..=100.0).contains(&value) {
        return Err(WeatherError::MetricOutOfBounds);
    }
    Ok(MetricValue::Observed((value * 10.0).round() as u16))
}

fn normalize_ratio(used: Option<u64>, total: Option<u64>) -> WeatherResult<MetricValue> {
    match (used, total) {
        (None, None) | (Some(0), Some(0)) => Ok(MetricValue::Unavailable),
        (Some(used), Some(total)) if total > 0 && used <= total => Ok(MetricValue::Observed(
            ((u128::from(used) * 1_000) / u128::from(total)) as u16,
        )),
        _ => Err(WeatherError::MetricOutOfBounds),
    }
}

fn signing_bytes(claims: &ResourceWeatherClaims) -> WeatherResult<Vec<u8>> {
    let encoded = serde_jcs::to_vec(claims).map_err(encoding_error)?;
    let mut bytes = Vec::with_capacity(SIGNING_DOMAIN.len() + encoded.len());
    bytes.extend_from_slice(SIGNING_DOMAIN);
    bytes.extend_from_slice(&encoded);
    Ok(bytes)
}

fn decode_record(bytes: &[u8]) -> WeatherResult<DurableWeatherRecord> {
    serde_json::from_slice(bytes).map_err(|_| WeatherError::DurableStateCorrupt)
}

fn record_holder(record: &DurableWeatherRecord) -> &str {
    &record.holder_id
}

fn validate_text(value: &str, error: WeatherError) -> WeatherResult<()> {
    if value.is_empty()
        || value.len() > MAX_TEXT_LEN
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(error);
    }
    Ok(())
}

fn reject_symlink_components(path: &Path) -> WeatherResult<()> {
    let mut current = PathBuf::new();
    for component in path.components() {
        match component {
            Component::RootDir | Component::Prefix(_) | Component::Normal(_) => {
                current.push(component.as_os_str());
            }
            Component::CurDir | Component::ParentDir => {
                return Err(WeatherError::RelativeDatabasePath);
            }
        }
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(WeatherError::DatabasePathIsSymlink);
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
            Err(error) => return Err(storage_error(error)),
        }
    }
    Ok(())
}

fn storage_error(error: impl fmt::Display) -> WeatherError {
    WeatherError::Storage(error.to_string())
}

fn encoding_error(error: impl fmt::Display) -> WeatherError {
    WeatherError::Encoding(error.to_string())
}
