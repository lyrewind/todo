pub mod schema;
pub mod models;
pub mod db;
pub mod controller;

use std::io;
use std::env;

use controller::*;

use actix_web::{
    HttpServer, App, web::Data
};

use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

const IP: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv()
        .ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Missing 'DATABASE_URL' in envinroment.");
    
    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create database pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(index_tasks)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
    })
    .bind((IP, PORT))?
    .run()
    .await
}
