use zmq;
use Message;

pub struct Exchange {
    subscriber: zmq::Socket,
    request: zmq::Socket,
}

impl Exchange {
    pub fn new(context: &zmq::Context, subscribe_addr: &str, request_addr: &str) -> Result<Exchange, zmq::Error> {
        let subscriber = context.socket(zmq::SUB)?;
        let request = context.socket(zmq::REQ)?;

        subscriber.connect(subscribe_addr)?;
        request.connect(request_addr)?;

        Ok(Exchange {
            subscriber: subscriber,
            request: request
        })
    }

    pub fn subscribe(&self, currency: &str) -> Result<(), zmq::Error> {
        self.subscriber.set_subscribe(currency.as_bytes())
    }

    pub fn unsubscribe(&self, currency: &str) -> Result<(), zmq::Error> {
        self.subscriber.set_unsubscribe(currency.as_bytes())
    }

    pub fn receive_rate_update(&self) -> Result<Option<::RateUpdate>, ::ReceiveError> {
        ::RateUpdate::receive(&self.subscriber, 0)
    }

    pub fn exchange(&self, exchange: &str, from: &str, to: &str, amount: f64, source: &str, destination: &str) -> Result<Option<::Invoice>, ::ReceiveError> {
        ::ExchangeRequest {
            exchange: exchange.to_string(),
            currency: from.to_string(),
            query: ::ExchangeQuery::Exchange(to.to_string(), amount, source.to_string(), destination.to_string()),
        }.send(&self.request, 0)?;
        ::Invoice::receive(&self.request, 0)
    }
}