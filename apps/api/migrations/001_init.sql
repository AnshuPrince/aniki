-- Enable pgvector extension
CREATE EXTENSION IF NOT EXISTS vector;

-- Users
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    display_name TEXT,
    credits DECIMAL(10, 2) NOT NULL DEFAULT 5.0,
    stripe_customer_id TEXT,
    razorpay_customer_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Magic link tokens (short-lived)
CREATE TABLE magic_link_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_magic_link_token_hash ON magic_link_tokens(token_hash);

-- Resumes
CREATE TABLE resumes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    filename TEXT NOT NULL,
    storage_key TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    chunk_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_resumes_user_id ON resumes(user_id);

-- Resume chunks with embeddings (pgvector)
CREATE TABLE resume_chunks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    resume_id UUID NOT NULL REFERENCES resumes(id) ON DELETE CASCADE,
    chunk_index INT NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_resume_chunks_resume_id ON resume_chunks(resume_id);

-- Sessions
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    resume_id UUID REFERENCES resumes(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'active',
    model TEXT NOT NULL DEFAULT 'gpt41',
    extra_context TEXT,
    enable_screen_ocr BOOLEAN NOT NULL DEFAULT FALSE,
    transcript TEXT,
    notes JSONB,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ
);

CREATE INDEX idx_sessions_user_id ON sessions(user_id);

-- Credit ledger (immutable audit trail)
CREATE TABLE credit_ledger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    delta DECIMAL(10, 2) NOT NULL,
    reason TEXT NOT NULL,
    session_id UUID REFERENCES sessions(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_credit_ledger_user_id ON credit_ledger(user_id);

-- Billing invoices (stub for M5)
CREATE TABLE billing_invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    external_id TEXT,
    amount_cents INT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    pdf_url TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_billing_invoices_user_id ON billing_invoices(user_id);

-- Payment webhook events (idempotency)
CREATE TABLE payment_webhook_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider TEXT NOT NULL,
    event_id TEXT NOT NULL UNIQUE,
    payload JSONB NOT NULL,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
