-- Add migration script here

ALTER TABLE notes DROP COLUMN block_id;
ALTER TABLE spaces DROP COLUMN checksum;
ALTER TABLE notes DROP COLUMN checksum;
ALTER TABLE files DROP COLUMN checksum;
DROP TABLE IF EXISTS sync_blocks;