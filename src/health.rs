use std::sync::Arc;

use actix_web::{
    get,
    web::{Data, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(health_check);
}

#[get("/health")]
async fn health_check(index: Data<i16>) -> HttpResponse {
    let t_index = Arc::new(&index);
    println!("Llega el dato de thead con valor: {}", t_index.to_string());
    HttpResponse::Ok()
        .insert_header(("thread-id", t_index.to_string()))
        .body("¡Hola, otro!!!")
}

#[cfg(test)]
mod test {

    use actix_web::{
        http::StatusCode,
        test,
        web::{self},
        App,
    };
    use std::str;

    use super::*;

    #[actix_web::test]
    async fn health_check_works() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(1i16))
                .service(health_check),
        )
        .await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.status().is_success())
    }
    #[actix_web::test]
    async fn health_check_status_200() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(1i16))
                .service(health_check),
        )
        .await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn health_check_heder_value_is_equals() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(1i16))
                .service(health_check),
        )
        .await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        let data = resp
            .headers()
            .get("thread-id")
            .map(|h| h.to_str().ok())
            .flatten();
        assert_eq!(data, Some("1"));
    }

    #[actix_web::test]
    async fn health_check_heder_value_is_different() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(5i16))
                .service(health_check),
        )
        .await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        let data = resp
            .headers()
            .get("thread-id")
            .map(|h| h.to_str().ok())
            .flatten();
        assert_ne!(data, Some("1"));
    }

    #[actix_web::test]
    async fn health_check_error_404() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(5i16))
                .service(health_check),
        )
        .await;
        let req = test::TestRequest::get().uri("/healt").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn health_check_response_body() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(5i16))
                .service(health_check),
        )
        .await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        // Verifica que el cuerpo de la respuesta sea el esperado
        let body = test::read_body(resp).await;
        // println!("Response body: {:?}", String::from(body));
        assert_eq!(
            str::from_utf8(&body).unwrap(),
            "¡Hola, otro!!!".to_string()
        );
    }
}
