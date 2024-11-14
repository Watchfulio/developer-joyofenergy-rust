use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, FixedOffset, Local, Weekday};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<DataStore>>,
}

impl Default for AppState {
    fn default() -> Self {
        let mut accounts = HashMap::new();
        accounts.insert(
            "smart-meter-0".to_string(),
            Account::new("price-plan-0", "Sarah"),
        );
        accounts.insert(
            "smart-meter-1".to_string(),
            Account::new("price-plan-1", "Peter"),
        );
        accounts.insert(
            "smart-meter-2".to_string(),
            Account::new("price-plan-0", "Charlie"),
        );
        accounts.insert(
            "smart-meter-3".to_string(),
            Account::new("price-plan-2", "Andrea"),
        );
        accounts.insert(
            "smart-meter-4".to_string(),
            Account::new("price-plan-1", "Alex"),
        );
        let price_plans = vec![
            PricePlan::new("price-plan-0", "Dr Evil's Dark Energy", 10.0, HashMap::new()),
            PricePlan::new("price-plan-1", "The Green Eco", 2.0, HashMap::new()),
            PricePlan::new("price-plan-2", "Power for Everyone", 1.0, HashMap::new()),
        ];

        Self {
            db: Arc::new(Mutex::new(DataStore::new(
                accounts,
                HashMap::new(),
                price_plans,
            ))),
        }
    }
}

pub fn init_state() -> AppState {
    let state = AppState::default();
    let mut db = state.db.lock().unwrap();
    db.add_readings(
        "smart-meter-1".to_string(),
        generate_random_readings(None, None),
    );

    state.to_owned()
}

#[derive(Debug)]
pub struct DataStore {
    accounts: HashMap<String, Account>,
    stored_readings: HashMap<String, Vec<ElectricityReading>>,
    price_plans: Vec<PricePlan>,
}

impl DataStore {
    pub fn new(
        accounts: HashMap<String, Account>,
        readings: HashMap<String, Vec<ElectricityReading>>,
        price_plans: Vec<PricePlan>,
    ) -> Self {
        Self {
            accounts: accounts,
            stored_readings: readings,
            price_plans: price_plans,
        }
    }

    pub fn add_readings(&mut self, smart_meter_id: String, readings: Vec<ElectricityReading>) {
        self.stored_readings
            .entry(smart_meter_id)
            .or_default()
            .extend(readings);
    }

    pub fn get_readings(&self, smart_meter_id: &String) -> Vec<ElectricityReading> {
        self.stored_readings
            .get(smart_meter_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_price_plans(&self) -> Vec<PricePlan> {
        self.price_plans.clone()
    }

    pub fn get_account_supplier_id(&self, smart_meter_id: &String) -> String {
        self.accounts
            .get(smart_meter_id)
            .unwrap()
            .price_plan_id
            .to_string()
    }
}

/// generated random electricity readings for seeding the datastore
/// takes the following arguments:
/// - duration: the amount of days readings are being generated for
/// - interval: the hour interval in which readings will be generated
fn generate_random_readings(
    duration: Option<i64>,
    interval: Option<i64>,
) -> Vec<ElectricityReading> {
    let duration = match duration {
        Some(d) => d,
        None => 10,
    };

    let interval = match interval {
        Some(i) => i,
        None => 6,
    };

    let mut readings = Vec::new();
    let mut rng = rand::thread_rng();

    let now = Local::now();
    let mut dummy_time = now;
    while dummy_time > now - Duration::days(duration) {
        let random_reading = rng.gen();
        let new_reading = ElectricityReading::new(dummy_time.into(), random_reading);
        readings.push(new_reading);
        dummy_time = dummy_time - Duration::hours(interval);
    }
    readings
}

#[derive(Clone, Debug)]
pub struct PricePlan {
    pub supplier_id: String,
    pub plan_name: String,
    pub unit_rate: f64,
    pub rate_multipliers: HashMap<Weekday, f64>
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
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.unit_rate.partial_cmp(&other.unit_rate).unwrap()
    }
}

impl PricePlan {
    pub fn new(supplier_id: &str, plan_name: &str, unit_rate: f64, rate_multipliers: HashMap<Weekday, f64>) -> Self {
        Self {
            supplier_id: supplier_id.to_string(),
            plan_name: plan_name.to_string(),
            unit_rate: unit_rate,
            rate_multipliers: rate_multipliers,
        }
    }

    /// calculate the cost of consumption given `stored_readings` from an account
    pub fn average_hourly_cost(&self, stored_readings: &Vec<ElectricityReading>) -> f64 {
        if stored_readings.is_empty() {
            return 0.0;
        }
        let average_reading = average_stored_reading(stored_readings);
        let hours_elapsed = total_hours_elapsed(stored_readings);

        let average_hourly_usage = average_reading / hours_elapsed;
        average_hourly_usage * self.unit_rate
    }
}

pub fn average_stored_reading(stored_readings: &Vec<ElectricityReading>) -> f64 {
    if stored_readings.is_empty() {
        return 0.0;
    }
    let readings_sum: f64 = stored_readings.iter().map(|r| r.reading).sum();
    readings_sum / stored_readings.len() as f64
}

pub fn total_hours_elapsed(stored_readings: &Vec<ElectricityReading>) -> f64 {
    if stored_readings.is_empty() {
        return 0.0;
    }
    let earliest = stored_readings.iter().min_by_key(|r| r.time).unwrap().time;
    let latest = stored_readings.iter().max_by_key(|r| r.time).unwrap().time;

    // divide by the amount of seconds in an hour in order to get how many hours,
    // or fractions of an hour have passed
    // we don't use `num_hours` because it only returns whole hours and we need
    // to handle more granular timestamps
    (latest.signed_duration_since(earliest).num_seconds() as f64) / 3600.0
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ElectricityReading {
    pub time: DateTime<FixedOffset>,
    pub reading: f64,
}

impl ElectricityReading {
    pub fn new(time: DateTime<FixedOffset>, reading: f64) -> Self {
        Self { time, reading }
    }
}

#[derive(Debug)]
pub struct Account {
    price_plan_id: String,
    #[allow(dead_code)]
    user: String,
}

impl Account {
    pub fn new(price_plan_id: &str, user: &str) -> Self {
        Self {
            price_plan_id: price_plan_id.to_string(),
            user: user.to_string(),
        }
    }
}
