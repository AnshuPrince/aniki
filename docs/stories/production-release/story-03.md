# API/web production gates and resume delete

**Slug:** production-release  
**Story:** 3 of 5

## User story

As a candidate, I want the production API to refuse a half-configured deploy and to let me delete a resume I uploaded so that failed or old resumes do not clutter the dashboard and embeddings cannot run on a guesswork key.

## Acceptance criteria

- [ ] GIVEN Fly (`APP_URL` is not localhost) WHEN `JWT_SECRET` is missing or equals the documented-dev default THEN the process **exits** on boot (do not serve)
- [ ] GIVEN production WHEN Google client id/secret are unset THEN boot fails (OAuth is the only login)
- [ ] GIVEN production WHEN `OPENAI_API_KEY` is unset THEN boot fails (resume embeddings and answers are not optional in prod)
- [ ] GIVEN `DELETE /resumes/{id}` as the owning user THEN resume and `resume_chunks` are removed; response 204
- [ ] GIVEN `DELETE /resumes/{id}` as another user THEN 404 (no existence leak)
- [ ] GIVEN the web resumes UI THEN delete with a confirm copy that RAG context is gone permanently
- [ ] GIVEN a processing error THEN status is `failed` (already in `rag::process_resume`) and the list shows it so delete is usable
- [ ] GIVEN Speechmatics unset THEN STT stub is **not** used in production (fail mint or boot — Solve chooses; do not ship demo transcripts to real interviews)

## Existing context

- Paths: `apps/api/src/config.rs`; `apps/api/src/main.rs`; `apps/api/src/routes/resumes.rs`; `apps/api/src/services/rag.rs`; `apps/web` resumes page
- Pattern: OAuth already 503 when unconfigured; boot-time fail-closed is stricter for Fly
- Migrations: prefer `ON DELETE CASCADE` on `resume_chunks.resume_id` if not already; idempotent SQL

## Out of scope

Retry-failed-resume job (re-upload is enough). Stripe. R2. Logging OpenAI bodies (PR #7 if unmerged is independent).

## Risks

Deleting the only resume mid-session. Mitigation: confirm dialog; sessions keep cached context until next prewarm (document). Boot fail-closed can brick Fly if a secret is accidentally unset — operator uses `fly secrets` + rollback.

## Definition of done

- [ ] Criteria met
- [ ] Tests: owner delete vs other user; config boot guards as unit tests where env can be faked
- [ ] Playbook: production boot checklist (secrets present)
