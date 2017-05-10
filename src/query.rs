use super::{Message, ReceiveError};

#[derive(Debug, Clone)]
pub enum ExchangeQuery {
    History
}

#[derive(Debug, Clone)]
pub struct ExchangeRequest {
    pub query: ExchangeQuery,
    pub exchange: String,
    pub currency: String
}

