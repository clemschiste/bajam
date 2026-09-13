CREATE TABLE heartbeats (
    -- picture_id is a footprint for an image to be inserted in
    -- it is a secret and must be inserted in the picture foreign key
    -- on heartbeat insertion send back picture_id for the upload

    id INTEGER PRIMARY KEY AUTOINCREMENT,
    picture_id TEXT UNIQUE NOT NULL,
    latitude FLOAT NOT NULL,
    longitude FLOAT NOT NULL,
    description TEXT,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
