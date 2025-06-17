-- Add migration script here

CREATE TABLE operations (
    id BLOB NOT NULL PRIMARY KEY,
    created_at TEXT NOT NULL,
    name TEXT NOT NULL,
    operation TEXT NOT NULL
);