use crate::datastore::reading::ElectricityReading;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetElectricityReadingRequest {
    pub time: String,
    pub reading: f64,
}

#[derive(Serialize, Debug, PartialEq, Copy, Clone)]
pub struct GetElectricityReadingResponse {
    pub time: DateTime<FixedOffset>,
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
