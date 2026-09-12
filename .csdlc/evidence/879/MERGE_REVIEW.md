# PR955 merge-resolution proof

Previous reviewed head: 1cdfbe7312fe80eb00e6c31e8a907db7a3fc9413
Merged main: be159210e0a87fdaca246b932d77af28770689a1

One content conflict in adl/src/cli/codefriend_cmd.rs. Resolution preserves both additive early dispatches: ingest/github and evidence; local/packet parsing and GitHub help remain intact.

Focused proof: codefriend_github_ingestion::installed_command_executes_transport_and_reads_packet_without_credentials passed (1); codefriend_evidence::local_adapter_cli_admission_restart_identity_and_deletion passed (1). Formatting and resolved-path whitespace checks passed. Prior main log trailing-newline warnings were not modified. Original R1/R2 findings and prior proof remain unchanged. Independent exact-head resolution review pending; no push.
