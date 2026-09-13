#!/bin/sh
set -e

until pg_isready -h "$PGHOST" -U "$PGUSER" -d "$PGDATABASE"; do
  echo "Waiting for postgres..."
  sleep 1
done

echo "Applying migrations..."
psql -v ON_ERROR_STOP=1 -f /migrations/001_init.sql
psql -v ON_ERROR_STOP=1 -f /migrations/002_indexes.sql
psql -v ON_ERROR_STOP=1 -f /migrations/003_google_sub.sql
echo "Done."
