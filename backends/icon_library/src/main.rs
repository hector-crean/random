use std::{sync::Arc, time::Instant};

use actix_web::{middleware, web, App, HttpServer};
use awc::{http::header, Client, Connector};
use dotenv::dotenv;
use reqwest;
use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};

mod figma;
mod services;

use services::fetch_figma_file;

pub struct AppState {
    figma_base_url: String,
    figma_file_key: String,
    figma_personal_access_token: String,
    awc_client: awc::Client,
    reqwest_client: reqwest::Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let client_tls_config = Arc::new(rustls_config());

    let figma_base_url: String =
        std::env::var("FIGMA_BASE_URL").expect("FIGMA_BASE_URL must be set");

    let figma_file_key: String =
        std::env::var("FIGMA_FILE_KEY").expect("FIGMA_FILE_KEY must be set");

    let figma_personal_access_token: String = std::env::var("FIGMA_PERSONAL_ACCESS_TOKEN")
        .expect("FIGMA_PERSONAL_ACCESS_TOKEN must be set");

    HttpServer::new(move || {
        // create client _inside_ `HttpServer::new` closure to have one per worker thread
        let awc_client = Client::builder()
            // a "connector" wraps the stream into an encrypted connection
            .connector(Connector::new().rustls(Arc::clone(&client_tls_config)))
            .finish();

        let reqwest_client = reqwest::Client::new();

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(AppState {
                figma_base_url: figma_base_url.clone(),
                figma_file_key: figma_file_key.clone(),
                figma_personal_access_token: figma_personal_access_token.clone(),
                awc_client: awc_client,
                reqwest_client: reqwest_client,
            }))
            .service(fetch_figma_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// Create simple rustls client config from root certificates.
fn rustls_config() -> ClientConfig {
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}
