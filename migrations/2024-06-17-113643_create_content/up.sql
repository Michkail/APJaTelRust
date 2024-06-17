CREATE TABLE content (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    author UUID NOT NULL REFERENCES users(id)
);
