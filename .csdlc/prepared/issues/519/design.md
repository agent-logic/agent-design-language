# TAIL-03 Publication Finalization Design

Issue #519 produces one publication-candidate packet bound to the exact revision reviewed by #518. It validates closing relationships, artifact redaction, and exact-head linkage without merging, tagging, releasing, or changing product behavior.

Execution starts only after #518 has a reviewed merge. Any candidate drift or ambiguous linkage fails closed and returns ownership to the prior stage.
