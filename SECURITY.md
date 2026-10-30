# Security Policy

## Supported Versions

Security fixes target the active `main` branch. Release-specific backports are assessed by maintainers; milestone names alone do not establish a supported package version. This policy covers the public ADL contracts and CLI. Report private Runtime, CodeFriend or C-SDLC implementation issues to their [repository owners](docs/REPOSITORIES.md).

## Reporting a Vulnerability

Use GitHub's [private vulnerability reporting form](https://github.com/agent-logic/agent-design-language/security/advisories/new) for the canonical `agent-logic/agent-design-language` repository. Private vulnerability reporting is enabled; you need a GitHub account to submit a report.

Include:
- affected file(s)/component(s)
- impact and exploitability
- minimal reproduction
- proposed remediation (if available)

Do not post sensitive exploit details, credentials or customer data in public issues. If you cannot access the private form, email [codefriend@agent-logic.ai](mailto:codefriend@agent-logic.ai) to arrange private reporting. Do not send credentials or customer data in the initial message.

## Triage Expectations

- Initial acknowledgement target: within 3 business days (a target, not a guaranteed response time)
- Severity assessment: critical/high/medium/low
- Fixes are prioritized by exploitability and user impact

## Disclosure

After remediation, publish a public summary including:
- affected versions/commits
- mitigation/fix
- any required user action

## Historical Security Architecture References

These dated design records provide context; they are not current deployment or security certification.

- Threat model (v0.7): `docs/security/THREAT_MODEL_v0.7.md`
- v0.7 design security section: `docs/milestones/v0.7/DESIGN_v0.7.md`
