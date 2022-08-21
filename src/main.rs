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

    //query completo
    let posts_result =posts.load::<Post>(&conn).expect("error al obtener los datos");
    println!("{}", "query completo");
    for post in posts_result 
    {
        println!("{}", post.title);
    }
    //query limitado
    let posts_result =posts.limit(1).load::<Post>(&conn).expect("error al obtener los datos");
    println!("{}", "query limitado");
    for post in posts_result 
    {
        println!("{}", post.title);
    }
    //query limitado con atributos
    let posts_result =posts.select((title, slug)).limit(1).load::<PostTitleAndSlug>(&conn).expect("error al obtener los datos");
    println!("{}", "query limitado con atributos");
    for post in posts_result 
    {
        println!("{:?}", post);
    }
    //query ilimitado con orden
    let posts_result =posts.order(id.desc()).load::<Post>(&conn).expect("error al obtener los datos");
    println!("{}", "query ilimitado con orden");
    for post in posts_result 
    {
        println!("{}", post.title);
    }
    //query ilimitado con condición
    let posts_result =posts.filter(id.eq(2)).load::<Post>(&conn).expect("error al obtener los datos");
    println!("{}", "query ilimitado con condición");
    for post in posts_result 
    {
        println!("{}", post.title);
    }
}