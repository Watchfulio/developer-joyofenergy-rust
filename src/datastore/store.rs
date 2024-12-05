use crate::datastore::account::Account;
use crate::datastore::plan::PricePlan;
use crate::datastore::reading::ElectricityReading;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DataStore {
    accounts: HashMap<String, Account>,
    price_plans: Vec<PricePlan>,
    readings: HashMap<String, Vec<ElectricityReading>>,
}

impl DataStore {
    pub fn new(
        accounts: HashMap<String, Account>,
        readings: HashMap<String, Vec<ElectricityReading>>,
        price_plans: Vec<PricePlan>,
    ) -> Self {
        Self {
            accounts,
            readings,
            price_plans,
        }
    }

    pub fn insert_readings(&mut self, smart_meter_id: String, readings: Vec<ElectricityReading>) {
        self.readings
            .entry(smart_meter_id)
            .or_default()
            .extend(readings);
    }

    pub fn get_readings(&self, smart_meter_id: &String) -> Vec<ElectricityReading> {
        self.readings
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
