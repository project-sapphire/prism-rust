use ::zmq;

use ::{Message, ReceiveError};
use ::{ExchangeQuery, ExchangeRequest, WalletQuery, WalletRequest};


impl Message for ExchangeQuery {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(match self {
            &ExchangeQuery::History(_) => "history",
            &ExchangeQuery::Status(_) => "status",
            &ExchangeQuery::Exchange(_, _, _, _) => "exchange",
        }, flags | zmq::SNDMORE)?;

        match self {
            &ExchangeQuery::History(age) => socket.send_str(&age.to_string(), flags),
            &ExchangeQuery::Status(ref transaction) => socket.send_str(transaction, flags),
            &ExchangeQuery::Exchange(ref to, amount, ref source, ref destination) => {
                socket.send_str(to, flags | zmq::SNDMORE)?;
                socket.send_str(&amount.to_string(), flags | zmq::SNDMORE)?;
                socket.send_str(source, flags | zmq::SNDMORE)?;
                socket.send_str(destination, flags)
            },
        }
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let query = socket.recv_string(flags)??;
        if query.len() == 0 { return Ok(None); }

        match query.as_ref() {
            "history" => Ok(Some(ExchangeQuery::History(socket.recv_string(flags)??.parse()?))),
            "status" => Ok(Some(ExchangeQuery::Status(socket.recv_string(flags)??))),
            "exchange" => Ok(Some(ExchangeQuery::Exchange(
                socket.recv_string(flags)??,
                socket.recv_string(flags)??.parse()?,
                socket.recv_string(flags)??,
                socket.recv_string(flags)??,
                ))),
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

impl Message for WalletQuery {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        let (query, more) = match self {
            &WalletQuery::Balance => ("balance", 0),
            &WalletQuery::Currencies => ("currencies", 0),
            &WalletQuery::Receive => ("receive", 0),
            &WalletQuery::Pay(_, _) => ("pay", zmq::SNDMORE),
        };

        socket.send_str(query, flags | more)?;

        match self {
            &WalletQuery::Pay(amount, ref address) => {
                socket.send_str(&amount.to_string(), flags | zmq::SNDMORE)?;
                socket.send_str(address, flags)
            },
            _ => Ok(()),
        }
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let query = socket.recv_string(flags)??;
        if query.len() == 0 { return Ok(None); }

        match query.as_ref() {
            "balance" => Ok(Some(WalletQuery::Balance)),
            "currencies" => Ok(Some(WalletQuery::Currencies)),
            "receive" => Ok(Some(WalletQuery::Receive)),
            "pay" => Ok(Some(WalletQuery::Pay(
                socket.recv_string(flags)??.parse()?,
                socket.recv_string(flags)??,
                ))),
            _ => Err(ReceiveError::String("invalid operation".to_string()))
        }
    }
}

impl Message for WalletRequest {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(&self.currency, flags | zmq::SNDMORE)?;
        self.query.send(socket, flags)
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let currency = socket.recv_string(flags)??;
        if currency.len() == 0 { return Ok(None); }

        //let query = match ExchangeQuery::receive(socket, flags)? {
        //    Some(x) => x,
        //    None => return Ok(None) // TODO(deox): should probably actually be an error
        //};
        let query = WalletQuery::receive(socket, flags)?.expect("poorly formatted message");

        Ok(Some(WalletRequest {
            query: query,
            currency: currency,
        }))
    }
}