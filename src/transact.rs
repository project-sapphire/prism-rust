#[derive(Clone, Debug)]
pub struct Invoice {
    pub address: String,
    pub currency: String,
    pub amount: f64,
}