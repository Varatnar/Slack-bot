CREATE TABLE items (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR UNIQUE NOT NULL,
    description TEXT,
    layer INTEGER NOT NULL,
        FOREIGN KEY (layer) REFERENCES item_groups(layer)
        ON UPDATE CASCADE
        ON DELETE CASCADE
)