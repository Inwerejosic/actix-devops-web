// #[allow!(unused_imports)]
extern crate diesel;
extern crate diesel_migrations;

mod db;
mod model;
mod schema;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use tera::{ Tera, Context};
use diesel::prelude::*;
// use chrono::Utc;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use db::DbPool;
use model::{IpLog, NewIpLog};
use schema::ip_logs::dsl::*;
use tracing_subscriber;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

async fn index(
    req: HttpRequest,
    tmpl: web::Data<tera::Tera>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn_info = req.connection_info();
    let ip = conn_info
        .realip_remote_addr()
        .unwrap_or("unknown remote address")
        .to_string();
    let reversed = if ip.contains('.') {
        ip.split('.').rev().collect::<Vec<&str>>().join(".")
    } else {
        ip.chars().rev().collect::<String>()
    };

    {
        let mut conn = pool.get().expect("Failed to get DB connection for migrations");
        conn.run_pending_migrations(MIGRATIONS).expect("Migrations failed");
    }

    {
        let mut conn = pool.get().expect("Failed to get DB connection");
        let new_log = NewIpLog {
            original_ip: &ip,
            reversed_ip: &reversed,
        };
        diesel::insert_into(ip_logs)
            .values(&new_log)
            .execute(&mut conn)
            .expect("Insert failed");
    }


    // Fetch latest 20 logs
    let logs_data: Vec<IpLog> = {
        let mut conn = pool.get().expect("Failed to get DB connection");
        ip_logs
            .order(id.desc())
            .limit(20)
            .load::<IpLog>(&mut conn)
            .expect("Query failed")
    };

    let mut ctx = Context::new();
    ctx.insert("original_ip", &ip);
    ctx.insert("reversed_ip", &reversed);
    ctx.insert("logs", &logs_data);


    let s = tmpl.render("index.html.tera", &ctx).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Template error: {}", e))
    })?;


    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();


    let pool = db::establish_pool();
    let tera = Tera::new("templates/**/*").expect("Failed to parse templates");


    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let bind = format!("{}:{}", host, port);


    println!("Listening on http://{}", bind);


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(index))
    })
        .bind(bind)?
        .run()
        .await
}