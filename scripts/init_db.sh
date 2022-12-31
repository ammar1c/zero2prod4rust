#!/usr/bin/env bash
set -x
set -eo pipefail
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_MAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"


if [[ -z "${SKIP_DOCKER}" ]]; then
  docker run \
    -e POSTGRES_USER="${DB_USER}" \
    -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
    -e POSTGRES_DB="${DB_MAME}" \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
fi

# Keep pinging postgres until it's ready
export PGPASSWORD="${DB_PASSWORD}"
until psql -h localhost -U "${DB_USER}" -p ${DB_PORT} -d "postgres" -c '\l'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_MAME}"
sqlx database create
sqlx migrate run
>&2 echo "Postgres is up - executing command"

