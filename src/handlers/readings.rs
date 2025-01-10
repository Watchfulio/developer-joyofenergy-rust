use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use crate::datastore::reading::ElectricityReading;
use crate::datastore::state::AppState;
use crate::models::readings::{CreateElectricityReadingsRequest, GetElectricityReadingResponse};

pub async fn get_readings(
    Path(smart_meter_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<GetElectricityReadingResponse>>, StatusCode> {
    let data_store = state.db.lock().unwrap();

    let stored_readings = data_store
        .get_readings(&smart_meter_id)
        .iter()
        .map(GetElectricityReadingResponse::from)
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
            time: r.time,
        })
        .collect::<Vec<ElectricityReading>>();
    data_store.insert_readings(smart_meter_id, readings);

    Ok("Readings created successfully".to_string())
}

#[cfg(test)]
mod tests {
    use axum::extract::{Path, State};
    use axum::Json;
    use time::macros::datetime;

    use crate::datastore::reading::ElectricityReading;
    use crate::datastore::state::AppState;
    use crate::handlers::readings::{create_readings, get_readings};
    use crate::models::readings::{
        CreateElectricityReadingsRequest, GetElectricityReadingRequest,
        GetElectricityReadingResponse,
    };

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
                    time: datetime!(2020-11-29 08:00:00 UTC),
                    reading: 1.0,
                },
                GetElectricityReadingRequest {
                    time: datetime!(2020-11-29 08:01:00 UTC),
                    reading: 2.0,
                },
                GetElectricityReadingRequest {
                    time: datetime!(2020-11-29 08:02:00 UTC),
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
        let Json(result) = get_readings(path, State(state)).await.unwrap();

        let expected_results = vec![
            GetElectricityReadingResponse {
                time: datetime!(2020-11-29 08:00:00 UTC),
                reading: 1.0,
            },
            GetElectricityReadingResponse {
                time: datetime!(2020-11-29 08:01:00 UTC),
                reading: 2.0,
            },
            GetElectricityReadingResponse {
                time: datetime!(2020-11-29 08:02:00 UTC),
                reading: 3.0,
            },
        ];
        assert_eq!(expected_results, result);
    }
}
