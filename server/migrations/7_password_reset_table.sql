CREATE TABLE reset_tokens(
    reset_token TEXT NOT NULL,
    user_id uuid NOT NULL REFERENCES users (user_id),
    PRIMARY KEY (reset_token),
    created_at TIMESTAMP(3) DEFAULT current_timestamp NOT NULL
)