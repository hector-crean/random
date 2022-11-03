use crate::{figma, AppState};
use actix_web::{get, post, web, HttpResponse};
use awc;

#[get("/")]
async fn fetch_figma_file(state: web::Data<AppState>) -> HttpResponse {
    let url = format!("https://api.figma.com/v1/files/EGwEdET4TWpZ8baTA60amp");

    match state
        .awc_client
        .get(url)
        .insert_header((
            "X-FIGMA-TOKEN",
            "figd_zCN63Lnpu9TQ0JE8xGu5SrfPxJ6JtnGZxIn78rHx",
        ))
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
