-- Your SQL goes here: ANSWER

create table answer (
    id serial primary key,
    content varchar(1000) not null,
    question integer references question on delete cascade,
    author integer not null references "user" on delete cascade
);