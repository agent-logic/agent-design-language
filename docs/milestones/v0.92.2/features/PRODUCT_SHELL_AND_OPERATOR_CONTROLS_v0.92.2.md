# Product Shell and Operator Controls

Status: planned. Owner: CF-SHELL.

Beta 1 must guide setup and onboarding, expose run configuration and status, make artifacts inspectable, and require explicit operator action for publication. Failure and partial states must be visible and recoverable without implying review success.

Acceptance requires a fresh-operator journey, accessible controls, safe credential handling, inspectable configuration, and explicit cancel/retry behavior. Enterprise connectors and autonomous mutation are non-goals.

CF-SHELL follows CF-REVIEW and must let a fresh operator configure, start, inspect, cancel and retry a real review, then open its actual artifacts through the installed product. Mock state, a placeholder UI or a shell design alone cannot close it. CF-UX separately owns the publication approval enforcement consumed by these controls.
