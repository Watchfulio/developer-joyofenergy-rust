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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use axum::body::to_bytes;
    use serde_json::{json, Value};

    async fn setup() -> Router {
        build().await
    }

    #[tokio::test]
    async fn test_create_readings() {
        let app = setup().await;

        let request_body = r#"{
            "smart_meter_id": "smart-meter-0",
            "electricity_readings": [
                {"time": "2024-01-01T00:00:00Z", "reading": 1.23}
            ]
        }"#;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/readings/create")
                    .header("Content-Type", "application/json")
                    .body(Body::from(request_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(body_str, "Readings created successfully");
    }

    #[tokio::test]
     async fn test_get_readings() {
        let app = setup().await;

        // First create some readings
        let create_body = json!({
            "smart_meter_id": "smart-meter-0",
            "electricity_readings": [
                {"time": "2024-01-01T00:00:00Z", "reading": 1.23}
            ]
        });

        let _ = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/readings/create")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&create_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Then get the readings
        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/readings/read/smart-meter-0")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let expected = json!([
            {
                "time": "2024-01-01T00:00:00Z",
                "reading": 1.23
            }
        ]);

        assert_eq!(body, expected);
    }

    #[tokio::test]
    async fn test_compare_price_plans() {
        let app = setup().await;

        // First create some readings to compare
        let create_body = json!({
            "smart_meter_id": "smart-meter-0",
            "electricity_readings": [
                {"time": "2024-01-01T00:00:00Z", "reading": 1.23}
            ]
        });

        let _ = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/readings/create")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&create_body).unwrap()))
                    .unwrap(),
            )
            .await;

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/price_plans/compare_all/smart-meter-0")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        // Check structure matches GetPricePlanCostResponse
        assert!(body.get("price_plans").is_some());
        assert!(body.get("supplier_id").is_some());
    }
}
