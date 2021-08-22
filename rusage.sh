#!/usr/bin/env bash

# https://stackoverflow.com/questions/1527049/how-can-i-join-elements-of-an-array-in-bash/53050617

function join_by { local d=${1-} f=${2-}; if shift 2; then printf %s "$f" "${@/#/$d}"; fi; }

exec crusage "$(join_by ' ' $@)"
