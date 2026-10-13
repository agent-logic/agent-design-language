#!/usr/bin/env bash
# Retired source-tree route; this is not a successful validation.
printf '%s\n' 'Runtime source was retired from ADL (#1223). Use the separately qualified Runtime or Infrastructure owner route. Original assertions are preserved in Git and retirement custody; they were not executed by this command.' >&2
return 2 2>/dev/null || exit 2
