use actix_web::web;
use crate::handlers;

pub fn version(cfg: &mut web::ServiceConfig) {
    cfg.route("/build_version", web::get().to(handlers::build_version));
}

pub fn persons(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/persons")
                    .route("", web::post().to(handlers::new_person))
                    .route("/{id}", web::get().to(handlers::get_person)),
    );
}
