-- Up migration: create_tax_tables
CREATE TABLE IF NOT EXISTS "tax_documents"
(
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id        UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    year           INT NOT NULL,
    document_type  VARCHAR(50) NOT NULL, -- 'W2', '1099', etc.
    file_name      VARCHAR(255) NOT NULL,
    file_path      VARCHAR(255) NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at     TIMESTAMPTZ NULL
);

CREATE TABLE IF NOT EXISTS "tax_data"
(
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id        UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    year           INT NOT NULL,
    data           JSONB NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at     TIMESTAMPTZ NULL,
    UNIQUE(user_id, year)
);

CREATE INDEX IF NOT EXISTS idx_tax_documents_user_id ON tax_documents(user_id);
CREATE INDEX IF NOT EXISTS idx_tax_data_user_id ON tax_data(user_id);
