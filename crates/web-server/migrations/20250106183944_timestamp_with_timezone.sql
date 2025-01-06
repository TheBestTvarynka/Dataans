-- Add migration script here

alter table used_invitation_token alter column used_at type timestamp with time zone;
alter table session alter column created_at type timestamp with time zone;
alter table session alter column expiration_date type timestamp with time zone;