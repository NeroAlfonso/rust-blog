#[macro_use] //para el esquema
extern crate diesel;
use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*; //?
fn main() {
    dotenv().ok();
    let db_url : String =env::var("DATABASE_URL").expect("db url variable no encontrada");
    PgConnection::establish(&db_url).expect("Imposible conectar a postgres");
    println!("{}", db_url);
    println!("Hello, world!");
}
