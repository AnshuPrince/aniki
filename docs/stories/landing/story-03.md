# CTAs, footer, signed-in skip

**Slug:** landing  
**Story:** 3 of 3

## User story

As a visitor or returning user, I want obvious next steps and a footer so that the page feels finished and `APP_URL` can be the public origin for magic links.

## Acceptance criteria

- [ ] GIVEN Get started / Sign in THEN `/login`
- [ ] GIVEN signed-in user on any marketing URL THEN redirect `/app`
- [ ] GIVEN footer THEN Product, Privacy, Pricing (anchors), Sign in; MIT/license or “personal project” — no fake company legal if we have none
- [ ] GIVEN magic link `APP_URL` THEN still `/auth/verify`; after verify → `/app`
- [ ] GIVEN Pages deploy THEN `/` is the landing (no auth cookie required)

## Existing context

- `VerifyPage`; ci-cd `APP_URL` = Pages origin

## Out of scope

Analytics pixels (needs Solve if added). Cookie banner only if we add trackers — v1: none.

## Risks

Adding analytics without a threat note. Do not add in this epic.

## Definition of done

- [ ] Criteria met
- [ ] No third-party tracker on the landing
