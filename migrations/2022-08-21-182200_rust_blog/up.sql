-- Your SQL goes here
CREATE TABLE posts
(
    id serial primary key,
    title varchar not null,
    slug varchar not null,
    body text not null
)