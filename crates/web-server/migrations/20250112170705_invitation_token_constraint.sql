-- Add migration script here

alter table used_invitation_token drop constraint used_invitation_token_pkey;
alter table used_invitation_token add primary key (token_id);