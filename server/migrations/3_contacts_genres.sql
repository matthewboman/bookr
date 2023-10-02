create table contacts_genres (
    primary key (contact_id, genre_id),
    contact_id int references contacts (contact_id) on update cascade on delete cascade,
    genre_id int references genres (genre_id) on update cascade on delete cascade,
    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null
);