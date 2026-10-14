#!/usr/bin/env bash
# Retired source-tree contract; not a successful validation.
printf '%s\n' 'Infrastructure source was retired from ADL (#1223). Run this contract from the separately reviewed agent-logic-infrastructure owner checkout. Original assertions remain in Git and retirement custody; this command did not execute them.' >&2
return 2 2>/dev/null || exit 2
