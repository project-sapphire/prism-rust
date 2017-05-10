use std::collections::HashMap;
use std::time::Instant;

use super::zmq;
use super::{Message, ReceiveError};


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

impl Message for Rate {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(&self.timestamp.to_string(), flags | zmq::SNDMORE);

        for (currency, value) in &self.values {
            socket.send_str(&currency, flags | zmq::SNDMORE)?;
            socket.send_str(&value.to_string(), flags | zmq::SNDMORE)?;
        }

        socket.send(b"", flags)?;
        Ok(())
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let mut values = HashMap::new();

        let timestamp = socket.recv_string(flags)??; 
        if timestamp.len() == 0 { return Ok(None); }
        
        let timestamp = timestamp.parse()?;

        loop {
            let other_currency = socket.recv_string(flags)??;
            if other_currency.len() == 0 { break; }

            let value = socket.recv_string(flags)??.parse()?;
            values.insert(other_currency, value);
        }

        Ok(Some(Self {
            timestamp: timestamp,
            values: values,
        }))
    }
}

impl Message for RateUpdate {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(&self.currency, flags | zmq::SNDMORE)?;
        socket.send_str(&self.exchange, flags | zmq::SNDMORE)?;
        self.rate.send(socket, 0)?;

        Ok(())
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let currency = socket.recv_string(flags)??;
        if currency.len() == 0 { return Ok(None); }

        let exchange = socket.recv_string(flags)??;

        let rate = Rate::receive(socket, flags)?.unwrap();

        Ok(Some(RateUpdate {
            currency: currency,
            exchange: exchange,
            rate: rate,
        }))
    }
}

