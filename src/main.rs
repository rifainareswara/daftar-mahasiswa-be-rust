use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod api;
mod db;
mod error;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let db = db::connect_db(&database_url).await.expect("Failed to connect to MongoDB");

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db.clone()))
            .configure(api::config)
    })
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}