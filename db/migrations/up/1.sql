CREATE DATABASE cosmas;

 \c cosmas

CREATE TABLE farm (
  id SERIAL PRIMARY KEY,
  is_me BOOLEAN NOT NULL DEFAULT false,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  joined_date TIMESTAMP NOT NULL DEFAULT NOW(),
  address VARCHAR(255),
  display_address BOOLEAN NOT NULL DEFAULT true,
  primary_produce VARCHAR(255),
  website VARCHAR(255)
);