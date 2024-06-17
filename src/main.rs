#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use thoth::admin::AdminApp;
use thoth::models::{AdminConfig, AdminContext};
use thoth::prelude::*;

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let admin_config = AdminConfig::default();
    let admin_context = AdminContext::new(pool.clone(), admin_config);

    HttpServer::new(move || {
        let admin_app = AdminApp::new(admin_context.clone());
        App::new()
            .data(pool.clone())
            .service(web::scope("/admin").configure(admin_app.configure()))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
