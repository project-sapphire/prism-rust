use ::zmq;

use ::{Message, ReceiveError};
use ::{ExchangeQuery, ExchangeRequest};


impl Message for ExchangeQuery {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(match self {
            &ExchangeQuery::History(_) => "history"
        }, flags | zmq::SNDMORE)?;

        match(self) {
            &ExchangeQuery::History(age) => socket.send_str(&age.to_string(), flags)
        }
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let query = socket.recv_string(flags)??;
        if query.len() == 0 { return Ok(None); }

        match query.as_ref() {
            "history" => Ok(Some(ExchangeQuery::History(socket.recv_string(flags)??.parse()?))),
            _ => Err(ReceiveError::String("invalid operation".to_string()))
        }
    }
}

impl Message for ExchangeRequest {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(&self.exchange, flags | zmq::SNDMORE)?;
        socket.send_str(&self.currency, flags | zmq::SNDMORE)?;
        self.query.send(socket, flags)
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let exchange = socket.recv_string(flags)??;
        if exchange.len() == 0 { return Ok(None); }

        let currency = socket.recv_string(flags)??;

        //let query = match ExchangeQuery::receive(socket, flags)? {
        //    Some(x) => x,
        //    None => return Ok(None) // TODO(deox): should probably actually be an error
        //};
        let query = ExchangeQuery::receive(socket, flags)?.expect("poorly formatted message");

        Ok(Some(ExchangeRequest {
            query: query,
            exchange: exchange,
            currency: currency,
        }))
    }
}