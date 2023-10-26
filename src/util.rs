use anyhow::anyhow;
use lnurl::{pay::PayResponse, Builder};

#[allow(dead_code)]
pub async fn payment_request_callback(username: String) -> anyhow::Result<PayResponse> {
    let client = Builder::default().build_async().unwrap();

    let info = client
        .make_request(format!("https://bitcoinbay.club/.well-known/lnurlp/{}", username).as_str())
        .await?;

    match info {
        lnurl::LnUrlResponse::LnUrlPayResponse(pay_res) => Ok(pay_res),
        lnurl::LnUrlResponse::LnUrlWithdrawResponse(_) => Err(anyhow!("Not pay response.")),
        lnurl::LnUrlResponse::LnUrlChannelResponse(_) => Err(anyhow!("Not pay response.")),
    }
}
