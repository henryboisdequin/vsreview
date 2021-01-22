-- Your SQL goes here: LIKE

create table likes (
    "user" integer references "user" on delete cascade,
    answer integer references answer on delete cascade,
    primary key ("user", answer)
);
