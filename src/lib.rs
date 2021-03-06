#![feature(specialization)]

extern crate zmq;

mod rate;
mod query;
mod transact;
mod serialize;

pub mod exchange;
pub mod wallet;

use std::vec::Vec;

use std::str::FromStr;
use std::string::ToString;


pub trait Message: Sized {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error>;
    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError>;
}

#[derive(Debug, Clone)]
pub enum ReceiveError {
    String(String),
    ZeroMQ(zmq::Error),
    Decode(Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum Error {
    Receive(ReceiveError),
}

impl From<String> for ReceiveError {
    fn from(e: String) -> ReceiveError {
        ReceiveError::String(e)
    }
}

impl From<zmq::Error> for ReceiveError {
    fn from(e: zmq::Error) -> ReceiveError {
        ReceiveError::ZeroMQ(e)
    }
}

impl From<Vec<u8>> for ReceiveError {
    fn from(e: Vec<u8>) -> ReceiveError {
        ReceiveError::Decode(e)
    }
}

impl<T: ToString> From<T> for ReceiveError {
    default fn from(e: T) -> ReceiveError {
        ReceiveError::String(e.to_string())
    }
}

impl From<ReceiveError> for Error {
    fn from(e: ReceiveError) -> Error {
        Error::Receive(e)
    }
}

/*impl<T: ToString + FromStr> Message for T {
    default fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(&self.to_string(), flags)
    }

    default fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        Self::from_str(&socket.recv_string()??)?
    }
}*/

impl Message for String {
    default fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(self, flags)
    }

    default fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        Ok(Some(socket.recv_string(flags)??))
    }
}

impl Message for f64 {
    default fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        socket.send_str(&self.to_string(), flags)
    }

    default fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        Ok(Some(Self::from_str(&socket.recv_string(flags)??)?))
    }
}

impl<T: Message> Message for Vec<T> {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error> {
        for message in self {
            message.send(socket, flags | zmq::SNDMORE)?;
        }
        socket.send(b"", flags)
    }

    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError> {
        let mut vec = Vec::new();

        while let Some(message) = T::receive(socket, flags)? {
            vec.push(message);
        }

        Ok(Some(vec))
    }
}

pub use rate::{Rate, RateUpdate};
pub use query::{ExchangeQuery, ExchangeRequest};
pub use query::{WalletQuery, WalletRequest};
pub use transact::{Invoice};
