use zmq;
use Message;

pub struct Wallet {
    request: zmq::Socket,
}

impl Wallet {
    pub fn new(context: &zmq::Context, request_addr: &str) -> Result<Wallet, zmq::Error> {
        let request = context.socket(zmq::REQ)?;
        request.connect(request_addr)?;

        Ok(Wallet {
            request: request
        })
    }

    pub fn receive(&self, currency: &str) -> Result<Option<String>, ::ReceiveError> {
        ::WalletRequest {
            currency: currency.to_string(),
            query: ::WalletQuery::Receive,
        }.send(&self.request, 0)?;
        String::receive(&self.request, 0)
    }

    pub fn pay(&self, invoice: &::Invoice) -> Result<Option<String>, ::ReceiveError> {
        ::WalletRequest {
            currency: invoice.currency.clone(),
            query: ::WalletQuery::Pay(invoice.amount, invoice.address.clone()),
        }.send(&self.request, 0)?;
        String::receive(&self.request, 0)
    }
}
