-- Add migration script here

create extension if not exists "uuid-ossp";

create table invitation_token (
    id uuid primary key,
    data bytea not null unique
);

create table "user" (
    id uuid primary key,
    username bytea not null unique,
    password bytea not null
);

create table used_invitation_token (
    token_id uuid references invitation_token(id),
    user_id uuid references "user"(id) on delete cascade,
    used_at timestamp with time zone not null,
    primary key (token_id)
);

create table session (
    id uuid primary key,
    user_id uuid not null references "user"(id) on delete cascade,
    created_at timestamp with time zone not null,
    expiration_date timestamp with time zone not null
);

create table operation (
    id uuid primary key,
    created_at timestamp with time zone not null,
    data bytea not null,
    checksum bytea not null
);
