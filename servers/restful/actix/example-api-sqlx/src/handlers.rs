use actix_web::{web, HttpResponse, Responder, error::Error};
use sqlx;

use crate::models::{Person, PersonUrl};
use crate::version;
use crate::models;
use crate::state;
use crate::errors::AppError;

pub async fn build_version() -> impl Responder {
    HttpResponse::Ok().json(version::BUILD_VERSION)
}

pub async fn get_person(url: web::Path<PersonUrl>, app: web::Data<state::AppData>) -> Result<HttpResponse, AppError> {
    let mut s = app.db.begin().await.unwrap();
    let p: Result<Person, sqlx::Error> = sqlx::query_as!(models::Person, 
        r#"SELECT id, name, surname, age, address, tel from persons WHERE id = $1"#, 
        url.id
    ).fetch_one(&mut *s).await;
    let _ = s.commit().await?;
    Ok(HttpResponse::Ok().json(p?))
}

pub async fn new_person(person: web::Json<models::NewPerson>, app: web::Data<state::AppData>) -> Result<HttpResponse, AppError> {
    let mut s = app.db.begin().await.unwrap();
    let p: Result<Person, sqlx::Error> = sqlx::query_as!(models::Person, 
        r#"
            INSERT INTO persons (name, surname, age, address, tel) 
            VALUES ($1, $2, $3, $4, $5) 
            RETURNING id, name, surname, age, address, tel"#, 
            person.name, person.surname, person.age, person.address, person.tel
    ).fetch_one(&mut *s).await;
    let _ = s.commit().await?;
    Ok(HttpResponse::Ok().json(p?))
}
