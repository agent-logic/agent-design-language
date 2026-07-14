# Prepared C-SDLC v2 issue packets

Packets under `issues/` are inert construction artifacts. They are deliberately
outside the canonical `.csdlc/issues/` store, so `csdlc-bind` cannot discover or
bind them and their preview claims confer no lifecycle authority.

Each packet contains all six generated cards, values, digests, audit preview,
and the exact `bootstrap-request.json` used to construct it. Before execution,
the issue's hard prerequisites must be evaluated from current typed evidence.
Only then may an authorized operator initialize the canonical record with:

```text
csdlc-init --root . --request .csdlc/prepared/issues/<issue>/bootstrap-request.json
```

Initialization is not authorization to delete. The execution route must still
fail closed on a false, stale, missing, ambiguous, or early decision. Prepared
claims must be refreshed if they are no longer current when initialization is
authorized.
