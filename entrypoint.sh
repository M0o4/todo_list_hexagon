#!/usr/bin/env bash
set -euo pipefail

: "${DATABASE_URL:?DATABASE_URL is required}"

# Optional flags:
: "${RUN_MIGRATIONS:=true}"      # set to "false" to skip migrations
: "${DB_WAIT_TIMEOUT:=60}"       # seconds
: "${DB_WAIT_INTERVAL:=1}"       # seconds between checks

echo "[entrypoint] waiting for database readiness..."
# pg_isready понимает DATABASE_URL целиком
deadline=$(( $(date +%s) + DB_WAIT_TIMEOUT ))
until pg_isready -d "$DATABASE_URL" -q; do
  if [ "$(date +%s)" -ge "$deadline" ]; then
    echo "[entrypoint] ERROR: database wait timed out (${DB_WAIT_TIMEOUT}s)" >&2
    exit 1
  fi
  sleep "$DB_WAIT_INTERVAL"
done
echo "[entrypoint] database is ready"

if [ "$RUN_MIGRATIONS" = "true" ]; then
  echo "[entrypoint] running migrations..."
  # sqlx возьмёт DATABASE_URL из окружения; cwd = /app, миграции в /app/migrations
  sqlx migrate run
else
  echo "[entrypoint] skipping migrations (RUN_MIGRATIONS=$RUN_MIGRATIONS)"
fi

echo "[entrypoint] starting app: $*"
exec "$@"
