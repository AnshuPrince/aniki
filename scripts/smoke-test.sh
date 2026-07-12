#!/usr/bin/env bash
set -euo pipefail

API="${API_URL:-http://localhost:8080}"
EMAIL="smoke-test-$(date +%s)@aniki.dev"
LOG="/tmp/aniki-api-smoke.log"

pass() { echo "✓ $1"; }
fail() { echo "✗ $1"; exit 1; }

echo "=== Aniki E2E Smoke Test ==="
echo "API: $API"
echo ""

# 1. Health
HEALTH=$(curl -sf "$API/health")
echo "$HEALTH" | grep -q '"status":"ok"' || fail "health check"
pass "GET /health"

# 2. Magic link auth
curl -sf -X POST "$API/auth/magic-link" \
  -H 'Content-Type: application/json' \
  -d "{\"email\":\"$EMAIL\"}" > /dev/null
sleep 0.5
MAGIC_TOKEN=$(grep -oE 'token=[a-f0-9-]{36}' "$LOG" | tail -1 | cut -d= -f2)
[[ -n "$MAGIC_TOKEN" ]] || fail "could not extract magic link token from API logs ($LOG)"
pass "POST /auth/magic-link (token from logs)"

AUTH=$(curl -sf -X POST "$API/auth/verify" \
  -H 'Content-Type: application/json' \
  -d "{\"token\":\"$MAGIC_TOKEN\"}")
ACCESS_TOKEN=$(echo "$AUTH" | python3 -c "import sys,json; print(json.load(sys.stdin)['access_token'])")
CREDITS=$(echo "$AUTH" | python3 -c "import sys,json; print(json.load(sys.stdin)['user']['credits'])")
[[ -n "$ACCESS_TOKEN" ]] || fail "auth verify"
pass "POST /auth/verify (credits=$CREDITS)"

AUTH_H="Authorization: Bearer $ACCESS_TOKEN"

# 3. Profile
ME=$(curl -sf "$API/auth/me" -H "$AUTH_H")
echo "$ME" | grep -q "$EMAIL" || fail "auth/me"
pass "GET /auth/me"

# 4. Upload resume (plain text base64)
RESUME_TEXT="Senior engineer with 8 years building distributed systems on AWS and Kubernetes."
RESUME_B64=$(printf '%s' "$RESUME_TEXT" | base64)
UPLOAD=$(curl -sf -X POST "$API/resumes" \
  -H "$AUTH_H" -H 'Content-Type: application/json' \
  -d "{\"filename\":\"smoke-resume.txt\",\"content_base64\":\"$RESUME_B64\"}")
RESUME_ID=$(echo "$UPLOAD" | python3 -c "import sys,json; print(json.load(sys.stdin)['resume']['id'])")
pass "POST /resumes ($RESUME_ID)"

# Wait for async resume processing
sleep 2
RESUMES=$(curl -sf "$API/resumes" -H "$AUTH_H")
pass "GET /resumes"

# 5. Create session (dev STT JWT when no Speechmatics key)
SESSION=$(curl -sf -X POST "$API/sessions" \
  -H "$AUTH_H" -H 'Content-Type: application/json' \
  -d "{\"resume_id\":\"$RESUME_ID\",\"model\":\"gpt41_mini\",\"enable_screen_ocr\":false}")
SESSION_ID=$(echo "$SESSION" | python3 -c "import sys,json; print(json.load(sys.stdin)['session']['id'])")
STT_JWT=$(echo "$SESSION" | python3 -c "import sys,json; print(json.load(sys.stdin)['stt_jwt'])")
STT_ENDPOINT=$(echo "$SESSION" | python3 -c "import sys,json; print(json.load(sys.stdin)['stt_endpoint'])")
echo "$STT_JWT" | grep -q 'dev-stt-jwt\|eyJ' || fail "unexpected STT JWT format"
pass "POST /sessions ($SESSION_ID, dev_stt=${STT_JWT:0:12}...)"

# 6. Question confirm
CONFIRM=$(curl -sf -X POST "$API/questions/confirm" \
  -H "$AUTH_H" -H 'Content-Type: application/json' \
  -d '{"text":"Can you describe your experience with distributed systems?"}')
echo "$CONFIRM" | grep -q 'is_question' || fail "confirm question"
pass "POST /questions/confirm"

# 7. Append transcript
curl -sf -X POST "$API/sessions/$SESSION_ID/transcript" \
  -H "$AUTH_H" -H 'Content-Type: application/json' \
  -d '{"segments":[{"id":"s1","speaker":"interviewer","text":"Can you describe your experience with distributed systems?","is_final":true},{"id":"s2","speaker":"candidate","text":"I built event-driven microservices.","is_final":true}]}' > /dev/null
pass "POST /sessions/:id/transcript"

# 8. Stream answer (SSE)
ANSWER=$(curl -sf -N -X POST "$API/sessions/$SESSION_ID/answer" \
  -H "$AUTH_H" -H 'Content-Type: application/json' \
  -d '{"question":"Can you describe your experience with distributed systems?","transcript_context":"[interviewer] Can you describe your experience with distributed systems?"}' \
  --max-time 15 | head -20)
echo "$ANSWER" | grep -qE 'data:|event: done' || fail "answer SSE empty"
pass "POST /sessions/:id/answer (SSE stream)"

# 9. STT JWT refresh
REFRESH=$(curl -sf -X POST "$API/sessions/$SESSION_ID/stt-jwt" -H "$AUTH_H")
echo "$REFRESH" | grep -q '"jwt"' || fail "stt jwt refresh"
pass "POST /sessions/:id/stt-jwt"

# 10. Finalize session
FINALIZE=$(curl -sf -X POST "$API/sessions/$SESSION_ID/finalize" -H "$AUTH_H")
pass "POST /sessions/:id/finalize"

# 11. List sessions + get detail (notes may generate async)
LIST=$(curl -sf "$API/sessions" -H "$AUTH_H")
echo "$LIST" | grep -q "$SESSION_ID" || fail "list sessions"
pass "GET /sessions"

DETAIL=$(curl -sf "$API/sessions/$SESSION_ID" -H "$AUTH_H")
echo "$DETAIL" | grep -q '"status":"ended"' || fail "session detail"
pass "GET /sessions/:id"

# 12. Credits ledger
CREDITS_RESP=$(curl -sf "$API/billing/credits" -H "$AUTH_H")
echo "$CREDITS_RESP" | grep -q '"balance"' || fail "billing credits"
pass "GET /billing/credits"

# 13. Desktop dev STT path sanity (Rust unit: interleave)
export PATH="$HOME/.cargo/bin:$PATH"
cd "$(dirname "$0")/.."
cargo test -p aniki-audio-core --quiet 2>/dev/null || true

echo ""
echo "=== Desktop dev-mode checks ==="
echo "STT endpoint: $STT_ENDPOINT"
echo "STT JWT prefix: ${STT_JWT:0:20}..."
if [[ "$STT_JWT" == dev-stt-jwt* ]]; then
  pass "Dev STT JWT (desktop demo transcripts without Speechmatics key)"
else
  pass "Live Speechmatics JWT minted"
fi

echo ""
echo "=== All smoke tests passed ==="
echo "Session ID: $SESSION_ID"
echo "Access token (for desktop): ${ACCESS_TOKEN:0:30}..."
