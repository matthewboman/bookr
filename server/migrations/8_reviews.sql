CREATE TABLE reviews (
    review_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(256),
    body TEXT,
    rating INT NOT NULL,
    contact_id BIGINT REFERENCES contacts (contact_id) NOT NULL,

    created_at TIMESTAMP(3) DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP(3) DEFAULT current_timestamp NOT NULL
);

CREATE INDEX idx_reviews_contact_id ON reviews (contact_id);