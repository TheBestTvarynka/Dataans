-- Add migration script here

ALTER TABLE spaces DROP COLUMN block_id;
ALTER TABLE files DROP COLUMN block_id;