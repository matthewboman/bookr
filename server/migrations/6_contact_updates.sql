ALTER TABLE contacts ADD COLUMN verified BOOLEAN DEFAULT FALSE NOT NULL;
ALTER TABLE contacts ADD COLUMN is_private BOOLEAN DEFAULT FALSE NOT NULL;
ALTER TABLE contacts ADD COLUMN user_id UUID REFERENCES users (user_id);

-- Existing contacts are verified
UPDATE contacts SET verified = TRUE;