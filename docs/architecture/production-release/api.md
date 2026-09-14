# API, boot, and client contract

## Boot (`ANIKI_ENV`)

| Env | Behavior |
|-----|----------|
| unset / `development` | Today’s defaults (localhost URLs, optional LLM/STT/Google) |
| `production` | `Config::from_env` errors → `main` exits before bind |

Fly: `ANIKI_ENV = "production"` in `fly.toml` `[env]` (not a secret).

## `DELETE /resumes/{id}`

- Auth: existing session JWT (`SessionClaims`).
- SQL: `DELETE FROM resumes WHERE id = $1 AND user_id = $2 RETURNING id`
- 0 rows → `404`
- success → `204 No Content`
- CORS: `DELETE` already allowed in `main.rs`

Shared client: `deleteResume(id: string): Promise<void>` (`fetch` 204).

Web: confirm dialog — “This removes the file and its answer context. Sessions keep a snapshot until they end.” Then `DELETE`.

## `GET /desktop/latest`

- Auth: session JWT. No token → 401.
- Server: `GET https://api.github.com/repos/AnshuPrince/aniki/releases/latest` (optional `GITHUB_TOKEN` env on Fly for rate limit).
- 200 example: `{ "version": "0.1.0", "macos_dmg": "https://...", "windows_exe": "https://..." }` (nulls if asset missing).
- 404 if no release yet.
- Shared client: `desktopLatest()` only used when `useAuth().user` is set.

## Overlay commands (Tauri)

| Command | Effect |
|---------|--------|
| `credential_get` | `Option<String>` from keyring |
| `credential_set` | store JWT |
| `credential_delete` | wipe JWT |

Renderer never writes `localStorage` for tokens after this slice.

