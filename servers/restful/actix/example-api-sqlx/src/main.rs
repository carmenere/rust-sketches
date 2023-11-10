use actix_web::{App, web, HttpServer};

use example_api_sqlx as lib;
use lib::state::AppData;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    let app_data = web::Data::new(AppData::new().await);
    let app = move || {
        App::new()
            .app_data(app_data.clone())
            .configure(lib::routes::version)
            .configure(lib::routes::persons)
    };

    HttpServer::new(app).bind("0.0.0.0:8888")?.run().await
}
