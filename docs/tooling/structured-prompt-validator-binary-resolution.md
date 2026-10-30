# Structured Prompt Validation Boundary

`adl/tools/validate_structured_prompt.sh` and the former
`adl-validate-structured-prompt` binary are deleted v1-era surfaces. They must
not be used by current bootstrap, review, or publication flows, and the wrapper
must not be recreated.

Current card validation is owned by the native C-SDLC v3 executable:

```sh
.adl/bin/native-v3/csdlc validate --request <request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>
```

For semantic card edits, use native v3 `csdlc edit` first, then validate the
resulting typed issue state. New cards should come from the active prompt-template
registry through the native renderer path; do not validate newly generated
work by shelling out to the deleted wrapper.

## Bootstrap Rule

Bootstrap or review-prep code that needs lifecycle validation must create or
load typed C-SDLC issue state and run `csdlc validate`. If the issue is not yet
typed, initialize it through `csdlc issue`; do not call deleted ADL shell
wrappers as a precondition to lifecycle creation.

## Regression Surface

The native v3 command-manifest test checks current guidance for references to
deleted or retained-only routes. It does not rewrite historical milestone
evidence or immutable legacy records.
