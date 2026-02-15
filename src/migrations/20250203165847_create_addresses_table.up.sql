-- Up migration: create_states_table
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS addresses
(
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- Raw & canonical forms
    address_raw          TEXT NOT NULL,
    street_address       TEXT,
    -- Geospatial
    latitude             DECIMAL(11, 8),
    longitude            DECIMAL(11, 8),
    -- Street decomposition
    route_number         TEXT,
    route_prefix         TEXT,
    route                TEXT,
    route_type           TEXT,
    route_suffix         TEXT,
    unit_type            TEXT,
    unit_number          TEXT,
    intersection         TEXT,
    -- Administrative hierarchy
    country_id           UUID REFERENCES countries(id) ON DELETE RESTRICT NOT NULL,
    state_id             UUID REFERENCES states(id) ON DELETE RESTRICT NULL,
    county               TEXT,
    locality             TEXT,
    sublocality          TEXT,
    subdivision          TEXT,
    neighborhood         TEXT,
    school_district      TEXT,
    zip                  TEXT,

    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ NULL
);

CREATE INDEX IF NOT EXISTS idx_addresses_id_active
    ON addresses (id)
    WHERE deleted_at IS NULL;

CREATE UNIQUE INDEX ux_addresses_lookup_active
    ON addresses (address_raw, locality, zip, country_id)
    WHERE deleted_at IS NULL;
