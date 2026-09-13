# Atomic session credits and join-by-ID

**Slug:** rag-answer  
**Story:** 3 of 3

## User story

As an operator, I want to start a session on the web and continue it in the overlay without a second credit charge, and I do not want a session row without a matching debit under concurrency.

## Acceptance criteria

- [ ] GIVEN create session WHEN credits are deducted THEN insert + debit happen in one transaction (no session without debit; no debit without session)
- [ ] GIVEN insufficient credits THEN no session row remains
- [ ] GIVEN an `active` session owned by me WHEN desktop attaches by ID THEN it uses `GET /sessions/{id}` + `POST /sessions/{id}/stt-jwt` and does not call create
- [ ] GIVEN a missing, ended, or other-user session ID THEN overlay shows an error and does not start audio
- [ ] GIVEN attach THEN Redis prewarm may be refreshed but credits are not deducted again

## Existing context

- Paths: `create_session` in `sessions.rs`; `billing::deduct_credits` already transactional internally; `OverlayContext.startSession`; web Sessions “copy session ID”; `refreshSttJwt` already on shared client

## Out of scope

Keychain; magic-link email.

## Risks

TOCTOU between balance check and insert today.

## Definition of done

- [ ] Criteria met
- [ ] Desktop Create or Sessions UI has attach/join field
- [ ] Tests or manual check noted
