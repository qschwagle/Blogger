/* REQUIRED FOR gen_random_uuid(), CREATE EXTENSION IF NOT EXISTS pgcrypto; */


CREATE TABLE IF NOT EXISTS Users (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    new_email TEXT,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    passhash TEXT NOT NULL,
    confirmed bool NOT NULL DEFAULT false,
    PRIMARY KEY (id)
);
/* vim:set sw=4 ts=4 et: */
