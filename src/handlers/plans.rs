use crate::datastore::state::AppState;
use crate::models::plans::{GetPricePlanCostResponse, GetRecommendationQueryParams};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use std::collections::{BTreeMap, HashMap};

/// Calculates hourly average costs across all price plans
///
/// # Returns
/// A tuple containing:
/// * The current supplier's price plan ID
/// * A map of price plan IDs to their average costs per hour
pub async fn get_price_plans(
    Path(smart_meter_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<GetPricePlanCostResponse>, StatusCode> {
    let data_store = state.db.lock().unwrap();

    let stored_readings = data_store.get_readings(&smart_meter_id);
    let mut price_plans = data_store.get_price_plans();
    price_plans.sort();

    let comparisons = price_plans
        .iter()
        .map(|price_plan| {
            let consumption_cost = price_plan.average_hourly_cost(&stored_readings);
            (price_plan.supplier_id.to_string(), consumption_cost)
        })
        .collect::<BTreeMap<String, f64>>();

    Ok(Json(GetPricePlanCostResponse {
        price_plans: comparisons,
        supplier_id: data_store.get_account_supplier_id(&smart_meter_id),
    }))
}

pub async fn get_recommended_plans(
    Path(smart_meter_id): Path<String>,
    Query(query): Query<GetRecommendationQueryParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<HashMap<String, f64>>>, StatusCode> {
    let data_store = state.db.lock().unwrap();

    let mut price_plans = data_store.get_price_plans();
    price_plans.sort();
    let stored_readings = data_store.get_readings(&smart_meter_id);
    let limit = query.limit;

    let response: Vec<HashMap<String, f64>> = price_plans
        .iter()
        .take(limit as usize)
        .map(|price_plan| {
            let cost = price_plan.average_hourly_cost(&stored_readings);
            let mut plan_recommendation = HashMap::new();
            plan_recommendation.insert(price_plan.supplier_id.clone(), cost);
            plan_recommendation
        })
        .collect();

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use crate::datastore::reading::ElectricityReading;
    use crate::datastore::state::AppState;
    use crate::handlers::plans::{get_price_plans, get_recommended_plans};
    use crate::models::plans::{GetPricePlanCostResponse, GetRecommendationQueryParams};
    use axum::extract::{Path, Query, State};
    use axum::Json;
    use time::macros::datetime;

    fn make_state() -> AppState {
        AppState::default()
    }

    #[tokio::test]
    async fn testing_getting_price_plans() {
        let state = make_state();
        {
            let mut db = state.db.lock().unwrap();
            let readings = vec![
                ElectricityReading {
                    time: datetime!(2020-11-29 08:00:00 UTC),
                    reading: 1.0,
                },
                ElectricityReading {
                    time: datetime!(2020-11-29 08:01:00 UTC),
                    reading: 2.0,
                },
                ElectricityReading {
                    time: datetime!(2020-11-29 08:02:00 UTC),
                    reading: 3.0,
                },
            ];
            db.insert_readings("smart-meter-0".to_string(), readings);
        }
        let path = Path("smart-meter-0".to_string());
        let Json(result) = get_price_plans(path, State(state)).await.unwrap();
        let mut expected_plans = BTreeMap::new();
        expected_plans.insert("price-plan-0".to_string(), 600.0);
        expected_plans.insert("price-plan-1".to_string(), 120.0);
        expected_plans.insert("price-plan-2".to_string(), 60.0);
        let expected_result = GetPricePlanCostResponse {
            price_plans: expected_plans,
            supplier_id: "price-plan-0".to_string(),
        };

        assert_eq!(expected_result, result);
    }

    #[tokio::test]
    async fn testing_getting_price_recommendations() {
        let state = make_state();
        {
            let mut db = state.db.lock().unwrap();
            let readings = vec![
                ElectricityReading {
                    time: datetime!(2020-11-29 08:00:00 UTC),
                    reading: 1.0,
                },
                ElectricityReading {
                    time: datetime!(2020-11-29 08:01:00 UTC),
                    reading: 2.0,
                },
                ElectricityReading {
                    time: datetime!(2020-11-29 08:02:00 UTC),
                    reading: 3.0,
                },
            ];
            db.insert_readings("smart-meter-0".to_string(), readings);
        }
        let path = Path("smart-meter-0".to_string());
        let limit = Query(GetRecommendationQueryParams { limit: 2 });

        let Json(result) = get_recommended_plans(path, limit, State(state))
            .await
            .unwrap();
        let expected_result: Vec<HashMap<String, f64>> = vec![
            HashMap::from([("price-plan-2".to_string(), 60.0)]),
            HashMap::from([("price-plan-1".to_string(), 120.0)]),
        ];

        assert_eq!(expected_result, result);
    }
}
