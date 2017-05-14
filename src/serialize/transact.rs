use ::zmq;

use ::{Message, ReceiveError};
use ::{Invoice};


impl Message for Invoice {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(&self.address, flags | zmq::SNDMORE)?;
        socket.send_str(&self.currency, flags | zmq::SNDMORE)?;
        socket.send_str(&self.amount.to_string(), flags)
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let address = socket.recv_string(flags)??;
        if address.len() == 0 { return Ok(None); }

        let currency = socket.recv_string(flags)??;

        let amount = socket.recv_string(flags)??.parse()?;

        Ok(Some(Invoice {
            address: address,
            currency: currency,
            amount: amount,
        }))
    }
}