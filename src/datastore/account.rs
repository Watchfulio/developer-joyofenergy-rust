#[derive(Debug)]
pub struct Account {
    pub price_plan_id: String,
    #[allow(dead_code)]
    pub user: String,
}

impl Account {
    pub fn new(price_plan_id: &str, user: &str) -> Self {
        Self {
            price_plan_id: price_plan_id.to_string(),
            user: user.to_string(),
        }
    }
}
