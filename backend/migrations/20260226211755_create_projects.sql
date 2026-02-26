CREATE TABLE IF NOT EXISTS projects
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    technologies TEXT NOT NULL
);

INSERT INTO projects (name, description, technologies)
VALUES ('Portfolio Backend', 'Rust/Axum REST API with security focus', 'Rust,Axum,SQLite');