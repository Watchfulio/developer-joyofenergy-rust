use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Debug, PartialEq)]
pub struct GetPricePlanCostResponse {
    pub price_plans: BTreeMap<String, f64>,
    pub supplier_id: String,
}

#[derive(Deserialize, Debug)]
pub struct GetRecommendationQueryParams {
    pub limit: u64,
}
