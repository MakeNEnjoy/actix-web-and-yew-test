mod database;
use std::sync::Mutex;

use database::Database;

use actix_web::{
    middleware::{NormalizePath},
    web::{self, Data},
    App,    
    HttpServer,
    Responder,
};
use serde_json::to_string;
use env_logger;

async fn get_students(data: Data<Mutex<Database>>) -> impl Responder {
    let db = data.lock().unwrap();
    let dummy_query = db.get_first_3_students();

    match to_string(&dummy_query) {
        Ok(json) => json,
        Err(_) => "Failed to serialise query response".to_string()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    HttpServer::new(|| {
        let db = Mutex::new(Database::new());
        App::new()
            .wrap(NormalizePath::trim()) // remove trailing slashes
            .app_data(Data::new(db))
            .route("/students", web::get().to(get_students))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}