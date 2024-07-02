use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::Arc;

#[get("/board")]
async fn get_board(data: actix_web::web::Data<Arc<super::Data>>) -> impl Responder {
    HttpResponse::Ok().json(&data.cells)
}

pub async fn start(data: super::Data) -> std::io::Result<()> {
    let data = Arc::new(data);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&data)))
            .service(get_board)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
