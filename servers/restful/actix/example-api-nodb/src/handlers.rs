use actix_web::{web, HttpResponse, Responder};
use crate::models::{Person, PersonUrl};
use crate::version;
use crate::models;
use crate::state;

pub async fn build_version() -> impl Responder {
    HttpResponse::Ok().json(version::BUILD_VERSION)
}

pub async fn get_person(url: web::Path<PersonUrl>, db: web::Data<state::Db>) -> HttpResponse {
    let id = url.id;
    let hm = db.persons.table.lock().unwrap();
    let p = hm.get(&id);
    match p {
        Some(p) => HttpResponse::Ok().json(&p),
        None => HttpResponse::NotFound().body("Not found!"),
    }
}

pub async fn new_person(person: web::Json<models::NewPerson>, db: web::Data<state::Db>) -> HttpResponse {
    let mut p = Person::from(person);
    let mut id = db.persons.id.lock().unwrap();
    *id += 1;
    p.id = *id;
    let r = HttpResponse::Ok().json(&p);
    let mut hm = db.persons.table.lock().unwrap();
    hm.insert(p.id, p);
    r 
}
