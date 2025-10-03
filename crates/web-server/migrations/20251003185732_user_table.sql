-- Add migration script here

create table "user" (
    id uuid primary key,
    password_hash text not null
);
