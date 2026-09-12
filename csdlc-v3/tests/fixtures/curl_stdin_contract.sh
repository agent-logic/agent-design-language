#!/bin/sh
IFS= read -r config
test "$config" = 'header = "Authorization: Bearer synthetic-sim01-secret"' || exit 11
test "$GITHUB_TOKEN" = 'synthetic-sim01-secret' || exit 12
test -z "${HOME+x}${HTTPS_PROXY+x}${GH_TOKEN+x}" || exit 13
test "$LC_ALL" = C || exit 14
test "$1" = -q && test "$2" = --config && test "$3" = - && test "$#" = 3 || exit 15
printf 'stdin-and-scope-verified'
