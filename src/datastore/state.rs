use crate::datastore::account::Account;
use crate::datastore::plan::PricePlan;
use crate::datastore::reading::ElectricityReading;
use crate::datastore::store::DataStore;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
            PricePlan::new(
                "price-plan-0",
                "Dr Evil's Dark Energy",
                10.0,
                HashMap::new(),
            ),
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

pub fn init() -> AppState {
    let state = AppState::default();
    let mut db = state.db.lock().unwrap();

    db.insert_readings(
        "smart-meter-1".to_string(),
        ElectricityReading::generate_random(None, None),
    );

    state.to_owned()
}
