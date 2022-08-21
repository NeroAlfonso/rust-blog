use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello_world() -> impl Responder
{
    HttpResponse::Ok().body("Hola mundo rust")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(hello_world)
    }).bind(("0.0.0.0", 9900)).unwrap().run().await
}