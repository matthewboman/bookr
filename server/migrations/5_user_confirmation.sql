ALTER TABLE users ADD COLUMN status TEXT DEFAULT 'pending';
ALTER TABLE users ADD COLUMN role TEXT DEFAULT 'standard';
ALTER TABLE users ADD COLUMN created_at timestamp(3) DEFAULT current_timestamp NOT NULL;

CREATE TABLE confirmation_tokens(
    confirmation_token TEXT NOT NULL,
    user_id uuid NOT NULL REFERENCES users (user_id),
    PRIMARY KEY (confirmation_token)
);