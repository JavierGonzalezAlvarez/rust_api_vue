extern crate tokio_pg_mapper_derive;
extern crate tokio_pg_mapper;

use serde::{Deserialize, Serialize};

use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, Deserialize)]
//nombre de la tabla
#[derive(PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct tbusers {
    pub id: i64,
    pub name: String,    
    pub adress: String,    
    pub telephone: String,    
    pub email: Option<String>,
    pub password: String,    
    pub comments: String
}

