# API sketch — oauth-login

All routes **public** (no Bearer). CORS already allows `APP_URL`.

## `GET /auth/oauth/google/start`

Response `200`:

```json
{ "authorization_url": "https://accounts.google.com/o/oauth2/v2/auth?..." }
```

`404`/`503` if Google env unset. Do not log the URL with the verifier.

## `POST /auth/oauth/google/callback`

Body:

```json
{ "code": "string", "state": "string" }
```

Response `200`: same as `POST /auth/verify` (`AuthResponse`).

Errors:

| Status | When |
|--------|------|
| 401 | bad/expired `state`, token exchange fail, `email_verified` false |
| 400 | missing fields |
| 503 | Google env unset |

## Unchanged

`POST /auth/magic-link`, `POST /auth/verify`, `GET /auth/me`.

## Shared TS

```ts
oauthGoogleStart(): Promise<{ authorization_url: string }>
oauthGoogleCallback(code: string, state: string): Promise<AuthResponse>
```

Web: `window.location.assign(authorization_url)`. Callback page reads `code`/`state` from the query, POSTs, `setToken`, `navigate('/app')`.
