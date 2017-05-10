use super::zmq;
use super::{Message, ReceiveError};

#[derive(Debug)]
pub enum ExchangeQuery {
    HISTORY
}

impl Message for ExchangeQuery {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(match self {
            &ExchangeQuery::HISTORY => "history"
        }, flags)
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let query = socket.recv_string(flags)??;
        if query.len() == 0 { return Ok(None); }

        match query.as_ref() {
            "history" => Ok(Some(ExchangeQuery::HISTORY)),
            _ => Err(ReceiveError::String("invalid operation".to_string()))
        }
    }
}


