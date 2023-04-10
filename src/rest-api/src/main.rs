use actix_web::{get, App, HttpResponse, HttpServer, Responder};


#[get("/")]
async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello.")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_hello)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
