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
    use self::models::{Post, NewPost};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    let new_post = NewPost{title: "primera publicación", body: "lelelaloli", slug: "primera-publicacion"};

    let post: Post =diesel::insert_into(posts::table).values(&new_post).get_result(&conn).expect("la inserción falló");

    let posts_result =posts.load::<Post>(&conn).expect("error al obtener los datos");
    for post in posts_result 
    {
        println!("{}", post.title);
    }
}