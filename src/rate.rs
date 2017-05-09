use std::collections::HashMap;

use super::zmq;
use super::{Message, ReceiveError};


#[derive(Clone, Debug)]
pub struct Rate {
    pub currency: String,
    pub values: HashMap<String, f64>,
}

impl Message for Rate {
    fn send(&self, socket: &zmq::Socket) -> Result<(), zmq::Error> {
        socket.send_str(&self.currency, zmq::SNDMORE)?;

        for (currency, value) in &self.values {
            socket.send_str(&currency, zmq::SNDMORE)?;
            socket.send_str(&value.to_string(), zmq::SNDMORE)?;
        }

        socket.send(b"", 0)?;
        Ok(())
    }

    fn receive(socket: &zmq::Socket) -> Result<Option<Self>, ReceiveError>
    {
        let currency = socket.recv_string(0)??;
        if currency.len() == 0 { return Ok(None); }

        let mut values = HashMap::new();

        loop {
            let other_currency = socket.recv_string(0)??;
            if other_currency.len() == 0 { break; }

            let value = socket.recv_string(0)??.parse()?;
            values.insert(other_currency, value);
        }

        Ok(Some(Self {
            currency: currency,
            values: values
        }))
    }
}
