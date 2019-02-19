-- Your SQL goes here
CREATE TABLE item_groups (
    layer INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR UNIQUE NOT NULL,
    description TEXT
)