use tonic_lnd::{in_mem_connect, Client, ConnectError};

pub struct LndClient(Client);

impl LndClient {
    pub async fn new(
        address: &str,
        cert_hex: &str,
        macaroon_hex: &str,
    ) -> anyhow::Result<LndClient> {
        let client = in_mem_connect(
            address.to_string(),
            cert_hex.to_string(),
            macaroon_hex.to_string(),
        )
        .await?;

        Ok(LndClient(client))
    }

    pub async fn create_invoice(&mut self) -> String {
        let invoice_request = tonic_lnd::lnrpc::InvoiceRequest {
            memo: "ben".to_string(),
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
