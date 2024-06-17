CREATE TABLE assets (
    id UUID PRIMARY KEY,
    content_id UUID NOT NULL REFERENCES content(id),
    image_url TEXT NOT NULL
);
