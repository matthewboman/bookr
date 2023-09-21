create table genres (
    genre_id serial not null primary key,
    genre_name text not null,

    created_at timestamp(3) default current_timestamp not null
);

create unique index genres_name_unique on genres (genre_name);
create unique index idx_genre_id on genres(genre_id);

insert into genres (genre_name) values ('alternative/rock');
insert into genres (genre_name) values ('indie');
insert into genres (genre_name) values ('punk/hardcore');
insert into genres (genre_name) values ('metal');
insert into genres (genre_name) values ('singer songwriter');
insert into genres (genre_name) values ('jam');
insert into genres (genre_name) values ('country/bluegrass');
insert into genres (genre_name) values ('pop');
insert into genres (genre_name) values ('jazz/blues');
insert into genres (genre_name) values ('rap/hip-hop');