# Web routes — landing

| Path | Auth | Page |
|------|------|------|
| `/` | public | Landing |
| `/login` | public | Magic link |
| `/auth/verify` | public | Verify → `/app` |
| `/app` | Bearer cookie/localStorage | Dashboard |
| `/app/resumes` | protected | Resumes |
| `/app/sessions` | protected | Sessions |
| `/app/billing` | protected | Billing |
| `/resumes`, `/sessions`, `/billing` | — | 302 → `/app/...` |

No API changes.
