CREATE TABLE pictures (
    -- upload_id TEXT PRIMARY KEY
    -- on upload/ use the picture id for insertion, rename it with the id ? (1.jpg, 2.jpg etc ?)
    -- the idea is to be able to delete the picture on demand...
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    picture_id TEXT NOT NULL,
    FOREIGN KEY (picture_id) REFERENCES heartbeats(picture_id)
);
