use actix_web::{App, get, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initializing Server
    HttpServer::new(|| {
        App::new()
        .service(index) // Associating service(s)/route_handler(s)
    })
    //Binding socket address server will receice requests on
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
// ----------------------- Route ------------------------
// The only route_handler for this example
#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hey! Welcome to the Actix REST API")
}