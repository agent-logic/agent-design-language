# MLX adapter qualification (#903)

`mlx` is a registered ADL workflow provider. It consumes the same editable
provider sidecar, profile expansion, cost admission and last-known-good reload
validation as other workflow providers. It supports macOS on Apple silicon only;
other platforms fail at adapter construction without a network request.

## Configuration

```yaml
schema: adl.provider_reload_sidecar.v1
providers:
  mlx_smoke:
    profile: mlx:llama-3.2-3b
    config:
      endpoint: http://127.0.0.1:18093/v1/chat/completions
      max_output_tokens: 16
      timeout_secs: 60
```

Alternatively use `type: mlx`, an explicit `default_model`, and the same config
limits. A local snapshot path may be the model identity if the offline server is
started with that exact path. The response must return that exact model identity.
`default_model` is reserved by MLX-LM and is rejected as a model value.

Only literal loopback HTTP endpoints with the exact `/v1/chat/completions` path
are accepted. Credentials, query strings, redirects and environment proxies are
not used. No credential is required for this isolated loopback server. Keep it
private; this is not a public authenticated inference service.

Requests are bounded to 16 KiB prompt bytes, 1–512 generated tokens and a
1–120 second client deadline. Responses are limited to 1 MiB and exactly one
nonempty text choice with a matching model. HTTP error bodies and prompts are
never copied into adapter error messages. Client deadlines do not guarantee that
an independently managed nonstream MLX-LM server has stopped generating.

## Local prerequisites and execution

Use a separately managed MLX-LM environment and an already available compatible
model. The issue-local environment pins MLX-LM 0.31.3 and MLX/Metal 0.32.2;
full installed versions will be retained with actual proof. Offline mode must be
set on the server, since MLX-LM can otherwise download requested missing models.
The adapter does not create services, manage models, or impose an OS memory cap.

See [the concrete execution plan](MLX_EXECUTION_PLAN.md). Its bounded server
launcher is only for the declared smoke; it is not installed into shared runtime
configuration. Run the explicit ignored test documented in
`adl/tests/mlx_provider.rs` to prove the real canonical reload → workflow Runtime
execution → registered MLX adapter path. A direct `Provider::complete` fixture
is not that proof.

## Validation status

- Library compilation and strict Clippy passed.
- Twelve deterministic internal protocol/negative tests passed.
- Public profile and supported-host loopback production-workflow tests passed.
  Unsupported-platform test is declared for non-Apple CI; not run on this Mac.
  Actual hardware smoke remains explicitly ignored and unrun.
- Corrected adapter source review passed; root separately reviewed and ran the
  reviewer-authored integration fixture. Real Metal smoke and required CI remain pending.
- Resident-kernel conversation support, general local-model support, comparative
  performance, and remote users' access to this machine are not claimed.

PVF: internal adapter tests are deterministic local protocol/negative proof,
CPU and bounded loopback fixtures, required provider-platform gate. Public profile
and unsupported-platform tests are deterministic CPU contracts. The explicitly
ignored actual-platform test is an observational local Metal/GPU proof and a
required milestone gate; its ordinary-CI skip is not a pass. No cloud resources
or credentials are involved.
