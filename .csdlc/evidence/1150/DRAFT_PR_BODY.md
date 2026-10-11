Part of #1150

This checkpoint publishes the native-bound CF-05 preparation packet without
closing the qualification issue.

- preserves all 12 surface/repository/platform cells and all 24 source-mapped
  obligations;
- records 0/12 accepted cells, 12 missing cells, and 24 missing obligations
  blocked from execution;
- identifies reusable historical evidence and its claim boundaries, including
  the retained #915 Q01-Q24 handoff at SHA-256
  `70df5cb0c723367ceb2dee306f455bc0c6515d6276588931cd78e2d2fe418956`;
- records the exact missing scenarios, execution procedures, resources, and
  authorizations without running qualification;
- retains the prompt-only SRP proposal with result `{"status":"not_run"}` as a
  reviewed, non-authoritative artifact;
- defines v0.93.1 as built, tested, and deployable, including the waiting-list
  and hosted-mode paths, while leaving public deployment and live launch to a
  later release step.

The native semantic state's publication template contains the final
implementation PR's required closing keyword. It is not the transport body for
this checkpoint. This file is the exact checkpoint body and intentionally uses
`Part of #1150` so merge cannot close the issue.

Validation:

- native `csdlc validate 1150`: completed, lifecycle digest valid, six-card
  validation passed;
- JSON evidence parsing: passed;
- matrix denominator: exactly 12 cells and 24 obligations;
- focused pre-PR review: completed after disposition of the checkpoint-body
  finding.

No qualification execution, dependency credit, provider call, spending,
public deployment, publication of private artifacts, or live launch occurred.
