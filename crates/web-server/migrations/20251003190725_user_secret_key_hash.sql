-- Add migration script here

alter table "user" rename column password_hash to secret_key_hash;