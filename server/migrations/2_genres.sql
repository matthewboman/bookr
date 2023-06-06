create table genres (
    genre_id serial not null primary key,
    name text not null,

    created_at timestamp(3) default current_timestamp not null
);

create unique index genres_name_unique on genres (name);