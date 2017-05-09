#![feature(specialization)]

extern crate zmq;

mod rate;

use std::vec::Vec;


pub trait Message: Sized {
    fn send(&self, socket: &zmq::Socket) -> Result<(), zmq::Error>;
    fn receive(socket: &zmq::Socket) -> Result<Option<Self>, ReceiveError>;
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

pub use rate::Rate;

