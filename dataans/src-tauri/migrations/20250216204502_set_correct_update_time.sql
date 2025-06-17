-- Add migration script here

UPDATE spaces set updated_at = created_at;
UPDATE notes set updated_at = created_at;