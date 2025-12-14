-- Basic schema

CREATE TABLE IF NOT EXISTS iss_fetch_log (
    id BIGSERIAL PRIMARY KEY,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    source_url TEXT NOT NULL,
    payload JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS telemetry_legacy (
    id BIGSERIAL PRIMARY KEY,
    recorded_at TIMESTAMPTZ NOT NULL,
    voltage NUMERIC(6,2) NOT NULL,
    temp NUMERIC(6,2) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    status_message TEXT,
    source_file TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cms_pages (
    id BIGSERIAL PRIMARY KEY,
    slug TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL
);

-- Seed with deliberately unsafe content for XSS practice
INSERT INTO cms_pages(slug, title, body)
VALUES
('welcome', 'Добро пожаловать', '<h3>Демо контент</h3><p>Этот текст хранится в БД</p>'),
('unsafe', 'Небезопасный пример', '<script>console.log("XSS training")
</script><p>Если вы видите всплывашку значит защита не работает</p>')
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS cms_blocks (
    id BIGSERIAL PRIMARY KEY,
    slug TEXT UNIQUE NOT NULL,
    content TEXT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE
);

INSERT INTO cms_blocks (slug, content) VALUES
('dashboard_experiment', '<div class="alert alert-info">Эксперимент</div>')
ON CONFLICT DO NOTHING;