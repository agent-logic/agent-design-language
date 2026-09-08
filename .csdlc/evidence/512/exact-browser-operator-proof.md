# Issue #512 operator browser proof

- Source: operator-provided browser screenshot in the issue #512 review session
- Product revision: `f431621f9dd246a918f295020cd53e3c3e265a9c`
- Publication-metadata head at capture review: `3b12d2a0cf55e9b180ee3f9b90025e2e2c2e72fb`
- URL visible in capture: `http://localhost:8000/demos/html-observatory/?runtime=v3&runtimeApiBase=https://wuji.dev.csm.agent-logic.ai:20997&live=1&_cb=final#runtime-proof`
- Image: `exact-browser-operator-screenshot.png`
- Image SHA-256: `32ec4493bfbd19bda8df967965b362a3236f2f25967b045b900cbcb3b308cafa`

The capture visibly demonstrates the redesigned responsive shell, the
`axioma-wuji` polis selection, six live agents, Runtime readiness, the live event
stream, an active inspector tab, Runtime v3 provenance, and a connected WSS
status. The already-retained focused test suite supplies executable coverage for
hash navigation, ARIA relationships, roving tab focus, keyboard key handling,
responsive breakpoints, recovery, and redaction.

The screenshot does not by itself prove every keyboard gesture. Codex attempted
both available bundled browser-controller entrypoints, but each stopped before
page interaction because the controller's own trusted RPC dependency could not
resolve within its configured trusted-code roots. No Observatory failure is
inferred from that controller initialization error.
