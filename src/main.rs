#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::Post;
use self::schema::posts;
use self::schema::posts::dsl::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder
{
    let conn =pool.get().expect("Error al traer una conexión");
    match web::block(move ||{posts.load::<Post>(&conn)}).await {
        Ok(data) => HttpResponse::Ok().body("Valores conseguidos"),
        Err(err) => HttpResponse::Ok().body("Error al obtener los valores")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url =env::var("DATABASE_URL").expect("Error al obtener las variables de entorno");

    let conn = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(conn).expect("Error al construir el pool de conexiones");

    HttpServer::new(move ||{
        App::new().service(index).data(pool.clone())
    }).bind(("0.0.0.0", 9900)).unwrap().run().await
}