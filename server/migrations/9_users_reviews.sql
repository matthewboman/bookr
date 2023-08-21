CREATE TABLE users_reviews (
    user_id UUID REFERENCES users (user_id) ON UPDATE CASCADE,
    review_id UUID REFERENCES reviews (review_id) ON UPDATE CASCADE ON DELETE CASCADE,

    created_at TIMESTAMP(3) DEFAULT current_timestamp NOT NULL,
    updated_at TIMESTAMP(3) DEFAULT current_timestamp NOT NULL,
    PRIMARY KEY (user_id, review_id)
);

CREATE INDEX idx_user_reviews_user_id ON users_reviews (user_id);
CREATE INDEX idx_user_reviews_review_id ON users_reviews (review_id);
CREATE INDEX idx_reviews_review_id ON reviews (review_id);
