use std::str::FromStr;

use crate::datastore::reading::ElectricityReading;
use crate::datastore::state::AppState;
use crate::models::readings::{CreateElectricityReadingsRequest, GetElectricityReadingResponse};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::DateTime;

pub async fn get_readings(
    Path(smart_meter_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<GetElectricityReadingResponse>>, StatusCode> {
    let data_store = state.db.lock().unwrap();

    let stored_readings = data_store
        .get_readings(&smart_meter_id)
        .iter()
        .map(|reading| GetElectricityReadingResponse::from(reading))
        .collect::<Vec<GetElectricityReadingResponse>>();

    Ok(Json(stored_readings))
}

pub async fn create_readings(
    State(state): State<AppState>,
    Json(body): Json<CreateElectricityReadingsRequest>,
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
    data_store.insert_readings(smart_meter_id, readings);

    Ok("Readings created successfully".to_string())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::datastore::reading::ElectricityReading;
    use crate::datastore::state::AppState;
    use crate::handlers::readings::{create_readings, get_readings};
    use crate::models::readings::{
        CreateElectricityReadingsRequest, GetElectricityReadingRequest,
        GetElectricityReadingResponse,
    };
    use axum::extract::{Path, State};
    use axum::Json;
    use chrono::DateTime;

    fn make_state() -> AppState {
        AppState::default()
    }

    #[tokio::test]
    async fn testing_getting_empty_readings() {
        let state = make_state();
        let path = Path("smart-meter-0".to_string());
        let Json(result) = get_readings(path, State(state)).await.unwrap();

        assert_eq!(Vec::<GetElectricityReadingResponse>::new(), result);
    }

    #[tokio::test]
    async fn testing_storing_readings() {
        let state = make_state();

        let request_body = Json(CreateElectricityReadingsRequest {
            smart_meter_id: "smart-meter-0".to_string(),
            electricity_readings: vec![
                GetElectricityReadingRequest {
                    time: "2020-11-29T08:00:00Z".to_string(),
                    reading: 1.0,
                },
                GetElectricityReadingRequest {
                    time: "2020-11-29T08:01:00Z".to_string(),
                    reading: 2.0,
                },
                GetElectricityReadingRequest {
                    time: "2020-11-29T08:02:00Z".to_string(),
                    reading: 3.0,
                },
            ],
        });

        let result = create_readings(State(state), request_body).await.unwrap();

        assert_eq!(result, "Readings created successfully".to_string());
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
            db.insert_readings("smart-meter-0".to_string(), readings);
        }
        let path = Path("smart-meter-0".to_string());
        let Json(result) = get_readings(path, State(state)).await.unwrap();

        let expected_results = vec![
            GetElectricityReadingResponse {
                time: DateTime::from_str("2020-11-29T08:00:00Z").unwrap(),
                reading: 1.0,
            },
            GetElectricityReadingResponse {
                time: DateTime::from_str("2020-11-29T08:01:00Z").unwrap(),
                reading: 2.0,
            },
            GetElectricityReadingResponse {
                time: DateTime::from_str("2020-11-29T08:02:00Z").unwrap(),
                reading: 3.0,
            },
        ];
        assert_eq!(expected_results, result);
    }
}
