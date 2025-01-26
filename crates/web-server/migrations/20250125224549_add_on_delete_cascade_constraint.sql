-- Add migration script here

alter table file drop constraint file_note_id_fkey;
alter table file add constraint file_note_id_fkey foreign key (note_id) references note(id) on delete cascade;

alter table note drop constraint note_block_id_fkey;
alter table note add constraint note_block_id_fkey foreign key (block_id) references sync_block(id) on delete cascade;

alter table note drop constraint note_space_id_fkey;
alter table note add constraint note_space_id_fkey foreign key (space_id) references space(id) on delete cascade;

alter table sync_block drop constraint sync_block_space_id_fkey;
alter table sync_block add constraint sync_block_space_id_fkey foreign key (space_id) references space(id) on delete cascade;

alter table space drop constraint space_user_id_fkey;
alter table space add constraint space_user_id_fkey foreign key (user_id) references "user"(id) on delete cascade;

alter table session drop constraint session_user_id_fkey;
alter table session add constraint session_user_id_fkey foreign key (user_id) references "user"(id) on delete cascade;

alter table used_invitation_token drop constraint used_invitation_token_user_id_fkey;
alter table used_invitation_token add constraint used_invitation_token_user_id_fkey foreign key (user_id) references "user"(id) on delete cascade;
