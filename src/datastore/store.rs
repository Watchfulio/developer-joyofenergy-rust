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

// ... existing code ...

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_reading(time: i64, reading: f64) -> ElectricityReading {
        ElectricityReading {
            time: chrono::DateTime::from_timestamp(time, 0).unwrap().into(),
            reading: reading,
        }
    }

    fn setup_test_store() -> DataStore {
        let mut accounts = HashMap::new();
        accounts.insert(
            "meter-1".to_string(),
            Account {
                price_plan_id: "plan-1".to_string(),
                user: "user-1".to_string(),
            },
        );

        let price_plans = vec![PricePlan {
            supplier_id: "plan-1".to_string(),
            plan_name: "plan-1".to_string(),
            rate_multipliers: HashMap::new(),
            unit_rate: 10.0,
        }];

        let readings = HashMap::new();

        DataStore::new(accounts, readings, price_plans)
    }

    #[test]
    fn test_insert_readings() {
        let mut store = setup_test_store();
        let readings = vec![
            create_test_reading(1000, 1.5),
            create_test_reading(2000, 2.5),
        ];

        store.insert_readings("meter-1".to_string(), readings.clone());

        assert_eq!(store.get_readings(&"meter-1".to_string()), readings);
    }

    #[test]
    fn test_get_readings_existing_meter() {
        let mut store = setup_test_store();
        let readings = vec![create_test_reading(1000, 1.5)];
        store.insert_readings("meter-1".to_string(), readings.clone());

        assert_eq!(store.get_readings(&"meter-1".to_string()), readings);
    }

    #[test]
    fn test_get_readings_nonexistent_meter() {
        let store = setup_test_store();

        assert!(store.get_readings(&"nonexistent".to_string()).is_empty());
    }

    #[test]
    fn test_get_price_plans() {
        let store = setup_test_store();
        let plans = store.get_price_plans();
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].supplier_id, "plan-1");
        assert_eq!(plans[0].unit_rate, 10.0);
    }

    #[test]
    fn test_get_account_supplier_id() {
        let store = setup_test_store();

        assert_eq!(
            store.get_account_supplier_id(&"meter-1".to_string()),
            "plan-1"
        );
    }

    #[test]
    #[should_panic]
    fn test_get_account_supplier_id_nonexistent() {
        let store = setup_test_store();
        store.get_account_supplier_id(&"nonexistent".to_string());
    }
}
