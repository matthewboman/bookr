create table genres (
    genre_id serial not null primary key,
    name text not null,

    created_at timestamp(3) default current_timestamp not null
);

create unique index genres_name_unique on genres (name);

insert into genres (name) values ('alternative/rock');
insert into genres (name) values ('indie');
insert into genres (name) values ('punk/harcore');
insert into genres (name) values ('metal');
insert into genres (name) values ('singer songwriter');
insert into genres (name) values ('jam');
insert into genres (name) values ('country/bluegrass');
insert into genres (name) values ('pop');
insert into genres (name) values ('jazz/blues');
insert into genres (name) values ('rap/hip-hop');