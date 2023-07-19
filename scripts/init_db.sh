#!/usr/bin/env bash
set -x
set -eo pipefail

DB_USER=${MONGO_INITDB_ROOT_USERNAME:=mongo}
DB_PASSWORD=${MONGO_INITDB_ROOT_PASSWORD:=password}

docker run -p 127.0.0.1:27017:27017 -d --name inventory_db \
	-e MONGO_INITDB_ROOT_USERNAME=${DB_USER} \
	-e MONGO_INITDB_ROOT_PASSWORD=${DB_PASSWORD} \
	mongo
