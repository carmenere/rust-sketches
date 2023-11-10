use actix_web::{App, web, HttpServer};
use example_api_nodb as lib;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let db = web::Data::new(lib::state::init());
    let app = move || {
        App::new()
            .app_data(db.clone())
            .configure(lib::routes::version)
            .configure(lib::routes::persons)
    };

    HttpServer::new(app).bind("0.0.0.0:7777")?.run().await
}
