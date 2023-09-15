CREATE TABLE reviews (
    review_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(256) NOT NULL,
    body TEXT NOT NULL,
    rating INT NOT NULL,
    contact_id INT REFERENCES contacts (contact_id) NOT NULL,
    user_id UUID REFERENCES users (user_id) NOT NULL,

    created_at TIMESTAMP(3) DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP(3) DEFAULT current_timestamp NOT NULL
);

CREATE INDEX idx_reviews_contact_id ON reviews (contact_id);
CREATE INDEX idx_reviews_user_id on reviews (user_id);
CREATE INDEX idx_reviews_created ON reviews (date(created_at));
CREATE INDEX idx_reviews_updated ON reviews (date(updated_at));