CREATE TABLE IF NOT EXISTS blog_posts
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    content TEXT NOT NULL,
    published_at TEXT NOT NULL
);

INSERT INTO blog_posts (title, slug, content, published_at)
VALUES (
    'Erster Post',
    'erster-post',
    'Das ist der Inhalt des ersten Posts.',
    '2026-03-10'
);
