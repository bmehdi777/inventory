#!/usr/bin/env bash

set -x
set -eo pipefail

RUNNING_CONTAINER=$(docker ps --filter 'name=redis' --format '{{.ID}}')
if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "A redis instance is already running, use the following to kill it."
  echo >&2 "    docker kill ${RUNNING_CONTAINER}"
  exit 1
fi

docker run \
    -p "6379:6379" \
    -d \
    --name "redis_inventory" \
    redis:7

