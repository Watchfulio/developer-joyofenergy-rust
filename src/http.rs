mod views;

use axum::{
    routing::{get, post},
    Router,
};

use crate::datastore::init_state;

pub async fn build() -> Router {
    let state = init_state();

    Router::new()
        .route("/readings/store", post(views::store_readings))
        .route("/readings/read/:smart_meter_id", get(views::get_readings))
        .route(
            "/price_plans/compare_all/:smart_meter_id",
            get(views::get_all_price_plans),
        )
        .route(
            "/price_plans/recommend/:smart_meter_id",
            get(views::get_recommended_plans),
        )
        .with_state(state)
}
