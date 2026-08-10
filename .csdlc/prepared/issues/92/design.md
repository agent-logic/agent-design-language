# Runtime TLS and mTLS Standardization

## Decision

Runtime transport security uses one Rustls policy and loading layer.

- Axum is the sole HTTP, HTTPS, and WSS server stack.
- Quinn remains the Guardian-to-Guardian QUIC transport.
- Both consume shared identity, trust, protocol, and crypto-provider policy.
- Runtime consumes externally provisioned CA-issued certificate material.
- Runtime does not issue certificates, mutate host trust, trust served leaves as
  roots, or disable certificate verification.

The word `certificate` in the transport layer means an X.509 certificate.
Guardian application authority credentials remain a separate signed-object
system and must not be presented as transport certificates.

## Profiles

### Public server TLS

Browser and Unity endpoints use ordinary server-auth TLS for a real DNS name.
The certificate may be provisioned by ACM or another external CA. Runtime sees
only an atomic certificate-chain path and private-key path. Clients use their
normal trusted roots and DNS verification.

### Private mutual TLS

Guardian peers use Quinn and Rustls TLS 1.3 with an explicit private CA root,
client and server identities, and `WebPkiClientVerifier`. A peer identity is
authenticated only after the Rustls handshake succeeds. Application admission
then binds that transport identity to the enrolled Guardian identity,
generation, trust domain, and authority state.

Runtime control, browser, and Unity endpoints use server-authenticated TLS plus
their declared application authorization. They do not join the Guardian mTLS
trust domain.

## Shared configuration

The shared policy exposes declarative values rather than transport-specific
commands:

- certificate chain path;
- private key path;
- expected DNS identity;
- trust source: platform/public roots or explicit private CA bundle;
- client authentication: disabled or required;
- minimum TLS protocol policy;
- ALPN values supplied by the transport adapter;
- handshake and idle timeouts owned by the transport adapter.

The shared loader uses maintained Rustls, WebPKI, and PEM APIs to construct
validated configuration. Axum and Quinn adapters translate that configuration
only where their library APIs differ. No endpoint parses certificates in its
handler or implements chain verification.

## Startup and rotation

Startup fails before listening when identity material is missing, malformed,
untrusted under the declared profile, unsuitable for server authentication,
outside its validity period, incomplete, or mismatched with the expected DNS
name. Private mTLS additionally fails on absent roots or unusable client
identity.

Certificate rotation remains infrastructure-owned. ACM-managed ingress rotates
outside Runtime. When Axum terminates TLS directly, an external manager replaces
the files atomically and the supervisor performs a controlled Runtime restart
with normal connection drain. Runtime does not own a certificate watcher,
hot-reload state machine, issuance path, or trust-store mutation.

## Removal

The following product behavior is deleted rather than deprecated:

- `RuntimeTlsBootstrapMode::LocalSelfSigned`;
- Runtime `rcgen` certificate issuance and reissue;
- macOS trust install, verify, and remove operations;
- trust ownership receipts and keychain digests;
- local certificate generation manifests;
- documentation and proofs that instruct clients to trust the served leaf.

Test-only certificate construction remains permitted only through a test CA
that signs separate server/client leaves. Production dependencies must not
retain `rcgen` solely for fixtures.

## API and proof truth

The served OpenAPI document must describe the authentication actually enforced
by the listener. Signed command authorization is not `mutualTLS`. ACIP/WSS has
one canonical frame contract per endpoint.

Operational proof starts the production Axum listener with externally
provisioned CA-issued material. Browser and Unity proof use normal platform
trust and DNS verification. Negative proof demonstrates rejection of a
self-signed leaf without adding it to a trust store.

## Migration order

1. Introduce shared configuration and validation types.
2. Route all Axum listeners through the shared builder.
3. Route Quinn trust and identity loading through the shared policy types.
4. Remove local issuance and host-trust mutation product code.
5. Reconcile Runtime init, OpenAPI, architecture, and consumer documentation.
6. Replace leaf-as-root fixtures and proof scripts with test-CA or external-CA
   material according to the proof boundary.
7. Run focused positive and negative TLS/mTLS validation and live consumer
   proof before releasing the Unity gate.

## Rejected alternatives

- `mkcert -install`: still introduces local CA issuance and host-trust mutation.
- Trusting the existing self-signed leaf: preserves the defect.
- Replacing Quinn with HTTP/2: unrelated transport redesign with no security
  simplification.
- Automatic QUIC fallback: duplicates transport semantics without evidence of
  need.
- Runtime-owned ACME: moves certificate issuance into the product boundary.

## Review basis

This design incorporates three independent repository reviews and Gemini 3.1
Pro review. The reviews agreed on removing production self-signed issuance,
unifying duplicated Axum/Rustls loading, separating transport identity from
application authority, and replacing misleading proof. The design deliberately
rejects Gemini's `mkcert -install` suggestion because it violates the operator's
no-host-trust-mutation constraint.
