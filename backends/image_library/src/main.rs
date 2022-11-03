use actix_web::{web::Data, App, HttpServer};
use aws_sdk_s3;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod services;
use services::{create_user_article, fetch_user_articles, fetch_users};

pub struct AppState {
    db: Pool<Postgres>,
    aws_s3_client: aws_sdk_s3::Client,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pg_database_url: String =
        std::env::var("PG_DATABASE_URL").expect("DATABASE_URL must be set");
    let aws_region: String = std::ev::var("AWS_REGION").expect("AWS_REGION must be set");

    HttpServer::new(move || {
        let pg_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&pg_database_url)
            .await
            .expect("Error buildinga connection pool");

        let aws_s3_client = get_aws_client(aws_region.as_str())?;

        App::new()
            .app_data(Data::new(AppState {
                db: pg_pool,
                aws_s3_client,
            }))
            .service(fetch_user_articles)
            .service(fetch_users)
            .service(create_user_article)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn get_aws_client(region: &str) -> Result<aws_sdk_s3::Client> {
    // get the id/secret from env
    let key_id: &str = env::var("AWS_ACCESS_KEY_ID").context("Missing S3_KEY_ID")?;
    let key_secret: &str = env::var("AWS_SECRET_KEY").context("Missing S3_KEY_SECRET")?;

    // build the aws cred
    let cred =
        aws_sdk_s3::Credentials::new(key_id, key_secret, None, None, "loaded-from-custom-env");

    // build the aws client
    let region = aws_sdk_s3::Region::new(region.to_string());
    let conf_builder = aws_sdk_s3::config::Builder::new()
        .region(region)
        .credentials_provider(cred);
    let conf = conf_builder.build();

    // build aws client
    let client = aws_sdk_s3::Client::from_conf(conf);
    Ok(client)
}
