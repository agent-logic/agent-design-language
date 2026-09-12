#!/bin/sh
cat >/dev/null
printf '%s' "$GITHUB_TOKEN"
printf '%s' "$GITHUB_TOKEN" >&2
