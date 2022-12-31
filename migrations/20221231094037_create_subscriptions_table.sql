-- Add migration script here
create table subscriptions (
    id uuid NOT NULL,
    primary key (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
);
