use std::collections::HashMap;

use super::zmq;
use super::{Message, ReceiveError};


#[derive(Clone, Debug)]
pub struct Rate {
    pub currency: String,
    pub values: HashMap<String, f64>,
}

impl Message for Rate {
    fn send(&self, socket: &zmq::Socket) {
        socket.send_str(&self.currency, zmq::SNDMORE);
        for (currency, value) in &self.values {
            socket.send_str(&currency, zmq::SNDMORE);
            socket.send_str(&value.to_string(), zmq::SNDMORE);
        }

        socket.send(b"", 0);
    }

    fn receive(socket: &zmq::Socket) -> Result<Option<Self>, ReceiveError>
    {
        let currency = socket.recv_string(0)?;
        Ok(Some(Self {
            currency: "abc".to_string(),
            values: HashMap::new()
        }))
    }
}
