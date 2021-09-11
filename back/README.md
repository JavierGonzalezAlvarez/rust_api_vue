# app en rust y vue

## instalar rust en linux
https://www.rust-lang.org/learn/get-started

$ sudo apt update 
$ sudo apt upgrade
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
opcion:1 
$ source $HOME/.cargo/env 

## crear un proyecto en rust
$ cargo new api --bin

## instalar ORM diesel
https://diesel.rs/guides/getting-started

en fichero cargo.toml:
[dependencies]
diesel = { version = "1.4.4", features = ["postgres"] }
dotenv = "0.15.0"

- en caso de tener instalado postgres, sql y mysql
$ cargo install diesel_cli

- si no lo tengo, saldra este error
$ cargo install diesel_cli --no-default-features --features postgres

## crear fichero de entorno env
api/env
$ api/echo DATABASE_URL=postgres://javier:2525_ap@localhost/apirustvue > .env

## crear la base de datos en postgresql o dejar que diesel la cree
$ psql -d postgres -U javier
CREATE DATABASE apirustvue WITH OWNER javier;
DROP DATABASE "apirustvue";

## trabajar con diesel
-crea fichero src\schema.rs y la base de datos si no existe
-crea carpeta migrations
$ diesel setup  

## migraciones
$ diesel migration generate add_users
genera add_users (nombre que le he dado yo a la migracion)

-down
    DROP TABLE users;

- up
    CREATE TABLE tbusers (
        id SERIAL NOT NULL PRIMARY KEY,
        name TEXT NOT NULL,
        adresss TEXT NOT NULL,
        telephone TEXT NOT NULL,
        email TEXT NOT NULL,
        password TEXT NOT NULL,
        comments TEXT NOT NULL,
        created_at TIMESTAMP NOT NULL
    );

-ejecutar schema up
$ diesel migration run
-ejecutar schema down
$ diesel migration redo

## compilar y correr rust
$ cargo run

## framework actix.rs para crear una api
https://actix.rs/docs/getting-started/


## crear modulos
handlers.rs
models.rs

