# GitHub larger-runner preflight

Use the native `csdlc-github runner-preflight` operation before treating a
queued larger-runner job as capacity pressure. A hosted runner can report
`Ready` while its group policy or dispatch route still prevents assignment.

The request is typed JSON:

```json
{
  "repository": "agent-logic/agent-design-language",
  "organization": "agent-logic",
  "runner_group_id": 3,
  "expected_label": "adl-ubuntu-24.04-16core",
  "workflow_path": ".github/workflows/ci.yaml",
  "canary_job_id": null,
  "expected_run_id": null,
  "expected_head_sha": null,
  "expected_pull_request": null,
  "queue_timeout_seconds": 300,
  "token_file": null
}
```

Run it with the installed v2 binary:

```text
.adl/bin/csdlc-v2/csdlc-github runner-preflight --request request.json
```

With `token_file` null, the shared resolver uses the approved environment or
default `$HOME/keys/github.token` source. The output never includes the token.
It reports:

- hosted runner label, status, and configured maximum;
- runner-group name and visibility;
- explicit access for the target repository;
- workflow-restriction state and selected workflow refs;
- stale, foreign, malformed, or unverified workflow refs;
- capacity, policy, dispatchability, and one overall classification.

## Classification

- `policy_ineligible`: repository selection, selected visibility, or
  branch-independent workflow policy is wrong. Waiting for capacity cannot help.
- `capacity_unavailable`: the expected hosted label is absent, not Ready, or
  has a zero configured maximum.
- `configuration_eligible_dispatch_unproven`: runner and group settings look
  correct, but no assigned canary proves the route. `Ready` alone is not enough.
- `dispatch_unavailable`: a bounded canary exceeded its queue threshold without
  assignment, completed without assignment, or ran on a different label.
- `eligible`: a canary job was assigned to the expected label. Continue to watch
  it to a terminal result for release evidence.

To evaluate a live canary, set `canary_job_id`, `expected_run_id`, and
`expected_head_sha` together. `expected_pull_request` is an optional additional
pin when GitHub's run response exposes a pull-request association. The command proves
dispatch only when the job is assigned to the expected label and runner group
and its workflow run exactly matches the requested workflow path, run, and
head SHA, plus the PR number when supplied. This prevents an old or unrelated job from satisfying the
gate.
Keep the queue threshold bounded. Do not repeatedly wait on an unassigned job;
record `dispatch_unavailable`, temporarily select a known working GitHub-hosted
label if urgent work must continue, and repair the larger-runner route separately.

The preflight is read-only. It does not change runner groups, repository access,
workflow restrictions, variables, or AWS resources.
