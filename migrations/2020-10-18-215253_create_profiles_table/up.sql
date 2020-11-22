-- Your SQL goes here
CREATE TABLE profiles (
	id SERIAL PRIMARY KEY,
	username VARCHAR(255) NOT NULL,
	bio TEXT,
	profile_picture VARCHAR(255),
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE
);
