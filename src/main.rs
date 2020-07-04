// Necessary imports here 
use actix_web::{App, get, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {    
    // Initializing server
    HttpServer::new(|| {
        App::new()
        // Associating service(s)/route_handler(s)
         .service(index)
    })
    // Binding socket address server will receive requests on
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
// ----------------------- Route ------------------------
// The only route_handler for this example
#[get("/students")]
async fn find_all() -> HttpResponse {
    HttpResponse::Ok().body("List of students")
}