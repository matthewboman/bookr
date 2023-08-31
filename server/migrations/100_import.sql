INSERT INTO users (user_id, email, password_hash)
VALUES ('42b8a1b3-6e75-4817-9bdd-912a93f40ec2', 'this@wont.work', 'testUuidForContacts');

INSERT INTO contacts (display_name, address, city, state, zip_code, country, latitude, longitude, capacity, email, user_id, age_range)
VALUES ('Static Age', '16 Lexington', 'Asheville', 'NC', '28806', 'USA', 35.5951, -82.5515, 99, 'info@static.com', '42b8a1b3-6e75-4817-9bdd-912a93f40ec2', 'all');

INSERT INTO contacts (display_name, address, city, state, zip_code, country, latitude, longitude, capacity, contact_form, user_id, age_range)
VALUES ('St. Vitus', '1120 Manhattan Ave', 'Brooklyn', 'NY', '11222', 'USA', 40.6782, -73.9442, 200, 'https://www.saintvitusbar.com/about', '42b8a1b3-6e75-4817-9bdd-912a93f40ec2', '21+');