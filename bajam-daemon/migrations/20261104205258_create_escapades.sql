CREATE TABLE escapades (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE escapades_map (
    escapade_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    role TEXT NOT NULL DEFAULT 'member',

    PRIMARY KEY (escapade_id, user_id),
    
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (escapade_id)
        REFERENCES escapades(id)
        ON DELETE CASCADE
);
