-- users table
CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    date_of_birth DATE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

-- tickets table
CREATE TABLE tickets (
    ticket_id UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    place_id UUID NOT NULL
);

-- places table
CREATE TABLE places (
    place_id UUID PRIMARY KEY NOT NULL,
    house_number INTEGER NOT NULL,
    water BOOLEAN NOT NULL DEFAULT FALSE,
    gas BOOLEAN NOT NULL DEFAULT FALSE,
    light BOOLEAN NOT NULL DEFAULT FALSE,
    in_shadow BOOLEAN NOT NULL DEFAULT FALSE
);

-- license plate table
CREATE TABLE license_plates (
    plate_id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    license_plate TEXT NOT NULL UNIQUE
);

-- create cards table
CREATE TABLE cards (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    card_id TEXT NOT NULL UNIQUE
);

-- activities table
CREATE TABLE activities (
    activity_id UUID PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    max_capacity INT NOT NULL
);

-- activity_signups table
CREATE TABLE activity_signups (
    activity_id UUID NOT NULL REFERENCES activities(activity_id),
    user_id UUID NOT NULL REFERENCES users(user_id),
    signup_time TIMESTAMP NOT NULL,
    PRIMARY KEY (activity_id, user_id)
);