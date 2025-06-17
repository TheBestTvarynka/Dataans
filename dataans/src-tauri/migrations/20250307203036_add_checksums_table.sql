-- Add migration script here

CREATE TABLE sync_blocks (
    id BLOB PRIMARY KEY NOT NULL,
    checksum BLOB NOT NULL,
    space_id BLOB NOT NULL REFERENCES spaces(id)
);

ALTER TABLE spaces ADD COLUMN block_id BLOB REFERENCES sync_blocks(id);
ALTER TABLE notes ADD COLUMN block_id BLOB REFERENCES sync_blocks(id);
ALTER TABLE files ADD COLUMN block_id BLOB REFERENCES sync_blocks(id);