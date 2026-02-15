-- Up migration: create_users_table
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "users"
(
    id                   UUID         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    first_name           VARCHAR      NOT NULL,
    last_name            VARCHAR      NOT NULL,
    email                VARCHAR      NOT NULL,
    phone                VARCHAR      NULL,
    password             VARCHAR      NOT NULL,
    password_reset_token VARCHAR      NULL,
    is_superuser         BOOLEAN      NOT NULL DEFAULT FALSE,
    verification_token   UUID         NULL DEFAULT (uuid_generate_v4()),
    last_logged_in_at    TIMESTAMP WITH TIME ZONE          DEFAULT NOW(),
    current_logged_in_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ NULL
);

-- Create a unique constraint on name and deleted_at to ensure uniqueness
CREATE UNIQUE INDEX IF NOT EXISTS uq_user_email_deleted_at ON "users" (email) WHERE deleted_at IS NULL;

-- Create a composite index for name and deleted_at since we query them together
CREATE INDEX IF NOT EXISTS idx_user_email_deleted_at ON "users" (email, deleted_at);

CREATE INDEX IF NOT EXISTS idx_users_id_active
    ON users (id)
    WHERE deleted_at IS NULL;

-- System user:p@ssw0rd!
INSERT INTO "users" (first_name, last_name, email, phone, password, is_superuser)
VALUES ('System', 'UserModel', 'system-user@storagely.io', NULL, '$argon2id$v=19$m=19456,t=2,p=1$fy9EtvooOC8bPAIoTA7gRg$fByeffOVQ2eQFIbCwqDnSLW6Ngq+niqujDZfbyUOiJo', true);
