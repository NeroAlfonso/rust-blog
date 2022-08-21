#[macro_use] //para el esquema
extern crate diesel;
use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*; //?
pub mod schema;
pub mod models;
fn main() {
    dotenv().ok();
    let db_url : String =env::var("DATABASE_URL").expect("db url variable no encontrada");
    let conn : PgConnection = PgConnection::establish(&db_url).expect("Imposible conectar a postgres");
    use self::models::{Post, NewPost, PostTitleAndSlug};
    use self::schema::posts;
    use self::schema::posts::dsl::*;
    //eliminar por id
    let post_update = diesel::delete(
        posts.filter(id.eq(4))).execute(&conn).expect("Error al eliminar");
    //eliminar por like
    let post_update = diesel::delete(
        posts.filter(slug.like("%publicacion%"))).execute(&conn).expect("Error al eliminar");
    //query completo
    let posts_result =posts.load::<Post>(&conn).expect("error al obtener los datos");
    println!("{}", "query completo");
    for post in posts_result 
    {
        println!("{:?}", post);
    }   
}