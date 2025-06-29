-- Add migration script here

ALTER TABLE files ADD COLUMN is_uploaded INTEGER NOT NULL DEFAULT FALSE;