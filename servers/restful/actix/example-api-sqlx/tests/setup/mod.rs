use actix_web::{web};

use example_api_sqlx as lib;
use lib::state::AppData;


pub async fn app_data() -> web::Data<AppData> {
    web::Data::new(AppData::new().await)
}
