use std::env;
use std::io::Result;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer};
use crate::state::AppState;
use crate::errors::MyError;
use crate::routers::user_routes;

#[path = "../errors.rs"]
mod errors;
#[path = "../state.rs"]
mod state;
#[path = "../routers.rs"]
mod routers;
#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database is NOT set");
    let db_pool = sqlx::MySqlPool::connect(&db_url).await.expect("Connect database error");

    let shared_data = web::Data::new(AppState{
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, req| {
                println!("{:?}, {:?}",err, req);
                MyError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .configure(user_routes)
    };

    println!("Http server start at 127.0.0.1:8989");
    HttpServer::new(app).bind("127.0.0.1:8989")?.run().await
}