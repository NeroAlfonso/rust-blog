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

    //let new_post = NewPost{title: "primera publicación", body: "lelelaloli", slug: "primera-publicacion"};

    //let post: Post =diesel::insert_into(posts::table).values(&new_post).get_result(&conn).expect("la inserción falló");

    //Un solo atributo
    /* let post_update = diesel::update(
        posts.filter(id.eq(4)))
            .set(slug.eq("lexlaloxx"))
    .get_result::<Post>(&conn).expect("Error al actualizar");*/
    //Multiples atributos
    let post_update = diesel::update(
        posts.filter(id.eq(4)))
            .set((slug.eq("lexlaloxx2"), body.eq("cuerpotx 2")))
    .get_result::<Post>(&conn).expect("Error al actualizar");

    //query completo
    let posts_result =posts.load::<Post>(&conn).expect("error al obtener los datos");
    println!("{}", "query completo");
    for post in posts_result 
    {
        println!("{:?}", post);
    }
    
}