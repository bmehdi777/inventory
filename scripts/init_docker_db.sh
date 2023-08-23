#!/usr/bin/env bash
set -x
set -eo pipefail

DB_ROOT_USER=${MONGO_INITDB_ROOT_USERNAME:=mongo}
DB_ROOT_PASSWORD=${MONGO_INITDB_ROOT_PASSWORD:=password}
DB_USER=${MONGO_USER:=mongo}
DB_PASSWORD=${MONGO_PASSWORD:=password}
DB_NAME=${MONGO_INITDB_DATABASE:=inventory}

docker run -p 127.0.0.1:27017:27017 -d --name inventory_db \
	-v "$(pwd)"/scripts/init_mongo.js:/docker-entrypoint-initdb.d/init_mongo.js:ro \
	-e MONGO_INITDB_ROOT_USERNAME=${DB_ROOT_USER} \
	-e MONGO_INITDB_ROOT_PASSWORD=${DB_ROOT_PASSWORD} \
	-e MONGO_INITDB_DATABASE=${DB_NAME} \
	-e MONGO_USER=${DB_USER} \
	-e MONGO_PASSWORD=${DB_PASSWORD} \
	mongo
