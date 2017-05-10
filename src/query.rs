use super::{Message, ReceiveError};

#[derive(Debug, Clone)]
pub enum ExchangeQuery {
    History(u64)
}

#[derive(Debug, Clone)]
pub struct ExchangeRequest {
    pub query: ExchangeQuery,
    pub exchange: String,
    pub currency: String
}
