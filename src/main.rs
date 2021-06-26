use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use dotenv::dotenv;

mod employees;

async fn welcome(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
          .route("/", web::get().to(welcome))
          .route("/{name}", web::get().to(welcome))
          .configure(employees::init_routes)
    })
      // 0.0.0.0は全てのIPアドレスがアクセル可能？
      .bind("0.0.0.0:8000")?
      .run()
      .await
}