-- Your SQL goes here
CREATE TABLE tbusers (
        id SERIAL NOT NULL PRIMARY KEY,
        name TEXT NOT NULL,
        adress TEXT NOT NULL,
        telephone TEXT NOT NULL,
        email TEXT NOT NULL,
        password TEXT NOT NULL,
        comments TEXT NOT NULL,
        created_at TIMESTAMP NOT NULL
    );