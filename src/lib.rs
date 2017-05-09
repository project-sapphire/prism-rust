extern crate zmq;

mod rate;

pub trait Message: Sized {
    fn send(&self, socket: &zmq::Socket);
    fn receive(socket: &zmq::Socket) -> Result<Option<Self>, ReceiveError>;
}

#[derive(Debug)]
pub enum ReceiveError {
    String(String),
    ZeroMQ(zmq::Error),
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

pub use rate::Rate;

