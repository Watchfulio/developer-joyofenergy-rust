use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::datastore::{AppState, ElectricityReading};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ElectricityReadingRequest {
    pub time: String,
    pub reading: f64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct StoreReadingRequest {
    smart_meter_id: String,
    electricity_readings: Vec<ElectricityReadingRequest>,
}

pub async fn store_readings(
    State(state): State<AppState>,
    Json(body): Json<StoreReadingRequest>,
) -> Result<String, StatusCode> {
    let smart_meter_id = body.smart_meter_id;
    let readings = body.electricity_readings;
    let mut data_store = state.db.lock().unwrap();

    let readings = readings
        .iter()
        .map(|r| ElectricityReading {
            reading: r.reading,
            time: DateTime::from_str(&r.time).unwrap(),
        })
        .collect::<Vec<ElectricityReading>>();
    data_store.add_readings(smart_meter_id, readings);

    Ok("Readings stored successfully".to_string())
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone)]
pub struct ElectricityReadingResponse {
    time: DateTime<FixedOffset>,
    reading: f64,
}

impl From<&ElectricityReading> for ElectricityReadingResponse {
    fn from(electricity_reading: &ElectricityReading) -> Self {
        Self {
            time: electricity_reading.time,
            reading: electricity_reading.reading,
        }
    }
}

pub async fn get_readings(
    Path(smart_meter_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ElectricityReadingResponse>>, StatusCode> {
    let data_store = state.db.lock().unwrap();

    let stored_readings = data_store
        .get_readings(&smart_meter_id)
        .iter()
        .map(|reading| ElectricityReadingResponse::from(reading))
        .collect::<Vec<ElectricityReadingResponse>>();

    Ok(Json(stored_readings))
}

#[derive(Serialize, Debug, PartialEq)]
pub struct PricePlanCostResponse {
    price_plans: BTreeMap<String, f64>,
    supplier_id: String,
}

// based on the stored readings of a given smart meter gets the average cost per hour
// for all of the available plans and the id of the current supplier that the account uses
pub async fn get_all_price_plans(
    Path(smart_meter_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<PricePlanCostResponse>, StatusCode> {
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

    Ok(Json(PricePlanCostResponse {
        price_plans: comparisons,
        supplier_id: data_store.get_account_supplier_id(&smart_meter_id),
    }))
}

#[derive(Deserialize, Debug)]
pub struct RecommendationQueryParams {
    limit: u64,
}

pub async fn get_recommended_plans(
    Path(smart_meter_id): Path<String>,
    Query(query): Query<RecommendationQueryParams>,
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
    use std::str::FromStr;

    use axum::extract::{Path, Query, State};
    use axum::Json;
    use chrono::DateTime;

    use crate::datastore::{AppState, ElectricityReading};
    use crate::http::views::{
        get_all_price_plans, get_readings, get_recommended_plans, store_readings,
        ElectricityReadingRequest, ElectricityReadingResponse, PricePlanCostResponse,
        RecommendationQueryParams, StoreReadingRequest,
    };

    fn make_state() -> AppState {
        AppState::default()
    }

    #[tokio::test]
    async fn testing_getting_empty_readings() {
        let state = make_state();
        let path = Path("smart-meter-0".to_string());
        let Json(result) = get_readings(path, State(state)).await.unwrap();

        assert_eq!(Vec::<ElectricityReadingResponse>::new(), result);
    }

    #[tokio::test]
    async fn testing_storing_readings() {
        let state = make_state();

        let request_body = Json(StoreReadingRequest {
            smart_meter_id: "smart-meter-0".to_string(),
            electricity_readings: vec![
                ElectricityReadingRequest {
                    time: "2020-11-29T08:00:00Z".to_string(),
                    reading: 1.0,
                },
                ElectricityReadingRequest {
                    time: "2020-11-29T08:01:00Z".to_string(),
                    reading: 2.0,
                },
                ElectricityReadingRequest {
                    time: "2020-11-29T08:02:00Z".to_string(),
                    reading: 3.0,
                },
            ],
        });

        let result = store_readings(State(state), request_body).await.unwrap();

        assert_eq!(result, "Readings stored successfully".to_string());
    }

    #[tokio::test]
    async fn testing_getting_existing_readings() {
        let state = make_state();
        {
            let mut db = state.db.lock().unwrap();
            let readings = vec![
                ElectricityReading {
                    time: DateTime::from_str("2020-11-29T08:00:00Z").unwrap(),
                    reading: 1.0,
                },
                ElectricityReading {
                    time: DateTime::from_str("2020-11-29T08:01:00Z").unwrap(),
                    reading: 2.0,
                },
                ElectricityReading {
                    time: DateTime::from_str("2020-11-29T08:02:00Z").unwrap(),
                    reading: 3.0,
                },
            ];
            db.add_readings("smart-meter-0".to_string(), readings);
        }
        let path = Path("smart-meter-0".to_string());
        let Json(result) = get_readings(path, State(state)).await.unwrap();

        let expected_results = vec![
            ElectricityReadingResponse {
                time: DateTime::from_str("2020-11-29T08:00:00Z").unwrap(),
                reading: 1.0,
            },
            ElectricityReadingResponse {
                time: DateTime::from_str("2020-11-29T08:01:00Z").unwrap(),
                reading: 2.0,
            },
            ElectricityReadingResponse {
                time: DateTime::from_str("2020-11-29T08:02:00Z").unwrap(),
                reading: 3.0,
            },
        ];
        assert_eq!(expected_results, result);
    }

    #[tokio::test]
    async fn testing_getting_price_plans() {
        let state = make_state();
        {
            let mut db = state.db.lock().unwrap();
            let readings = vec![
                ElectricityReading {
                    time: DateTime::from_str("2020-11-29T08:00:00Z").unwrap(),
                    reading: 1.0,
                },
                ElectricityReading {
                    time: DateTime::from_str("2020-11-29T08:01:00Z").unwrap(),
                    reading: 2.0,
                },
                ElectricityReading {
                    time: DateTime::from_str("2020-11-29T08:02:00Z").unwrap(),
                    reading: 3.0,
                },
            ];
            db.add_readings("smart-meter-0".to_string(), readings);
        }
        let path = Path("smart-meter-0".to_string());
        let Json(result) = get_all_price_plans(path, State(state)).await.unwrap();
        let mut expected_plans = BTreeMap::new();
        expected_plans.insert("price-plan-0".to_string(), 600.0);
        expected_plans.insert("price-plan-1".to_string(), 120.0);
        expected_plans.insert("price-plan-2".to_string(), 60.0);
        let expected_result = PricePlanCostResponse {
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
                    time: DateTime::from_str("2020-11-29T08:00:00Z").unwrap(),
                    reading: 1.0,
                },
                ElectricityReading {
                    time: DateTime::from_str("2020-11-29T08:01:00Z").unwrap(),
                    reading: 2.0,
                },
                ElectricityReading {
                    time: DateTime::from_str("2020-11-29T08:02:00Z").unwrap(),
                    reading: 3.0,
                },
            ];
            db.add_readings("smart-meter-0".to_string(), readings);
        }
        let path = Path("smart-meter-0".to_string());
        let limit = Query(RecommendationQueryParams { limit: 2 });

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
