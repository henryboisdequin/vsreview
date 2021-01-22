-- Your SQL goes here: USER

create table "user" (
    id serial primary key,
    username text not null unique,
    email text not null unique,
    bio text not null,
    profile_image text,
    password_hash text not null
);