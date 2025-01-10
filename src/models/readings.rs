use crate::datastore::reading::ElectricityReading;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetElectricityReadingRequest {
    #[serde(with = "time::serde::rfc3339")]
    pub time: OffsetDateTime,
    pub reading: f64,
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone)]
pub struct GetElectricityReadingResponse {
    #[serde(with = "time::serde::rfc3339")]
    pub time: OffsetDateTime,
    pub reading: f64,
}

impl From<&ElectricityReading> for GetElectricityReadingResponse {
    fn from(electricity_reading: &ElectricityReading) -> Self {
        Self {
            time: electricity_reading.time,
            reading: electricity_reading.reading,
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateElectricityReadingsRequest {
    pub smart_meter_id: String,
    pub electricity_readings: Vec<GetElectricityReadingRequest>,
}
