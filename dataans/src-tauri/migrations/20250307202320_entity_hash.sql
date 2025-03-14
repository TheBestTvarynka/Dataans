-- Add migration script here

ALTER TABLE spaces DROP COLUMN is_synced;
ALTER TABLE spaces ADD COLUMN checksum BLOB;

ALTER TABLE notes DROP COLUMN is_synced;
ALTER TABLE notes ADD COLUMN checksum BLOB;

ALTER TABLE files DROP COLUMN is_synced;
ALTER TABLE files ADD COLUMN checksum BLOB;