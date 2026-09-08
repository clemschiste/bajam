CREATE TABLE heartbeats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    latitude FLOAT NOT NULL,
    longitude FLOAT NOT NULL,
    description TEXT,
    timestamp TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    upload_id TEXT NULL,
    FOREIGN KEY (upload_id) REFERENCES photos(upload_id)
);
