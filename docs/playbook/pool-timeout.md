# Database pool timeout

Symptom: API logs `pool timed out while waiting for an open connection`.

1. `curl -s localhost:8080/health` — check `database` and `db_pool` (`size` vs `idle`).
2. `docker compose ps` — Postgres must be `healthy`.
3. Restart the API if Postgres restarted underneath it.
4. Confirm `DATABASE_URL`. Optional: raise `DATABASE_MAX_CONNECTIONS`.

See also root `README.md` Troubleshooting.
