use crate::datastore::reading::ElectricityReading;
use chrono::Weekday;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PricePlan {
    pub supplier_id: String,
    #[allow(dead_code)]
    pub plan_name: String,
    pub unit_rate: f64,
    #[allow(dead_code)]
    pub rate_multipliers: HashMap<Weekday, f64>,
}

impl Eq for PricePlan {}

impl PartialEq<Self> for PricePlan {
    fn eq(&self, other: &Self) -> bool {
        self.unit_rate == other.unit_rate
    }
}

impl PartialOrd<Self> for PricePlan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.unit_rate.partial_cmp(&other.unit_rate)
    }
}

impl Ord for PricePlan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unit_rate.partial_cmp(&other.unit_rate).unwrap()
    }
}

impl PricePlan {
    pub fn new(
        supplier_id: &str,
        plan_name: &str,
        unit_rate: f64,
        rate_multipliers: HashMap<Weekday, f64>,
    ) -> Self {
        Self {
            supplier_id: supplier_id.to_string(),
            plan_name: plan_name.to_string(),
            unit_rate,
            rate_multipliers,
        }
    }

    /// Calculates the total cost for a collection of electricity readings
    ///
    /// # Arguments
    /// * `stored_readings` - Vector of electricity readings from an account
    ///
    /// # Returns
    /// The total cost in currency units based on the plan's unit rate
    pub fn average_hourly_cost(&self, stored_readings: &Vec<ElectricityReading>) -> f64 {
        if stored_readings.is_empty() {
            return 0.0;
        }
        let average_reading = Self::average_reading(stored_readings);
        let hours_elapsed = Self::total_hours_elapsed(stored_readings);

        let average_hourly_usage = average_reading / hours_elapsed;
        average_hourly_usage * self.unit_rate
    }

    fn average_reading(stored_readings: &Vec<ElectricityReading>) -> f64 {
        if stored_readings.is_empty() {
            return 0.0;
        }
        let readings_sum: f64 = stored_readings.iter().map(|r| r.reading).sum();
        readings_sum / stored_readings.len() as f64
    }

    fn total_hours_elapsed(stored_readings: &Vec<ElectricityReading>) -> f64 {
        if stored_readings.is_empty() {
            return 0.0;
        }
        let earliest = stored_readings.iter().min_by_key(|r| r.time).unwrap().time;
        let latest = stored_readings.iter().max_by_key(|r| r.time).unwrap().time;

        // Convert time difference to fractional hours
        // by dividing total seconds by 3600 (seconds per hour).
        // Using seconds provides more precise calculations than `num_hours()`
        // which truncates partial hours.
        (latest.signed_duration_since(earliest).num_seconds() as f64) / 3600.0
    }
}
