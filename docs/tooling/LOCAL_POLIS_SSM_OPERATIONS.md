# Local Polis SSM Operations

ADL uses AWS Systems Manager as an operations-plane bridge for approved local
polis hosts. SSM can collect host status and bounded operational evidence; it
does not own polis state, governance authority, provider choice, memory, or
runtime semantics.

Run the WP-08 local polis proof with the Agent Logic AWS profile:

```bash
ADL_AWS_LOCAL_POLIS_SSM_ACCOUNT_SHA256="<operator-approved-agent-logic-account-sha256>" \
  bash adl/tools/run_wp08_local_polis_ssm_proof.sh \
    --out docs/milestones/v0.91.7/review/runtime/wp08_local_polis_ssm_4687 \
    --profile agent-logic-admin \
    --region us-west-2
```

Do not derive the expected account hash from the profile being checked. The
hash is an approval gate: it must come from a trusted operator-approved source
for the Agent Logic business account. If the profile resolves to a different
account, the runner exits before SSM inventory or command mutation.

The runner:

- verifies the approved account hash before any SSM mutation;
- discovers the online `wuji`, `nessus`, and `opticon` managed nodes;
- runs bounded status commands through SSM;
- enables CloudWatch output under `/adl/local-polis-ssm/4687`;
- writes `local_polis_ssm_summary.json` with raw account, instance, and command
  identifiers replaced by SHA-256 hashes.

Validate retained proof with:

```bash
python3 adl/tools/validate_wp08_local_polis_ssm_proof.py \
  docs/milestones/v0.91.7/review/runtime/wp08_local_polis_ssm_4687/local_polis_ssm_summary.json
```

Operational boundaries follow ADR 0035: SSM is useful for health, inventory,
bounded command execution, and evidence collection, but it is not a polis
control plane.
