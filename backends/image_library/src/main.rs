use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod services;
use services::{create_user_article, fetch_user_articles, fetch_users};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pg_database_url = std::env::var("PG_DATABASE_URL").expect("DATABASE_URL must be set");

    println!("{}", &pg_database_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_database_url)
        .await
        .expect("Error buildinga connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(fetch_user_articles)
            .service(fetch_users)
            .service(create_user_article)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
