use std::collections::HashMap;


#[derive(Clone, Debug)]
pub struct Rate {
    pub timestamp: u64,
    pub values: HashMap<String, f64>,
}

#[derive(Clone, Debug)]
pub struct RateUpdate {
    pub exchange: String,
    pub currency: String,
    pub rate: Rate,
}

