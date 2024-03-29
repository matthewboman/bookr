create table contacts (
    contact_id serial not null primary key,
    display_name text not null,
    address text,
    city text not null,
    state text,
    zip_code text,
    country text, 
    latitude real,
    longitude real,
    capacity int,
    email text,
    contact_form text,
    age_range text not null,

    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null
);