-- Add migration script here

create table space (
    id uuid primary key,
    data bytea not null,
    checksum bytea not null,
    user_id uuid not null references "user"(id)
);

create table sync_block (
    id uuid primary key,
    number int not null,
    checksum bytea not null,
    space_id uuid not null references space(id)
);

create table note (
    id uuid primary key,
    data bytea not null,
    checksum bytea not null,
    space_id uuid not null references space(id),
    block_id uuid not null references sync_block(id)
);

create table file (
    id uuid primary key,
    data bytea not null,
    note_id uuid not null references note(id)
);
