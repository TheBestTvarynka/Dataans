-- Add migration script here

CREATE TABLE IF NOT EXISTS spaces(
  id BLOB NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  avatar_id TEXT NOT NULL REFERENCES files(id),
  created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS notes(
  id BLOB NOT NULL PRIMARY KEY,
  text TEXT NOT NULL,
  created_at TEXT NOT NULL,
  space_id BLOB NOT NULL REFERENCES spaces(id)
);

CREATE TABLE IF NOT EXISTS notes_files(
  note_id BLOB NOT NULL REFERENCES notes(id),
  file_id BLOB NOT NULL REFERENCES files(id),
  PRIMARY KEY(note_id, file_id)
);

CREATE TABLE IF NOT EXISTS files(
  id BLOB NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  path TEXT NOT NULL
);
