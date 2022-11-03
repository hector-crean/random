use crate::{figma, AppState};
use actix_web::{get, post, web, HttpResponse};
use awc::{self, http};

#[get("/files")]
async fn fetch_figma_file(state: web::Data<AppState>) -> HttpResponse {
    let url = format!("https://api.figma.com/v1/files/EGwEdET4TWpZ8baTA60amp");

    match state
        .awc_client
        .get(url)
        .insert_header((awc::http::header::USER_AGENT, "Actix-web"))
        .insert_header(("X-Figma-Token", state.figma_personal_access_token.as_str()))
        .insert_header((awc::http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .send()
        .await
    {
        Ok(mut resp) => match resp.body().await {
            Ok(payload) => HttpResponse::Ok().body(payload),
            Err(payload_err) => {
                HttpResponse::NotFound().json(format!("Payload Error: {}", payload_err))
            }
        },
        Err(send_request_err) => {
            HttpResponse::NotFound().json(format!("Send Request Error: {}", send_request_err))
        }
    }
}
