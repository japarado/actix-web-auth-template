-- Your SQL goes here
CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	email VARCHAR(255) NOT NULL,
	password VARCHAR(255) NOT NULL
)
