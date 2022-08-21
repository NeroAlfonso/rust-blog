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
    use self::models::Post;
    use self::schema::posts::dsl::*;
    let posts_result =posts.load::<Post>(&conn).expect("error al obtener los datos");
    for post in posts_result 
    {
        println!("{}", post.title);
    }
}
