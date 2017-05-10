#![feature(specialization)]

extern crate zmq;

mod rate;
mod query;

use std::vec::Vec;


pub trait Message: Sized {
    fn send(&self, socket: &zmq::Socket, flags: i32) -> Result<(), zmq::Error>;
    fn receive(socket: &zmq::Socket, flags: i32) -> Result<Option<Self>, ReceiveError>;
}

#[derive(Debug)]
pub enum ReceiveError {
    String(String),
    ZeroMQ(zmq::Error),
    Decode(Vec<u8>),
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
pub use query::{ExchangeQuery};

