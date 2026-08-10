use ::adl::adl::ProviderSpec;
use ::adl::provider::{
    build_speech_provider, expand_provider_profiles, AudioContainer, AudioEncoding,
    DeepgramSpeechProvider, SpeechErrorKind, SpeechProvider, SynthesisRequest,
    TranscriptionRequest,
};
use ::adl::provider_substrate::{provider_substrate_v1, CapabilityModeV1};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;
use tiny_http::{Header, Response, Server, StatusCode};

use super::support::adl_doc_from_yaml;

const TEST_KEY_ENV: &str = "ADL_DEEPGRAM_TEST_KEY_66";
const TEST_KEY_FILE_ENV: &str = "ADL_DEEPGRAM_TEST_KEY_FILE_66";
const TEST_KEY: &str = "deepgram-test-secret-66";

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn provider_spec(endpoint: &str, timeout_secs: u64) -> ProviderSpec {
    let mut config = HashMap::new();
    config.insert("endpoint".to_string(), Value::String(endpoint.to_string()));
    config.insert("allow_test_endpoint".to_string(), Value::Bool(true));
    config.insert(
        "api_key_env".to_string(),
        Value::String(TEST_KEY_ENV.to_string()),
    );
    config.insert("timeout_secs".to_string(), json!(timeout_secs));
    ProviderSpec {
        id: Some("deepgram_test".to_string()),
        profile: None,
        kind: "deepgram".to_string(),
        base_url: None,
        default_model: Some("aura-2-pluto-en".to_string()),
        config,
    }
}

fn with_test_key<T>(run: impl FnOnce() -> T) -> T {
    let _guard = env_lock().lock().expect("Deepgram test env lock");
    let previous = env::var_os(TEST_KEY_ENV);
    env::set_var(TEST_KEY_ENV, TEST_KEY);
    let result = run();
    match previous {
        Some(value) => env::set_var(TEST_KEY_ENV, value),
        None => env::remove_var(TEST_KEY_ENV),
    }
    result
}

fn without_test_credentials<T>(run: impl FnOnce() -> T) -> T {
    let _guard = env_lock().lock().expect("Deepgram test env lock");
    let previous_key = env::var_os(TEST_KEY_ENV);
    let previous_file = env::var_os(TEST_KEY_FILE_ENV);
    env::remove_var(TEST_KEY_ENV);
    env::remove_var(TEST_KEY_FILE_ENV);
    let result = run();
    match previous_key {
        Some(value) => env::set_var(TEST_KEY_ENV, value),
        None => env::remove_var(TEST_KEY_ENV),
    }
    match previous_file {
        Some(value) => env::set_var(TEST_KEY_FILE_ENV, value),
        None => env::remove_var(TEST_KEY_FILE_ENV),
    }
    result
}

fn test_server(
    handler: impl FnOnce(tiny_http::Request) + Send + 'static,
) -> (String, thread::JoinHandle<()>) {
    let server = Server::http("127.0.0.1:0").expect("bind loopback server");
    let endpoint = format!("http://{}", server.server_addr());
    let handle = thread::spawn(move || {
        let request = server.recv().expect("receive Deepgram request");
        handler(request);
    });
    (endpoint, handle)
}

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).expect("valid fixture header")
}

fn wav_fixture() -> Vec<u8> {
    let mut wav = b"RIFF".to_vec();
    wav.extend_from_slice(&[40, 0, 0, 0]);
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&[16, 0, 0, 0, 1, 0, 1, 0]);
    wav.extend_from_slice(&24_000_u32.to_le_bytes());
    wav.extend_from_slice(&48_000_u32.to_le_bytes());
    wav.extend_from_slice(&[2, 0, 16, 0]);
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&[4, 0, 0, 0]);
    wav.extend_from_slice(&[0, 0, 1, 0]);
    wav
}

fn empty_wav_fixture() -> Vec<u8> {
    let mut wav = wav_fixture();
    wav.truncate(44);
    wav[4..8].copy_from_slice(&36_u32.to_le_bytes());
    wav[40..44].copy_from_slice(&0_u32.to_le_bytes());
    wav
}

fn mp3_fixture() -> Vec<u8> {
    let mut mp3 = b"ID3\x04\x00\x00\x00\x00\x00\x00".to_vec();
    mp3.extend_from_slice(&[0xff, 0xfb, 0x90, 0x00]);
    mp3.resize(427, 0);
    mp3
}

fn synthesis_request() -> SynthesisRequest {
    SynthesisRequest {
        text: "A small deterministic fixture.".to_string(),
        model: "aura-2".to_string(),
        voice: "aura-2-pluto-en".to_string(),
        encoding: AudioEncoding::Linear16,
        container: AudioContainer::Wav,
        sample_rate: 24_000,
    }
}

#[test]
fn deepgram_profiles_expand_and_advertise_only_native_speech() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  speech:
    profile: "deepgram:aura-2-pluto-en"
agents:
  a1:
    provider: "speech"
    model: "aura-2-pluto-en"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let expanded = expand_provider_profiles(&doc).expect("expand Deepgram profile");
    let spec = &expanded.providers["speech"];
    assert_eq!(spec.kind, "deepgram");
    assert_eq!(spec.default_model.as_deref(), Some("aura-2-pluto-en"));
    let substrate = provider_substrate_v1("speech", spec).expect("Deepgram substrate");
    assert_eq!(substrate.vendor, "deepgram");
    assert!(substrate.capabilities.speech_synthesis.supported);
    assert!(substrate.capabilities.speech_transcription.supported);
    assert_eq!(
        substrate.capabilities.speech_synthesis.mode,
        CapabilityModeV1::Native
    );
    assert!(!substrate.capabilities.tool_calling.supported);
    assert!(!substrate.capabilities.structured_json.supported);
}

#[test]
fn deepgram_synthesis_constructs_request_and_validates_wav() {
    let expected_audio = wav_fixture();
    let response_audio = expected_audio.clone();
    let (endpoint, server) = test_server(move |mut request| {
        assert_eq!(request.method().as_str(), "POST");
        assert!(request.url().starts_with("/v1/speak?"));
        assert!(request.url().contains("model=aura-2-pluto-en"));
        assert!(request.url().contains("encoding=linear16"));
        assert!(request.url().contains("container=wav"));
        let authorization = request
            .headers()
            .iter()
            .find(|item| item.field.equiv("authorization"))
            .expect("authorization header");
        assert_eq!(authorization.value.as_str(), format!("Token {TEST_KEY}"));
        let mut body = String::new();
        request
            .as_reader()
            .read_to_string(&mut body)
            .expect("read synthesis body");
        assert_eq!(
            serde_json::from_str::<Value>(&body).expect("JSON body"),
            json!({"text": "A small deterministic fixture."})
        );
        request
            .respond(
                Response::from_data(response_audio)
                    .with_header(header("content-type", "audio/wav"))
                    .with_header(header("dg-request-id", "tts-request-66"))
                    .with_header(header("dg-model-name", "aura-2-pluto-en"))
                    .with_header(header("dg-char-count", "30")),
            )
            .expect("respond synthesis");
    });
    with_test_key(|| {
        let provider = DeepgramSpeechProvider::from_spec("speech", &provider_spec(&endpoint, 5))
            .expect("construct provider");
        let result = provider
            .synthesize(&synthesis_request())
            .expect("synthesize fixture");
        assert_eq!(result.audio, expected_audio);
        assert_eq!(
            result.provenance.request_id.as_deref(),
            Some("tts-request-66")
        );
        assert_eq!(result.provenance.input_units, 30);
        assert_eq!(result.provenance.request_identity.len(), 64);
    });
    server.join().expect("join synthesis server");
}

#[test]
fn deepgram_mp3_uses_fixed_media_contract_without_invalid_query_parameters() {
    let response_audio = mp3_fixture();
    let expected_audio = response_audio.clone();
    let (endpoint, server) = test_server(move |request| {
        assert!(request.url().contains("encoding=mp3"));
        assert!(!request.url().contains("container="));
        assert!(!request.url().contains("sample_rate="));
        request
            .respond(
                Response::from_data(response_audio)
                    .with_header(header("content-type", "audio/mpeg")),
            )
            .expect("respond MP3 synthesis");
    });
    with_test_key(|| {
        let provider = DeepgramSpeechProvider::from_spec("speech", &provider_spec(&endpoint, 5))
            .expect("construct provider");
        let mut request = synthesis_request();
        request.encoding = AudioEncoding::Mp3;
        request.container = AudioContainer::None;
        request.sample_rate = 22_050;
        let result = provider
            .synthesize(&request)
            .expect("synthesize MP3 fixture");
        assert_eq!(result.audio, expected_audio);
        assert_eq!(result.sample_rate, 22_050);
    });
    server.join().expect("join MP3 synthesis server");
}

#[test]
fn deepgram_transcription_parses_structured_result() {
    let (endpoint, server) = test_server(move |mut request| {
        assert!(request.url().starts_with("/v1/listen?"));
        assert!(request.url().contains("model=nova-3"));
        assert!(request.url().contains("language=en-US"));
        let mut audio = Vec::new();
        request
            .as_reader()
            .read_to_end(&mut audio)
            .expect("read audio");
        assert_eq!(audio, wav_fixture());
        let payload = json!({
            "metadata": {
                "request_id": "stt-request-66",
                "duration": 1.25,
                "model_info": {"model-uuid": {"name": "nova-3"}}
            },
            "results": {"channels": [{
                "detected_language": "en-US",
                "alternatives": [{
                "transcript": "Cognitive spacetime.",
                "confidence": 0.98,
                "words": [{"word": "cognitive", "start": 0.0, "end": 0.4, "confidence": 0.99}]
            }]}]}
        });
        request
            .respond(
                Response::from_string(payload.to_string())
                    .with_header(header("content-type", "application/json")),
            )
            .expect("respond transcription");
    });
    with_test_key(|| {
        let provider = DeepgramSpeechProvider::from_spec("speech", &provider_spec(&endpoint, 5))
            .expect("construct provider");
        let result = provider
            .transcribe(&TranscriptionRequest {
                audio: wav_fixture(),
                content_type: "audio/wav".to_string(),
                model: "nova-3".to_string(),
                language: "en-US".to_string(),
            })
            .expect("transcribe fixture");
        assert_eq!(result.transcript, "Cognitive spacetime.");
        assert_eq!(result.language.as_deref(), Some("en-US"));
        assert_eq!(result.provenance.audio_seconds, Some(1.25));
        assert_eq!(result.words.len(), 1);
    });
    server.join().expect("join transcription server");
}

#[test]
fn deepgram_transcription_falls_back_to_requested_language() {
    let (endpoint, server) = test_server(move |request| {
        let payload = json!({
            "metadata": {"request_id": "stt-language-fallback", "duration": 0.1},
            "results": {"channels": [{"alternatives": [{
                "transcript": "Hello.",
                "confidence": 0.9,
                "words": []
            }]}]}
        });
        request
            .respond(
                Response::from_string(payload.to_string())
                    .with_header(header("content-type", "application/json")),
            )
            .expect("respond transcription fallback");
    });
    with_test_key(|| {
        let provider = DeepgramSpeechProvider::from_spec("speech", &provider_spec(&endpoint, 5))
            .expect("construct provider");
        let result = provider
            .transcribe(&TranscriptionRequest {
                audio: wav_fixture(),
                content_type: "audio/wav".to_string(),
                model: "nova-3".to_string(),
                language: "en-US".to_string(),
            })
            .expect("transcribe fixture");
        assert_eq!(result.language.as_deref(), Some("en-US"));
    });
    server.join().expect("join transcription fallback server");
}

#[test]
fn deepgram_rejects_unapproved_endpoint_and_unsupported_media_before_network() {
    let mut unapproved = provider_spec("https://example.com", 5);
    unapproved.config.remove("allow_test_endpoint");
    let error = match build_speech_provider("speech", &unapproved) {
        Ok(_) => panic!("unapproved endpoint must fail"),
        Err(error) => error,
    };
    assert_eq!(error.kind, SpeechErrorKind::InvalidInput);

    with_test_key(|| {
        let provider =
            DeepgramSpeechProvider::from_spec("speech", &provider_spec("http://127.0.0.1:9", 5))
                .expect("construct provider");
        let mut request = synthesis_request();
        request.container = AudioContainer::None;
        let error = provider
            .synthesize(&request)
            .expect_err("unsupported media must fail");
        assert_eq!(error.kind, SpeechErrorKind::UnsupportedMedia);

        let error = provider
            .transcribe(&TranscriptionRequest {
                audio: b"not-a-wav".to_vec(),
                content_type: "audio/wav".to_string(),
                model: "nova-3".to_string(),
                language: "en-US".to_string(),
            })
            .expect_err("declared WAV must contain WAV bytes");
        assert_eq!(error.kind, SpeechErrorKind::UnsupportedMedia);
    });
}

#[test]
fn deepgram_rejects_empty_or_truncated_media_payloads() {
    for (response_audio, encoding, container, sample_rate, content_type) in [
        (
            empty_wav_fixture(),
            AudioEncoding::Linear16,
            AudioContainer::Wav,
            24_000,
            "audio/wav",
        ),
        (
            vec![0xff, 0xfb, 0x90, 0x00],
            AudioEncoding::Mp3,
            AudioContainer::None,
            22_050,
            "audio/mpeg",
        ),
        (
            b"ID3\x04\x00\x00\x00\x00\x00\x00".to_vec(),
            AudioEncoding::Mp3,
            AudioContainer::None,
            22_050,
            "audio/mpeg",
        ),
    ] {
        let (endpoint, server) = test_server(move |request| {
            request
                .respond(
                    Response::from_data(response_audio)
                        .with_header(header("content-type", content_type)),
                )
                .expect("respond unusable synthesis media");
        });
        with_test_key(|| {
            let provider =
                DeepgramSpeechProvider::from_spec("speech", &provider_spec(&endpoint, 5))
                    .expect("construct provider");
            let mut request = synthesis_request();
            request.encoding = encoding;
            request.container = container;
            request.sample_rate = sample_rate;
            assert_eq!(
                provider
                    .synthesize(&request)
                    .expect_err("unusable synthesis media must fail")
                    .kind,
                SpeechErrorKind::MalformedResponse
            );
        });
        server.join().expect("join unusable media server");
    }

    with_test_key(|| {
        let provider =
            DeepgramSpeechProvider::from_spec("speech", &provider_spec("http://127.0.0.1:9", 5))
                .expect("construct provider");
        for (audio, content_type) in [
            (empty_wav_fixture(), "audio/wav"),
            (vec![0xff, 0xfb, 0x90, 0x00], "audio/mpeg"),
            (b"ID3\x04\x00\x00\x00\x00\x00\x00".to_vec(), "audio/mpeg"),
        ] {
            assert_eq!(
                provider
                    .transcribe(&TranscriptionRequest {
                        audio,
                        content_type: content_type.to_string(),
                        model: "nova-3".to_string(),
                        language: "en-US".to_string(),
                    })
                    .expect_err("unusable transcription media must fail")
                    .kind,
                SpeechErrorKind::UnsupportedMedia
            );
        }
    });
}

#[test]
fn deepgram_configuration_and_credentials_fail_closed() {
    let mut wrong_kind = provider_spec("http://127.0.0.1:9", 5);
    wrong_kind.kind = "http".to_string();
    let error = match build_speech_provider("speech", &wrong_kind) {
        Ok(_) => panic!("non-Deepgram provider kind must fail"),
        Err(error) => error,
    };
    assert_eq!(error.kind, SpeechErrorKind::InvalidInput);

    assert_eq!(
        DeepgramSpeechProvider::from_spec("speech", &provider_spec(":not-a-url", 5))
            .expect_err("invalid endpoint must fail")
            .kind,
        SpeechErrorKind::InvalidInput
    );
    for endpoint in [
        "https://user:secret@api.deepgram.com",
        "https://api.deepgram.com?api_key=secret",
        "https://api.deepgram.com#secret",
    ] {
        let mut spec = provider_spec(endpoint, 5);
        spec.config.remove("allow_test_endpoint");
        let error = DeepgramSpeechProvider::from_spec("speech", &spec)
            .expect_err("credential-bearing endpoint must fail");
        assert_eq!(error.kind, SpeechErrorKind::InvalidInput);
        assert!(!format!("{error:?}").contains("secret"));
    }
    assert_eq!(
        DeepgramSpeechProvider::from_spec("speech", &provider_spec("http://127.0.0.1:9", 0))
            .expect_err("zero timeout must fail")
            .kind,
        SpeechErrorKind::InvalidInput
    );

    without_test_credentials(|| {
        let mut spec = provider_spec("http://127.0.0.1:9", 5);
        spec.config.insert(
            "api_key_file_env".to_string(),
            Value::String(TEST_KEY_FILE_ENV.to_string()),
        );
        let provider = DeepgramSpeechProvider::from_spec("speech", &spec)
            .expect("construct credential-free provider");
        assert_eq!(
            provider
                .synthesize(&synthesis_request())
                .expect_err("missing credentials must fail before transport")
                .kind,
            SpeechErrorKind::Authentication
        );
    });
}

#[test]
fn deepgram_request_validation_rejects_missing_and_mismatched_inputs() {
    with_test_key(|| {
        let provider =
            DeepgramSpeechProvider::from_spec("speech", &provider_spec("http://127.0.0.1:9", 5))
                .expect("construct provider");

        let mut empty_text = synthesis_request();
        empty_text.text.clear();
        assert_eq!(
            provider
                .synthesize(&empty_text)
                .expect_err("empty synthesis text must fail")
                .kind,
            SpeechErrorKind::InvalidInput
        );

        let mut invalid_mp3_rate = synthesis_request();
        invalid_mp3_rate.encoding = AudioEncoding::Mp3;
        invalid_mp3_rate.container = AudioContainer::None;
        assert_eq!(
            provider
                .synthesize(&invalid_mp3_rate)
                .expect_err("noncanonical MP3 sample rate must fail")
                .kind,
            SpeechErrorKind::UnsupportedMedia
        );

        assert_eq!(
            provider
                .transcribe(&TranscriptionRequest {
                    audio: Vec::new(),
                    content_type: "audio/wav".to_string(),
                    model: "nova-3".to_string(),
                    language: "en-US".to_string(),
                })
                .expect_err("empty transcription audio must fail")
                .kind,
            SpeechErrorKind::InvalidInput
        );
        assert_eq!(
            provider
                .transcribe(&TranscriptionRequest {
                    audio: wav_fixture(),
                    content_type: "audio/ogg".to_string(),
                    model: "nova-3".to_string(),
                    language: "en-US".to_string(),
                })
                .expect_err("unsupported transcription media must fail")
                .kind,
            SpeechErrorKind::UnsupportedMedia
        );
    });
}

#[test]
fn deepgram_maps_provider_statuses_and_never_exposes_credentials() {
    for (status, expected) in [
        (401, SpeechErrorKind::Authentication),
        (429, SpeechErrorKind::Throttling),
        (400, SpeechErrorKind::InvalidInput),
        (415, SpeechErrorKind::UnsupportedMedia),
    ] {
        let (endpoint, server) = test_server(move |request| {
            request
                .respond(Response::empty(StatusCode(status)))
                .expect("respond error");
        });
        with_test_key(|| {
            let provider =
                DeepgramSpeechProvider::from_spec("speech", &provider_spec(&endpoint, 5))
                    .expect("construct provider");
            let debug = format!("{provider:?}");
            assert!(!debug.contains(TEST_KEY));
            let error = provider
                .synthesize(&synthesis_request())
                .expect_err("status must fail");
            assert_eq!(error.kind, expected);
            assert!(!error.to_string().contains(TEST_KEY));
            assert!(!format!("{error:?}").contains(TEST_KEY));
        });
        server.join().expect("join status server");
    }
}

#[test]
fn deepgram_rejects_non_audio_and_malformed_transcription_responses() {
    let (tts_endpoint, tts_server) = test_server(|request| {
        request
            .respond(
                Response::from_string(r#"{"error":"not audio"}"#)
                    .with_header(header("content-type", "application/json")),
            )
            .expect("respond malformed audio");
    });
    with_test_key(|| {
        let provider =
            DeepgramSpeechProvider::from_spec("speech", &provider_spec(&tts_endpoint, 5))
                .expect("construct provider");
        assert_eq!(
            provider
                .synthesize(&synthesis_request())
                .expect_err("JSON is not audio")
                .kind,
            SpeechErrorKind::MalformedResponse
        );
    });
    tts_server.join().expect("join malformed TTS server");

    let (stt_endpoint, stt_server) = test_server(|request| {
        request
            .respond(
                Response::from_string(r#"{"metadata":{},"results":{"channels":[]}}"#)
                    .with_header(header("content-type", "application/json")),
            )
            .expect("respond malformed transcript");
    });
    with_test_key(|| {
        let provider =
            DeepgramSpeechProvider::from_spec("speech", &provider_spec(&stt_endpoint, 5))
                .expect("construct provider");
        assert_eq!(
            provider
                .transcribe(&TranscriptionRequest {
                    audio: wav_fixture(),
                    content_type: "audio/wav".to_string(),
                    model: "nova-3".to_string(),
                    language: "en-US".to_string(),
                })
                .expect_err("missing alternatives must fail")
                .kind,
            SpeechErrorKind::MalformedResponse
        );
    });
    stt_server.join().expect("join malformed STT server");
}

#[test]
fn deepgram_timeout_has_stable_kind() {
    let (endpoint, server) = test_server(|request| {
        thread::sleep(Duration::from_millis(1_250));
        let _ = request.respond(Response::from_data(wav_fixture()));
    });
    with_test_key(|| {
        let provider = DeepgramSpeechProvider::from_spec("speech", &provider_spec(&endpoint, 1))
            .expect("construct provider");
        let error = provider
            .synthesize(&synthesis_request())
            .expect_err("request must time out");
        assert_eq!(error.kind, SpeechErrorKind::Timeout);
    });
    server.join().expect("join timeout server");
}

#[test]
#[ignore = "requires an operator-approved Deepgram credential and writes redacted issue evidence"]
fn deepgram_pluto_nova3_round_trip() {
    let spec = ProviderSpec {
        id: Some("deepgram_live".to_string()),
        profile: None,
        kind: "deepgram".to_string(),
        base_url: None,
        default_model: Some("aura-2-pluto-en".to_string()),
        config: HashMap::new(),
    };
    let provider = DeepgramSpeechProvider::from_spec("deepgram_live", &spec)
        .expect("construct live Deepgram provider");
    let synthesis = provider
        .synthesize(&SynthesisRequest {
            text: "Cognitive spacetime is ready.".to_string(),
            model: "aura-2".to_string(),
            voice: "aura-2-pluto-en".to_string(),
            encoding: AudioEncoding::Linear16,
            container: AudioContainer::Wav,
            sample_rate: 24_000,
        })
        .expect("live Pluto synthesis");
    let transcription = provider
        .transcribe(&TranscriptionRequest {
            audio: synthesis.audio.clone(),
            content_type: "audio/wav".to_string(),
            model: "nova-3".to_string(),
            language: "en-US".to_string(),
        })
        .expect("live Nova-3 transcription");
    let normalized_transcript = transcription
        .transcript
        .chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect::<String>();
    let expected_phrase_matched = normalized_transcript == "cognitivespacetimeisready";
    assert!(
        expected_phrase_matched,
        "Nova-3 must recover the synthesized phrase"
    );

    let character_cost = synthesis.provenance.input_units as f64 / 1_000.0 * 0.030;
    let transcription_cost =
        transcription.provenance.audio_seconds.unwrap_or_default() / 60.0 * 0.0077;
    let receipt = json!({
        "schema": "adl.deepgram.live_receipt.v1",
        "provider": "deepgram",
        "result": "pass",
        "expected_phrase_matched": expected_phrase_matched,
        "synthesis": {
            "model": synthesis.provenance.model,
            "voice": synthesis.provenance.voice,
            "request_id": synthesis.provenance.request_id,
            "request_identity": synthesis.provenance.request_identity,
            "elapsed_ms": synthesis.provenance.elapsed_ms,
            "character_count": synthesis.provenance.input_units,
            "encoding": synthesis.encoding,
            "container": synthesis.container,
            "sample_rate": synthesis.sample_rate,
            "audio_bytes": synthesis.audio.len()
        },
        "transcription": {
            "model": transcription.provenance.model,
            "request_id": transcription.provenance.request_id,
            "request_identity": transcription.provenance.request_identity,
            "elapsed_ms": transcription.provenance.elapsed_ms,
            "audio_seconds": transcription.provenance.audio_seconds,
            "language": transcription.language
        },
        "estimated_cost_usd": character_cost + transcription_cost,
        "pricing_snapshot": {
            "observed_on": "2026-08-09",
            "source": "https://deepgram.com/pricing",
            "aura_2_usd_per_1000_characters": 0.030,
            "nova_3_prerecorded_usd_per_minute": 0.0077
        },
        "redaction": {
            "source_text_retained": false,
            "audio_retained": false,
            "credential_retained": false,
            "authorization_retained": false,
            "transcript_retained": false
        }
    });
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root");
    let receipt_path = repo_root.join(".csdlc/evidence/66/deepgram-live-receipt.json");
    fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
        .expect("create receipt directory");
    fs::write(
        receipt_path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&receipt).expect("serialize receipt")
        ),
    )
    .expect("write redacted receipt");
}
