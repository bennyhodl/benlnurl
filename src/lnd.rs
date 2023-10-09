use tonic_lnd::{in_mem_connect, Client};

pub struct LndClient(Client);

impl LndClient {
    pub async fn new(
        address: String,
        cert_hex: String,
        macaroon_hex: String,
    ) -> anyhow::Result<LndClient> {
        let client = in_mem_connect(address, cert_hex, macaroon_hex).await?;

        Ok(LndClient(client))
    }

    pub async fn create_invoice(&mut self, memo: String) -> String {
        let invoice_request = tonic_lnd::lnrpc::InvoiceRequest {
            memo: format!("Pay {} becuase they use benlnurl", memo),
            private: false,
            value_msat: 100000,
            is_keysend: false,
            is_amp: false,
        };

        let response = self
            .0
            .lightning()
            .add_invoice(invoice_request)
            .await
            .expect("BAD!");

        response.into_inner().payment_request
    }
}
