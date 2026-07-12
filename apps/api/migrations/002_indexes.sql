-- HNSW index for resume chunk similarity search
CREATE INDEX IF NOT EXISTS idx_resume_chunks_embedding_hnsw
ON resume_chunks USING hnsw (embedding vector_cosine_ops);

CREATE INDEX IF NOT EXISTS idx_sessions_user_started
ON sessions (user_id, started_at DESC);
