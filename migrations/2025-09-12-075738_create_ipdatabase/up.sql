-- Your SQL goes here
-- migrations/2025-09-12-000000_create_ip_logs/up.sql
CREATE TABLE ip_logs (
    id SERIAL PRIMARY KEY,
    original_ip TEXT NOT NULL,
    reversed_ip TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
