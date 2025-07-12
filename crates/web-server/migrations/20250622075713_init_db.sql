-- Add migration script here

create extension if not exists "uuid-ossp";

create table operation (
    id uuid primary key,
    created_at timestamp with time zone not null,
    data bytea not null,
    checksum bytea not null
);
