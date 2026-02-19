CREATE TABLE IF NOT EXISTS url_analytics (
    id BIGSERIAL PRIMARY KEY,
    url_id BIGINT NOT NULL REFERENCES urls(id) ON DELETE CASCADE,
    user_agent TEXT,
    ip_address INET,
    referrer TEXT,
    country_code VARCHAR(3),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_analytics_url_id ON url_analytics (url_id);
CREATE INDEX IF NOT EXISTS idx_analytics_created_at ON url_analytics (created_at);
