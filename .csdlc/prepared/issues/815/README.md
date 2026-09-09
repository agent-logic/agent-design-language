# Issue #815 cloud mutation authorization

AWS #727 and GCP #731 mutation entrypoints share one fail-closed authorization
contract: an operator signs the canonical bounded JSON packet with an external
Ed25519 key, and the runtime validator verifies it against
`$HOME/keys/adl-cloud-authorization.allowed_signers` before any provider
mutation. Editing a packet, plan, projection, identity, bound, or expiry after
signing invalidates authority.

The canonical payload is the whole packet serialized as sorted compact JSON
with a trailing newline, except
`operator_authorization.signature` is omitted. Sign with OpenSSH namespace
`adl-cloud-authorization-v1`. The trusted allowed-signers file is an external
operator-controlled public-key anchor, not a repository file, and must not be
group- or world-writable.

For GCP, start from `GCP_MUTATION_AUTHORIZATION.template.json`. The signature
binds the exact repository, issue, project, region, zone, network, subnet,
service-account impersonation, spend limits, cleanup expiry, rollback commands,
and SHA-256 digests of both the saved binary plan and reviewed JSON projection.
The historical v1 packet remains evidence of the earlier run, not reusable
mutation authority.

For AWS, use the v2 template and #727 runbook. In addition to signature and
identity checks, validation hashes the actual `.tfplan`, verifies the digest
sidecar, derives JSON from those same bytes with `terraform show -json`, and
compares that projection to the signed reviewed JSON.

Read-only provider inspection does not require a mutation signature. Only the
two mutation entrypoints consume these authorization packets. Run the local
negative proof with:

```sh
bash .csdlc/prepared/issues/815/test-cloud-authorization.sh
```

This proof creates an ephemeral test signer and mocked Terraform executable
inside the bound worktree. It performs no provider calls and grants no live
authority.
