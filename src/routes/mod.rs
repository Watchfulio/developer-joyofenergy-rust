use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    datastore::state,
    handlers::{plans, readings},
};

pub async fn build() -> Router {
    let state = state::init();

    Router::new()
        .route("/readings/create", post(readings::create_readings))
        .route(
            "/readings/read/:smart_meter_id",
            get(readings::get_readings),
        )
        .route(
            "/price_plans/compare_all/:smart_meter_id",
            get(plans::get_price_plans),
        )
        .route(
            "/price_plans/recommend/:smart_meter_id",
            get(plans::get_recommended_plans),
        )
        .with_state(state)
}
