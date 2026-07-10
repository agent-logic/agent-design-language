# WP-12 Security And CAV Pre-v0.92 Requirements Gate (#4656)

## Metadata

- Issue: `#4656`
- Parent sprint: `#4639`
- Milestone: `v0.91.7`
- Status: gate recorded; #4657 operations and #4658 schema projection integrated; other child blockers open
- Machine-readable companion: `docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json`

## Purpose

Record the WP-12 security and CAV gate that prevents `v0.92` activation from
silently inheriting unresolved security, CAV, protocol, access-rule, or public
evidence claims.

This packet is not a launch-readiness claim. It is the issue-local control
surface for `#4656`: the security/CAV requirements are now named, owner-bound,
and fail closed until the child issues prove them or the operator explicitly
approves a non-claim with evidence and risk.

## Findings

### F-4656-01: WP-12 security/CAV readiness remains blocked until child proofs land

Severity: blocker

Evidence:

- `docs/milestones/v0.91.7/review/runtime/SOAK2_REVIEW_BLOCKER_REGISTER_4844.md`
  assigns `capability_envelope` and `security_cav_boundary` to `#4656` and
  keeps them blocked before final activation claims.
- `docs/milestones/v0.91.7/review/runtime/soak2_4682/security_cav_boundary/proof_packet.json`
  records a fail-closed paused-boundary proof, not a complete CAV readiness
  proof.
- `docs/milestones/v0.91.7/features/SECURITY_RESIDUAL_READINESS_v0.91.7.md`
  requires threat-model review and explicit blocker or approval records before
  security work can move out of activation.

Disposition:

- `#4656` records the gate and requirement ledger.
- `#4914`, `#4917`, and `#4920` remain required before adversarial CAV,
  tamper-evident custody, key-management, witness, or receipt readiness can be
  claimed.

### F-4656-02: ACIP/A2A security cannot be claimed until protocol and access owners settle

Severity: blocker

Evidence:

- `docs/milestones/v0.91.7/features/ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md`
  states that unresolved activation-path protocol decisions block `v0.92`
  unless the operator explicitly scopes them out.
- `docs/milestones/v0.91.7/review/runtime/ACIP_RUNTIME_STREAM_SUBSTRATE_4900.md`
  selects WebSocket and `tokio-tungstenite` for carrier mechanics, but
  explicitly excludes protobuf, production WebSocket authentication, reconnect
  scheduling, cross-polis transport, and access-rule closure.
- `docs/milestones/v0.91.7/review/runtime/SOAK2_REVIEW_BLOCKER_REGISTER_4844.md`
  keeps `acip_a2a_path` blocked under `#4658`.

Disposition:

- `#4658` records integrated proof for schema/protobuf projection and
  consumption posture.
- `#4659` owns the bounded WebSocket transport path that consumes the #4900
  carrier decision.
- `#4660` owns external-agent access rules, denial behavior, and trust
  boundaries.

### F-4656-03: Public evidence, profile privacy, and launch narrative need custody and key proof

Severity: high

Evidence:

- `docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md` says capability envelope,
  witnesses, receipt, and activation-path security may be consumed only as
  integrated proof, operator-scoped-out evidence, or blocked evidence.
- `docs/milestones/v0.91.7/features/SECURITY_RESIDUAL_READINESS_v0.91.7.md`
  keeps public evidence and profile privacy requirements in scope.

Disposition:

- `#4917` must provide tamper-evident custody proof before public evidence or
  profile privacy language relies on retained artifacts.
- `#4920` must provide key rotation and break-glass policy before durable
  signing, custody, or recovery claims become activation evidence.

## Requirement Ledger

| Requirement | Owner | Current state | v0.92 impact |
| --- | --- | --- | --- |
| Capability envelope, witness, and receipt readiness | `#4656` with `#4914`, `#4917`, `#4920` | blocked until child proofs | Blocks capability-envelope and birthday-evidence claims. |
| Security/CAV activation boundary | `#4656` with `#4914`, `#4917`, `#4920` | blocked until child proofs | Blocks security/CAV readiness. |
| SSM and local polis operations readiness | `#4657` | integrated proven | Supports SSM operations claims; secret values, provider/model execution, governance authority, and unattended mutation remain non-claims. |
| ACIP/A2A schema and protobuf projection | `#4658` with `#4900` | integrated proven | Schema/projection ready; #4659 and #4660 still block full ACIP/A2A readiness. |
| ACIP WebSocket transport path | `#4659` with `#4900` | child issue open | Blocks transport activation. |
| External-agent access rules | `#4660` | child issue open | Blocks external-agent trust claims. |
| CAV runtime red-blue proof | `#4914` | child issue open | Blocks adversarial CAV claims. |
| Tamper-evident evidence custody | `#4917` | child issue open | Blocks public evidence and profile privacy claims. |
| Key rotation and break-glass policy | `#4920` | child issue open | Blocks durable key-management claims. |
| Curiosity/Constructability security gates | `#4637` with `#4692`, `#4693` | blocked until promoted or non-claimed | Blocks public claims if promoted into activation. |

## Activation Rule

WP-12 and `v0.92` may consume a row only when it is one of:

- `integrated_proven`: implementation runs in the integrated path with retained
  evidence;
- `operator_scoped_out`: implementation proof is outside `v0.92` activation
  scope, with evidence, risk, and operator approval recorded;
- `blocked_with_evidence`: named missing evidence or decision prevents
  activation use.

Any row still marked `child_issue_open`, `blocked_until_child_proofs`, or
`blocked_until_promoted_or_non_claimed` is not activation-ready.

## Non-Claims

- This packet does not claim `v0.92` security readiness.
- This packet does not claim ACIP/A2A/protobuf protocol completion.
- This packet does not approve external-agent trust, production WebSocket
  authentication, or launch-scope CAV claims.
- This packet does not move unresolved activation-path work to `v0.93` without
  explicit operator approval.

## Validation

Focused local validation for this packet:

```sh
python3 -m json.tool docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json >/dev/null
python3 - <<'PY'
import json
path = "docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json"
data = json.load(open(path, encoding="utf-8"))
assert data["schema"] == "adl.wp12.security_cav_gate.v1"
assert data["issue"] == 4656
assert data["requirements"]
integrated = {"ssm_and_local_polis_secret_readiness", "acip_a2a_schema_and_protobuf_projection"}
for row in data["requirements"]:
    for key in ("id", "owner_issue", "state", "v092_disposition", "evidence", "required_before_claim"):
        assert row.get(key), (row.get("id"), key)
    if row["id"] in integrated:
        assert row["state"] == "integrated_proven", row["id"]
    else:
        assert row["state"] != "integrated_proven", row["id"]
PY
git diff --check
```

This validation proves the retained ledger is parseable, records #4657 and
#4658 as integrated rows, and keeps every other row fail-closed until later
WP-12 owner issues provide proof or explicit scoped-out approval.
