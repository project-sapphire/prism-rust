use ::zmq;

use ::{Message, ReceiveError};
use ::{ExchangeQuery, ExchangeRequest};


impl Message for ::ExchangeQuery {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(match self {
            &::ExchangeQuery::History => "history"
        }, flags)
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let query = socket.recv_string(flags)??;
        if query.len() == 0 { return Ok(None); }

        match query.as_ref() {
            "history" => Ok(Some(ExchangeQuery::History)),
            _ => Err(ReceiveError::String("invalid operation".to_string()))
        }
    }
}

impl Message for ExchangeRequest {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        self.query.send(socket, flags | zmq::SNDMORE)?;
        socket.send_str(&self.exchange, flags | zmq::SNDMORE)?;
        socket.send_str(&self.currency, flags)
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let query = match ExchangeQuery::receive(socket, flags)? {
            Some(x) => x,
            None => return Ok(None)
        };

        Ok(Some(ExchangeRequest {
            query: query,
            exchange: socket.recv_string(flags)??,
            currency: socket.recv_string(flags)??,
        }))
    }
}