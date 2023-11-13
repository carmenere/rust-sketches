use actix_web::{http::StatusCode, HttpResponse};
use std::sync::Once;
mod setup;
use example_api_sqlx as lib;

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(env_logger::init);
}

#[tokio::test]
async fn create_new_person()  {
    init();
    let app_data: actix_web::web::Data<lib::state::AppData> = setup::app_data().await;
    let person = lib::models::NewPerson {
        name: "Anton".to_string(),
        surname: "Romanov".to_string(),
        age: 20,
        address: Some("Moscow".to_string()),
        tel: Some("915".to_string())
    };

    let resp = lib::handlers::new_person(actix_web::web::Json(person), app_data).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_existing_person() {
    init();
    let app_data = setup::app_data().await;
    let person = lib::models::NewPerson {
        name: "Boris".to_string(),
        surname: "Romanov".to_string(),
        age: 20,
        address: Some("Moscow".to_string()),
        tel: Some("915".to_string())
    };

    let resp = lib::handlers::new_person(actix_web::web::Json(person.clone()), app_data.clone()).await.unwrap();
    println!("STATUS1: {}", resp.status());
    let resp = lib::handlers::new_person(actix_web::web::Json(person), app_data).await.unwrap();
    println!("STATUS2: {}", resp.status());
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}