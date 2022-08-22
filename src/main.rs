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

use self::models::{Post, NewPost, NewPostHandler};
use self::schema::posts;
use self::schema::posts::dsl::*;

use tera::Tera;

#[post("/post/new")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder
{
    println!("{:?}", item);
    let conn =pool.get().expect("Error al traer una conexión");

    let new_post = NewPost{
        title: "wm",
        slug: "kdsadkk",
        body: "dskdlsak"
    };

    match web::block(move ||{Post::create_post(&conn, &item)}).await {
        Ok(data) => 
        {
            HttpResponse::Ok().body(format!("{:?}", data))
        },
        Err(err) => HttpResponse::Ok().body("Error al obtener los valores")
    }
}
#[get("/posts")]
async fn load_posts(pool: web::Data<DbPool>) -> impl Responder
{
    let conn =pool.get().expect("Error al traer una conexión");
    match web::block(move ||{posts.load::<Post>(&conn)}).await {
        Ok(data) => 
        {
            HttpResponse::Ok().body(format!("{:?}", data))
        },
        Err(err) => HttpResponse::Ok().body("Error al obtener los valores")
    }
}

#[get("/")]
async fn index(template_manager: web::Data<tera::Tera>) -> impl Responder
{
    let mut ctx = tera::Context::new();
    HttpResponse::Ok().content_type("text/html")
        .body(template_manager.render("index.html", &ctx).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url =env::var("DATABASE_URL").expect("Error al obtener las variables de entorno");

    let conn = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(conn).expect("Error al construir el pool de conexiones");

    HttpServer::new(move ||{
        let tera =Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .service(index)
            .service(load_posts)
            .service(new_post)
                .data(pool.clone())
                .data(tera)
    }).bind(("0.0.0.0", 9990)).unwrap().run().await
}