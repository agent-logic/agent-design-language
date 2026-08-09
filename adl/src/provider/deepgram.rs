use crate::adl;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{StatusCode, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::env;
use std::fmt;
use std::fs;
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

const DEEPGRAM_BASE_URL: &str = "https://api.deepgram.com";
const DEFAULT_API_KEY_ENV: &str = "DEEPGRAM_API_KEY";
const DEFAULT_API_KEY_FILE_ENV: &str = "ADL_DEEPGRAM_API_KEY_FILE";
const DEFAULT_TIMEOUT_SECS: u64 = 120;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AudioEncoding {
    Linear16,
    Mp3,
}

impl AudioEncoding {
    fn query_value(self) -> &'static str {
        match self {
            Self::Linear16 => "linear16",
            Self::Mp3 => "mp3",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AudioContainer {
    Wav,
    None,
}

impl AudioContainer {
    fn query_value(self) -> &'static str {
        match self {
            Self::Wav => "wav",
            Self::None => "none",
        }
    }
}

#[derive(Clone)]
pub struct SynthesisRequest {
    pub text: String,
    pub model: String,
    pub voice: String,
    pub encoding: AudioEncoding,
    pub container: AudioContainer,
    pub sample_rate: u32,
}

impl fmt::Debug for SynthesisRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SynthesisRequest")
            .field("text", &"<redacted>")
            .field("text_chars", &self.text.chars().count())
            .field("model", &self.model)
            .field("voice", &self.voice)
            .field("encoding", &self.encoding)
            .field("container", &self.container)
            .field("sample_rate", &self.sample_rate)
            .finish()
    }
}

#[derive(Clone)]
pub struct TranscriptionRequest {
    pub audio: Vec<u8>,
    pub content_type: String,
    pub model: String,
    pub language: String,
}

impl fmt::Debug for TranscriptionRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TranscriptionRequest")
            .field("audio", &"<redacted>")
            .field("audio_bytes", &self.audio.len())
            .field("content_type", &self.content_type)
            .field("model", &self.model)
            .field("language", &self.language)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeechProvenance {
    pub provider: String,
    pub operation: String,
    pub model: String,
    pub voice: Option<String>,
    pub request_id: Option<String>,
    pub request_identity: String,
    pub elapsed_ms: u64,
    pub input_units: u64,
    pub audio_seconds: Option<f64>,
}

#[derive(Clone, PartialEq)]
pub struct SynthesisResult {
    pub audio: Vec<u8>,
    pub encoding: AudioEncoding,
    pub container: AudioContainer,
    pub sample_rate: u32,
    pub provenance: SpeechProvenance,
}

impl fmt::Debug for SynthesisResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SynthesisResult")
            .field("audio", &"<redacted>")
            .field("audio_bytes", &self.audio.len())
            .field("encoding", &self.encoding)
            .field("container", &self.container)
            .field("sample_rate", &self.sample_rate)
            .field("provenance", &self.provenance)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TranscriptWord {
    pub word: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TranscriptionResult {
    pub transcript: String,
    pub confidence: f64,
    pub language: Option<String>,
    pub words: Vec<TranscriptWord>,
    pub provenance: SpeechProvenance,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SpeechErrorKind {
    Authentication,
    Throttling,
    InvalidInput,
    UnsupportedMedia,
    Timeout,
    Transport,
    MalformedResponse,
}

#[derive(Debug)]
pub struct SpeechProviderError {
    pub kind: SpeechErrorKind,
    message: String,
}

impl SpeechProviderError {
    fn new(kind: SpeechErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for SpeechProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "deepgram {:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for SpeechProviderError {}

pub trait SpeechProvider: Send + Sync {
    fn synthesize(
        &self,
        request: &SynthesisRequest,
    ) -> Result<SynthesisResult, SpeechProviderError>;

    fn transcribe(
        &self,
        request: &TranscriptionRequest,
    ) -> Result<TranscriptionResult, SpeechProviderError>;
}

pub struct DeepgramSpeechProvider {
    provider_id: String,
    base_url: Url,
    api_key_env: String,
    api_key_file_env: String,
    client: Client,
}

impl fmt::Debug for DeepgramSpeechProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeepgramSpeechProvider")
            .field("provider_id", &self.provider_id)
            .field("base_url", &self.base_url)
            .field("credentials", &"<redacted-source>")
            .finish()
    }
}

pub fn build_speech_provider(
    provider_id: &str,
    spec: &adl::ProviderSpec,
) -> Result<Box<dyn SpeechProvider>, SpeechProviderError> {
    if spec.kind.trim() != "deepgram" {
        return Err(SpeechProviderError::new(
            SpeechErrorKind::InvalidInput,
            format!("provider '{provider_id}' is not a deepgram speech provider"),
        ));
    }
    Ok(Box::new(DeepgramSpeechProvider::from_spec(
        provider_id,
        spec,
    )?))
}

impl DeepgramSpeechProvider {
    pub fn from_spec(
        provider_id: &str,
        spec: &adl::ProviderSpec,
    ) -> Result<Self, SpeechProviderError> {
        let configured = spec
            .config
            .get("endpoint")
            .and_then(|value| value.as_str())
            .or(spec.base_url.as_deref())
            .unwrap_or(DEEPGRAM_BASE_URL)
            .trim_end_matches('/');
        let base_url = normalize_base_url(configured)?;
        let is_official = base_url.scheme() == "https"
            && matches!(
                base_url.host_str(),
                Some("api.deepgram.com" | "api.eu.deepgram.com")
            );
        let loopback = matches!(
            base_url.host_str(),
            Some("localhost" | "127.0.0.1" | "::1" | "[::1]")
        );
        let allow_test_endpoint = spec
            .config
            .get("allow_test_endpoint")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        if !(is_official || loopback && allow_test_endpoint) {
            return Err(SpeechProviderError::new(
                SpeechErrorKind::InvalidInput,
                "Deepgram credentials may only be sent to an official HTTPS endpoint or an explicitly enabled loopback test endpoint",
            ));
        }

        let timeout_secs = spec
            .config
            .get("timeout_secs")
            .and_then(|value| value.as_u64())
            .unwrap_or(DEFAULT_TIMEOUT_SECS);
        if timeout_secs == 0 {
            return Err(SpeechProviderError::new(
                SpeechErrorKind::InvalidInput,
                "timeout_secs must be greater than zero",
            ));
        }
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .map_err(|_| {
                SpeechProviderError::new(
                    SpeechErrorKind::Transport,
                    "failed to construct HTTP client",
                )
            })?;
        Ok(Self {
            provider_id: provider_id.to_string(),
            base_url,
            api_key_env: config_string(spec, "api_key_env")
                .unwrap_or_else(|| DEFAULT_API_KEY_ENV.to_string()),
            api_key_file_env: config_string(spec, "api_key_file_env")
                .unwrap_or_else(|| DEFAULT_API_KEY_FILE_ENV.to_string()),
            client,
        })
    }

    fn credential(&self) -> Result<Zeroizing<String>, SpeechProviderError> {
        if let Ok(value) = env::var(&self.api_key_env) {
            if !value.trim().is_empty() {
                return Ok(Zeroizing::new(value.trim().to_string()));
            }
        }
        if let Ok(path) = env::var(&self.api_key_file_env) {
            if !path.trim().is_empty() {
                let value = fs::read_to_string(path.trim()).map_err(|_| {
                    SpeechProviderError::new(
                        SpeechErrorKind::Authentication,
                        "configured Deepgram key file could not be read",
                    )
                })?;
                if !value.trim().is_empty() {
                    return Ok(Zeroizing::new(value.trim().to_string()));
                }
            }
        }
        Err(SpeechProviderError::new(
            SpeechErrorKind::Authentication,
            format!(
                "no Deepgram credential is available from {} or the configured key-file environment variable",
                self.api_key_env
            ),
        ))
    }

    fn authorization(&self) -> Result<HeaderValue, SpeechProviderError> {
        let key = self.credential()?;
        let mut value = Zeroizing::new(Vec::with_capacity(6 + key.len()));
        value.extend_from_slice(b"Token ");
        value.extend_from_slice(key.as_bytes());
        HeaderValue::from_bytes(value.as_slice()).map_err(|_| {
            SpeechProviderError::new(
                SpeechErrorKind::Authentication,
                "Deepgram credential has an invalid header representation",
            )
        })
    }

    fn endpoint(&self, path: &str) -> Result<Url, SpeechProviderError> {
        self.base_url.join(path).map_err(|_| {
            SpeechProviderError::new(SpeechErrorKind::InvalidInput, "invalid endpoint path")
        })
    }
}

impl SpeechProvider for DeepgramSpeechProvider {
    fn synthesize(
        &self,
        request: &SynthesisRequest,
    ) -> Result<SynthesisResult, SpeechProviderError> {
        validate_synthesis_request(request)?;
        let mut url = self.endpoint("v1/speak")?;
        let provider_model = if request.voice.trim().is_empty() {
            request.model.trim()
        } else {
            request.voice.trim()
        };
        {
            let mut query = url.query_pairs_mut();
            query
                .append_pair("model", provider_model)
                .append_pair("encoding", request.encoding.query_value());
            if request.encoding == AudioEncoding::Linear16 {
                query
                    .append_pair("container", request.container.query_value())
                    .append_pair("sample_rate", &request.sample_rate.to_string());
            }
        }
        let started = Instant::now();
        let response = self
            .client
            .post(url)
            .header(AUTHORIZATION, self.authorization()?)
            .header(CONTENT_TYPE, "application/json")
            .json(&json!({"text": request.text}))
            .send()
            .map_err(map_transport_error)?;
        let response = require_success(response)?;
        let headers = response.headers().clone();
        let audio = response.bytes().map_err(map_transport_error)?.to_vec();
        validate_audio(
            &audio,
            request.encoding,
            request.container,
            request.sample_rate,
            &headers,
        )?;
        let char_count = header_u64(&headers, "dg-char-count")
            .unwrap_or_else(|| request.text.chars().count() as u64);
        Ok(SynthesisResult {
            audio,
            encoding: request.encoding,
            container: request.container,
            sample_rate: request.sample_rate,
            provenance: SpeechProvenance {
                provider: self.provider_id.clone(),
                operation: "synthesis".to_string(),
                model: header_string(&headers, "dg-model-name")
                    .unwrap_or_else(|| request.model.clone()),
                voice: Some(provider_model.to_string()),
                request_id: header_string(&headers, "dg-request-id"),
                request_identity: request_identity(&json!({
                    "operation": "synthesis",
                    "model": request.model,
                    "voice": request.voice,
                    "encoding": request.encoding,
                    "container": request.container,
                    "sample_rate": request.sample_rate,
                    "character_count": request.text.chars().count()
                })),
                elapsed_ms: elapsed_ms(started),
                input_units: char_count,
                audio_seconds: None,
            },
        })
    }

    fn transcribe(
        &self,
        request: &TranscriptionRequest,
    ) -> Result<TranscriptionResult, SpeechProviderError> {
        validate_transcription_request(request)?;
        let mut url = self.endpoint("v1/listen")?;
        url.query_pairs_mut()
            .append_pair("model", request.model.trim())
            .append_pair("language", request.language.trim())
            .append_pair("smart_format", "true");
        let started = Instant::now();
        let response = self
            .client
            .post(url)
            .header(AUTHORIZATION, self.authorization()?)
            .header(CONTENT_TYPE, request.content_type.trim())
            .body(request.audio.clone())
            .send()
            .map_err(map_transport_error)?;
        let response = require_success(response)?;
        let payload: ListenResponse = response.json().map_err(|_| {
            SpeechProviderError::new(
                SpeechErrorKind::MalformedResponse,
                "Deepgram transcription response was not valid JSON",
            )
        })?;
        let alternative = payload
            .results
            .channels
            .first()
            .and_then(|channel| channel.alternatives.first())
            .ok_or_else(|| {
                SpeechProviderError::new(
                    SpeechErrorKind::MalformedResponse,
                    "Deepgram transcription response contained no alternatives",
                )
            })?;
        let words = alternative
            .words
            .iter()
            .map(|word| TranscriptWord {
                word: word.word.clone(),
                start_seconds: word.start,
                end_seconds: word.end,
                confidence: word.confidence,
            })
            .collect();
        Ok(TranscriptionResult {
            transcript: alternative.transcript.clone(),
            confidence: alternative.confidence,
            language: alternative.languages.first().cloned().or_else(|| {
                alternative
                    .words
                    .iter()
                    .find_map(|word| word.language.clone())
            }),
            words,
            provenance: SpeechProvenance {
                provider: self.provider_id.clone(),
                operation: "transcription".to_string(),
                model: payload
                    .metadata
                    .model_name()
                    .unwrap_or_else(|| request.model.clone()),
                voice: None,
                request_id: payload.metadata.request_id,
                request_identity: request_identity(&json!({
                    "operation": "transcription",
                    "model": request.model,
                    "language": request.language,
                    "content_type": request.content_type,
                    "audio_bytes": request.audio.len()
                })),
                elapsed_ms: elapsed_ms(started),
                input_units: request.audio.len() as u64,
                audio_seconds: payload.metadata.duration,
            },
        })
    }
}

fn config_string(spec: &adl::ProviderSpec, key: &str) -> Option<String> {
    spec.config
        .get(key)
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
}

fn normalize_base_url(value: &str) -> Result<Url, SpeechProviderError> {
    let normalized = if value.ends_with("/v1/speak") {
        value.trim_end_matches("/v1/speak")
    } else if value.ends_with("/v1/listen") {
        value.trim_end_matches("/v1/listen")
    } else {
        value
    };
    Url::parse(&format!("{}/", normalized.trim_end_matches('/'))).map_err(|_| {
        SpeechProviderError::new(SpeechErrorKind::InvalidInput, "invalid Deepgram base URL")
    })
}

fn validate_synthesis_request(request: &SynthesisRequest) -> Result<(), SpeechProviderError> {
    if request.text.trim().is_empty()
        || request.model.trim().is_empty()
        || request.voice.trim().is_empty()
        || request.sample_rate == 0
    {
        return Err(SpeechProviderError::new(
            SpeechErrorKind::InvalidInput,
            "text, model, voice, and non-zero sample_rate are required",
        ));
    }
    if !matches!(
        (request.encoding, request.container),
        (AudioEncoding::Linear16, AudioContainer::Wav) | (AudioEncoding::Mp3, AudioContainer::None)
    ) {
        return Err(SpeechProviderError::new(
            SpeechErrorKind::UnsupportedMedia,
            "unsupported encoding/container combination",
        ));
    }
    if request.encoding == AudioEncoding::Mp3 && request.sample_rate != 22_050 {
        return Err(SpeechProviderError::new(
            SpeechErrorKind::UnsupportedMedia,
            "Deepgram MP3 synthesis uses the fixed 22050 Hz sample rate",
        ));
    }
    Ok(())
}

fn validate_transcription_request(
    request: &TranscriptionRequest,
) -> Result<(), SpeechProviderError> {
    if request.audio.is_empty()
        || request.model.trim().is_empty()
        || request.language.trim().is_empty()
    {
        return Err(SpeechProviderError::new(
            SpeechErrorKind::InvalidInput,
            "audio, model, and language are required",
        ));
    }
    if !matches!(
        request.content_type.trim(),
        "audio/wav" | "audio/x-wav" | "audio/mpeg" | "audio/mp3"
    ) {
        return Err(SpeechProviderError::new(
            SpeechErrorKind::UnsupportedMedia,
            "unsupported prerecorded audio content type",
        ));
    }
    let media_matches = match request.content_type.trim() {
        "audio/wav" | "audio/x-wav" => wav_properties(&request.audio).is_some(),
        "audio/mpeg" | "audio/mp3" => is_mp3(&request.audio),
        _ => false,
    };
    if !media_matches {
        return Err(SpeechProviderError::new(
            SpeechErrorKind::UnsupportedMedia,
            "prerecorded audio bytes did not match the declared content type",
        ));
    }
    Ok(())
}

fn validate_audio(
    bytes: &[u8],
    encoding: AudioEncoding,
    container: AudioContainer,
    sample_rate: u32,
    headers: &HeaderMap,
) -> Result<(), SpeechProviderError> {
    let content_type = header_string(headers, "content-type").unwrap_or_default();
    if !content_type.starts_with("audio/") {
        return Err(SpeechProviderError::new(
            SpeechErrorKind::MalformedResponse,
            "Deepgram synthesis response was not audio",
        ));
    }
    let valid = match (encoding, container) {
        (AudioEncoding::Linear16, AudioContainer::Wav) => matches!(
            wav_properties(bytes),
            Some(WavProperties {
                audio_format: 1,
                channels: 1,
                sample_rate: actual_rate,
                bits_per_sample: 16,
            }) if actual_rate == sample_rate
        ),
        (AudioEncoding::Mp3, AudioContainer::None) => sample_rate == 22_050 && is_mp3(bytes),
        _ => false,
    };
    if !valid {
        return Err(SpeechProviderError::new(
            SpeechErrorKind::MalformedResponse,
            "Deepgram synthesis bytes did not match the declared media format",
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WavProperties {
    audio_format: u16,
    channels: u16,
    sample_rate: u32,
    bits_per_sample: u16,
}

fn wav_properties(bytes: &[u8]) -> Option<WavProperties> {
    if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return None;
    }
    let mut offset = 12_usize;
    while offset.checked_add(8)? <= bytes.len() {
        let chunk_id = &bytes[offset..offset + 4];
        let chunk_len = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().ok()?) as usize;
        let data_start = offset.checked_add(8)?;
        let data_end = data_start.checked_add(chunk_len)?;
        if data_end > bytes.len() {
            return None;
        }
        if chunk_id == b"fmt " {
            if chunk_len < 16 {
                return None;
            }
            return Some(WavProperties {
                audio_format: u16::from_le_bytes(
                    bytes[data_start..data_start + 2].try_into().ok()?,
                ),
                channels: u16::from_le_bytes(
                    bytes[data_start + 2..data_start + 4].try_into().ok()?,
                ),
                sample_rate: u32::from_le_bytes(
                    bytes[data_start + 4..data_start + 8].try_into().ok()?,
                ),
                bits_per_sample: u16::from_le_bytes(
                    bytes[data_start + 14..data_start + 16].try_into().ok()?,
                ),
            });
        }
        offset = data_end.checked_add(chunk_len % 2)?;
    }
    None
}

fn is_mp3(bytes: &[u8]) -> bool {
    bytes.starts_with(b"ID3") || (bytes.len() >= 2 && bytes[0] == 0xff && bytes[1] & 0xe0 == 0xe0)
}

fn require_success(response: Response) -> Result<Response, SpeechProviderError> {
    if response.status().is_success() {
        return Ok(response);
    }
    let kind = match response.status() {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => SpeechErrorKind::Authentication,
        StatusCode::TOO_MANY_REQUESTS => SpeechErrorKind::Throttling,
        StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => SpeechErrorKind::InvalidInput,
        StatusCode::UNSUPPORTED_MEDIA_TYPE => SpeechErrorKind::UnsupportedMedia,
        StatusCode::REQUEST_TIMEOUT | StatusCode::GATEWAY_TIMEOUT => SpeechErrorKind::Timeout,
        _ => SpeechErrorKind::Transport,
    };
    Err(SpeechProviderError::new(
        kind,
        format!(
            "request failed with HTTP status {}",
            response.status().as_u16()
        ),
    ))
}

fn map_transport_error(error: reqwest::Error) -> SpeechProviderError {
    if error.is_timeout() {
        SpeechProviderError::new(SpeechErrorKind::Timeout, "request timed out")
    } else {
        SpeechProviderError::new(SpeechErrorKind::Transport, "HTTP transport failed")
    }
}

fn header_string(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(ToString::to_string)
}

fn header_u64(headers: &HeaderMap, name: &str) -> Option<u64> {
    header_string(headers, name)?.parse().ok()
}

fn request_identity(value: &serde_json::Value) -> String {
    let bytes = serde_json::to_vec(value).unwrap_or_default();
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn elapsed_ms(started: Instant) -> u64 {
    started.elapsed().as_millis().try_into().unwrap_or(u64::MAX)
}

#[derive(Deserialize)]
struct ListenResponse {
    metadata: ListenMetadata,
    results: ListenResults,
}

#[derive(Deserialize)]
struct ListenMetadata {
    request_id: Option<String>,
    duration: Option<f64>,
    #[serde(default)]
    model_info: serde_json::Map<String, serde_json::Value>,
}

impl ListenMetadata {
    fn model_name(&self) -> Option<String> {
        self.model_info
            .values()
            .find_map(|value| value.get("name").and_then(|name| name.as_str()))
            .map(ToString::to_string)
    }
}

#[derive(Deserialize)]
struct ListenResults {
    channels: Vec<ListenChannel>,
}

#[derive(Deserialize)]
struct ListenChannel {
    alternatives: Vec<ListenAlternative>,
}

#[derive(Deserialize)]
struct ListenAlternative {
    transcript: String,
    #[serde(default)]
    confidence: f64,
    #[serde(default)]
    languages: Vec<String>,
    #[serde(default)]
    words: Vec<ListenWord>,
}

#[derive(Deserialize)]
struct ListenWord {
    word: String,
    start: f64,
    end: f64,
    #[serde(default)]
    confidence: f64,
    #[serde(default)]
    language: Option<String>,
}
