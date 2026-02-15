-- Up migration: create_states_table
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "states"
(
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    country_id  UUID REFERENCES countries(id) ON DELETE RESTRICT NOT NULL,
    name        TEXT NOT NULL,
    code        TEXT NOT NULL, -- e.g. CA, NY, TX

    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =========================
-- Indexes
-- =========================

CREATE INDEX IF NOT EXISTS states_country_id_index
    ON states (country_id);

CREATE INDEX IF NOT EXISTS states_name_index
    ON states (name);

CREATE UNIQUE INDEX IF NOT EXISTS states_country_code_uindex  ON states (country_id, code);


-- =========================
-- Seeders
-- =========================
WITH us AS (
    SELECT id AS country_id FROM countries WHERE iso3_code = 'USA' LIMIT 1
)
INSERT INTO states (country_id, name, code)
SELECT us.country_id, s.name, s.code
FROM us CROSS JOIN (VALUES
    ('Alabama', 'AL'),
    ('Alaska', 'AK'),
    ('Arizona', 'AZ'),
    ('Arkansas', 'AR'),
    ('California', 'CA'),
    ('Colorado', 'CO'),
    ('Connecticut', 'CT'),
    ('Delaware', 'DE'),
    ('Florida', 'FL'),
    ('Georgia', 'GA'),
    ('Hawaii', 'HI'),
    ('Idaho', 'ID'),
    ('Illinois', 'IL'),
    ('Indiana', 'IN'),
    ('Iowa', 'IA'),
    ('Kansas', 'KS'),
    ('Kentucky', 'KY'),
    ('Louisiana', 'LA'),
    ('Maine', 'ME'),
    ('Maryland', 'MD'),
    ('Massachusetts', 'MA'),
    ('Michigan', 'MI'),
    ('Minnesota', 'MN'),
    ('Mississippi', 'MS'),
    ('Missouri', 'MO'),
    ('Montana', 'MT'),
    ('Nebraska', 'NE'),
    ('Nevada', 'NV'),
    ('New Hampshire', 'NH'),
    ('New Jersey', 'NJ'),
    ('New Mexico', 'NM'),
    ('New York', 'NY'),
    ('North Carolina', 'NC'),
    ('North Dakota', 'ND'),
    ('Ohio', 'OH'),
    ('Oklahoma', 'OK'),
    ('Oregon', 'OR'),
    ('Pennsylvania', 'PA'),
    ('Rhode Island', 'RI'),
    ('South Carolina', 'SC'),
    ('South Dakota', 'SD'),
    ('Tennessee', 'TN'),
    ('Texas', 'TX'),
    ('Utah', 'UT'),
    ('Vermont', 'VT'),
    ('Virginia', 'VA'),
    ('Washington', 'WA'),
    ('West Virginia', 'WV'),
    ('Wisconsin', 'WI'),
    ('Wyoming', 'WY')
) AS s(name, code)
ON CONFLICT DO NOTHING;

WITH ca AS (
    SELECT id AS country_id FROM countries WHERE iso3_code = 'CAN' LIMIT 1
)
INSERT INTO states (country_id, name, code)
SELECT ca.country_id,  s.name, s.code
FROM ca CROSS JOIN (VALUES
        ('Alberta', 'AB'),
        ('British Columbia', 'BC'),
        ('Manitoba', 'MB'),
        ('New Brunswick', 'NB'),
        ('Newfoundland and Labrador', 'NL'),
        ('Nova Scotia', 'NS'),
        ('Ontario', 'ON'),
        ('Prince Edward Island', 'PE'),
        ('Quebec', 'QC'),
        ('Saskatchewan', 'SK'),
        ('Northwest Territories', 'NT'),
        ('Nunavut', 'NU'),
        ('Yukon', 'YT')
) AS s(name, code) ON CONFLICT DO NOTHING;
