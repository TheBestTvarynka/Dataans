-- Add migration script here

create extension if not exists "uuid-ossp";

create table invitation_token (
    id uuid primary key,
    data bytea not null unique
);

insert into invitation_token (id, data) values ('f47ac10b-58cc-4372-a567-0e02b2c3d479', decode('46adad8fb27f5d208629226bfb75b8de4fe5eab7a7352583ca4a3b6516b87ec3', 'hex'));

create table "user" (
    id uuid primary key,
    username bytea not null unique,
    password bytea not null
);

create table used_invitation_token (
    token_id uuid references invitation_token(id),
    user_id uuid references "user"(id),
    used_at timestamp not null,
    primary key (token_id, user_id)
);

create table session (
    id uuid primary key,
    user_id uuid not null references "user"(id),
    token bytea not null unique,
    created_at timestamp not null,
    expiration_date timestamp not null
);
