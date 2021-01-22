-- Your SQL goes here: QUESTION

create table question (
    id serial primary key,
    title varchar(80) not null,
    content varchar(1000) not null,
    tag_list text[] not null,
    author integer not null references "user" on delete cascade
);
