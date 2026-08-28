# GCP-A operator reauth and proof packet

Issue: #490

Status: resolved by operator reauthentication and retained as historical
blocker/proof-routing evidence.

## Exact blocker

Before operator reauthentication, the read-only GCP-A proof script was executed,
but all readbacks that required a fresh access token failed with:

```text
There was a problem refreshing your current auth tokens: Reauthentication failed. cannot prompt during non-interactive execution.
Please run:

  $ gcloud auth login
```

The active account/config readbacks were retained at the time of the failure.
After the operator completed `gcloud auth login`, the same issue-owned
read-only proof was rerun successfully and the accepted denominator is now
recorded in
`docs/operations/cloud/gcp/decisions/GCP_HIERARCHY_COST_DECISION.md`. No GCP
mutation was performed during either run.

## Operator one-command reauth and proof

If the account session expires again, run this from a terminal where
browser-based login is allowed:

```bash
gcloud auth login daniel@agent-logic.ai && gcloud config set account daniel@agent-logic.ai && cd /Volumes/FastWork/adl-worktrees/adl-issue-490-gcp-hierarchy-cost-decision && bash docs/milestones/v0.92.1/evidence/cloud/gcp-a/run-readonly-decision-readbacks.sh && bash .csdlc/prepared/issues/490/validate-gcp-a-decision.sh .
```

This command changes only local `gcloud` authentication/configuration state and
then runs issue-owned read-only GCP readbacks plus the local redaction and
no-mutation validator. It does not create, update, delete, enable, disable, or
apply any GCP resource.

## After successful rerun

Completed for #490:

1. Retained the successful readbacks under
   `docs/milestones/v0.92.1/evidence/cloud/gcp-a/readbacks/`.
2. Updated `docs/operations/cloud/gcp/decisions/GCP_HIERARCHY_COST_DECISION.md`
   to accepted decisions only where readbacks prove the denominator.
3. Re-ran the local decision validator and typed C-SDLC doctor.
4. Finalized typed C-SDLC state to `phase=implemented`.
