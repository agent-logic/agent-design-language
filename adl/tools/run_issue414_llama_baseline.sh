#!/usr/bin/env bash
# Retired ADL source entrypoint; no original model or restore assertions execute.
printf '%s\n' 'The ADL issue414 baseline route was retired by #1223. Use the reviewed agent-logic-infrastructure continuity contract and explicitly qualified Runtime artifacts. Original llama/model, CSM and restore assertions remain in Git and retirement custody; their live acceptance is not claimed.' >&2
return 2 2>/dev/null || exit 2
