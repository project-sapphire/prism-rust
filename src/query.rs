use super::{Message, ReceiveError};

#[derive(Debug, Clone)]
pub enum ExchangeQuery {
    History(u64),
    // this is no good... :(
    Exchange(String, f64, String, String),
    Status(String),
}

#[derive(Debug, Clone)]
pub enum WalletQuery {
    Balance,
    Currencies,
    Pay(f64, String),
    Receive
}

#[derive(Debug, Clone)]
pub struct ExchangeRequest {
    pub query: ExchangeQuery,
    pub exchange: String,
    pub currency: String,
}

#[derive(Debug, Clone)]
pub struct WalletRequest {
    pub query: WalletQuery,
    pub currency: String,
}
