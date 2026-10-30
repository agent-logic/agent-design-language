//! PVF: public-contract regression; deterministic, small CPU/memory, no I/O.
//! Release proof role: retained declaration syntax, not provider admission.
//! Release gate: required public source validation; packaged-consumer proof is separate.
use adl_schema::syntax::{
    ollama_endpoint_declaration_valid, remote_endpoint_declaration_valid,
    verification_key_source_valid,
};

#[test]
fn remote_declarations_allow_tls_and_only_loopback_plaintext() {
    for endpoint in [
        "https://example.invalid/api",
        "  https://example.invalid/api  ",
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://[::1]:8080",
    ] {
        assert!(remote_endpoint_declaration_valid(endpoint), "{endpoint}");
    }
    for endpoint in [
        "",
        "not a URL",
        "https://",
        "http://example.invalid",
        "http://localhost.example.invalid",
        "http://127.0.0.2",
        "ftp://localhost",
        "file:///tmp/provider",
    ] {
        assert!(!remote_endpoint_declaration_valid(endpoint), "{endpoint}");
    }
}

#[test]
fn ollama_wire_scheme_check_does_not_claim_remote_admission() {
    for endpoint in [
        "http://localhost:11434",
        " HTTPS://example.invalid ",
        "http://",
    ] {
        assert!(ollama_endpoint_declaration_valid(endpoint), "{endpoint}");
    }
    for endpoint in [
        "",
        "localhost:11434",
        "ftp://example.invalid",
        "file:///provider",
    ] {
        assert!(!ollama_endpoint_declaration_valid(endpoint), "{endpoint}");
    }
    // The legacy wire check accepts a scheme-only declaration, while the
    // stricter remote declaration check rejects its missing host.
    assert!(!remote_endpoint_declaration_valid("http://"));
}

#[test]
fn verification_sources_normalize_only_the_declared_allowlist() {
    for source in ["embedded", " EXPLICIT_KEY ", "Embedded"] {
        assert!(verification_key_source_valid(source), "{source}");
    }
    for source in [
        "",
        "key_file",
        "remote",
        "embedded,explicit_key",
        "explicit-key",
    ] {
        assert!(!verification_key_source_valid(source), "{source}");
    }
}
