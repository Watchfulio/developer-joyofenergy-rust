use rand::Rng;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ElectricityReading {
    pub time: OffsetDateTime,
    pub reading: f64,
}

impl ElectricityReading {
    pub fn new(time: OffsetDateTime, reading: f64) -> Self {
        Self { time, reading }
    }

    /// Generates a collection of electricity readings for testing purposes
    ///
    /// # Arguments
    /// * `duration` - Number of days to generate readings for
    /// * `interval` - Hour interval between readings
    ///
    /// # Returns
    /// A vector of `ElectricityReading` instances with randomized values
    pub(super) fn generate_random(
        duration: Option<i64>,
        interval: Option<i64>,
    ) -> Vec<ElectricityReading> {
        let duration = duration.unwrap_or_else(|| 10);

        let interval = interval.unwrap_or_else(|| 6);

        let mut readings = Vec::new();
        let mut rng = rand::thread_rng();

        let now = OffsetDateTime::now_utc();
        let mut dummy_time = now;
        while dummy_time > now - Duration::days(duration) {
            let random_reading = rng.gen();
            let new_reading = ElectricityReading::new(dummy_time.into(), random_reading);
            readings.push(new_reading);
            dummy_time = dummy_time - Duration::hours(interval);
        }
        readings
    }
}
