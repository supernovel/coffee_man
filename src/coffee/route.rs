use crate::payload::Payload;
/// 커피 선택 관련 라우트 정의
use actix_web::{web, HttpResponse};
use serde_json::json;

pub fn add_post(_payload: web::Form<Payload>) -> HttpResponse {
    HttpResponse::MethodNotAllowed().json(json!({
        "response_type": "ephemeral",
        "text": "Sorry, Not implemented yet."
    }))
}

pub fn pick_post(_payload: web::Form<Payload>) -> HttpResponse {
    HttpResponse::MethodNotAllowed().json(json!({
        "response_type": "ephemeral",
        "text": "Sorry, Not implemented yet."
    }))
}

pub fn start_pick_post(payload: web::Form<Payload>) -> HttpResponse {
    match &payload.text {
        Some(text) => match text.parse::<u32>() {
            Ok(number) => HttpResponse::Ok().json(super::start_pick(number)),
            Err(_) => HttpResponse::BadRequest().json(json!({
                "response_type": "ephemeral",
                "text": "Required natural number."
            })),
        },
        None => HttpResponse::NotFound().json(json!({
            "response_type": "ephemeral",
            "text": "Sorry, that didn't work. Please try again."
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_start_pick() -> Result<(), Error> {
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/start").route(web::post().to(start_pick_post)))
                .service(web::resource("/pick").route(web::post().to(pick_post)))
                .service(web::resource("/add").route(web::post().to(add_post))),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/start")
            .set_form(&json!({
                "token": "gIkuvaNzQIHg97ATvDxqgjtO",
                "team_id": "T0001",
                "team_domain": "example",
                "enterprise_id": "E0001",
                "enterprise_name": "Globular%20Construct%20Inc",
                "channel_id": "C2147483705",
                "channel_name": "test",
                "user_id": "U2147483697",
                "user_name": "Steve",
                "command": "/weather",
                "text": "94070",
                "response_url": "https://hooks.slack.com/commands/1234/5678",
                "trigger_id": "13345224609.738474920.8088930838d88f008e0"
            }))
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"name":"my-name","number":43}"##);

        Ok(())
    }
}
