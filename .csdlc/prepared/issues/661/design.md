# Issue #661 design

The resident Shepherd already owns a configured provider/model after #640, but
the conversation executor special-cases `shepherd` and returns a synthetic
acknowledgement. Remove that special case and route Shepherd work through the
same governed provider-backed execution boundary used by other resident agents.

The reply envelope remains `conversation_reply.v1`, with existing recipient,
conversation, work, and attribution fields. Provider errors remain errors; no
synthetic success is permitted. Agent-to-agent initiation is intentionally
outside this issue.
