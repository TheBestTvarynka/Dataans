-- Add migration script here

drop table if exists "file";
drop table if exists note;
drop table if exists sync_block;
drop table if exists space;

create table operation (
    id uuid primary key,
    created_at timestamp with time zone not null,
    data bytea not null,
    checksum bytea not null
);