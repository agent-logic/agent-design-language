# AWS-B evidence packet

Issue: #485 (`AWS-B`)

This packet contains redacted readback evidence for the AWS access and billing baseline.

Retained evidence rules:

- Do not retain credentials, token values, credential-file contents, raw environment dumps, or payment details.
- Redact account IDs, ARNs, user IDs, role IDs, email-like strings, and request IDs where practical.
- Treat unavailable permissions as gaps, not proof.
- Keep AWS commands read-only unless a later typed lane explicitly records operator-approved mutation authority.

Primary entrypoints:

- `run-access-billing-readbacks.sh` collects bounded readbacks.
- `.csdlc/prepared/issues/485/validate-aws-b-baseline.sh` validates baseline coverage, redaction, and no-mutation posture.
