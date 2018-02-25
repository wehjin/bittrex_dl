use super::*;
use chrono::prelude::*;
use serialize::hex::ToHex;

pub fn fetch_json(credentials: &Credentials, report_type: &ReportType) -> Result<String, AppError> {
    let uri = match report_type {
        &ReportType::Deposits => DEPOSITS_URI_STRING,
        &ReportType::Withdrawals => WITHDRAWALS_URI_STRING,
    };
    api::Request::new(uri, credentials).fetch()
}

const DEPOSITS_URI_STRING: &str = "https://bittrex.com/api/v1.1/account/getdeposithistory";
const WITHDRAWALS_URI_STRING: &str = "https://bittrex.com/api/v1.1/account/getwithdrawalhistory";

#[derive(Clone, PartialEq, Debug)]
struct Request {
    uri: String,
    signature: String,
}

impl Request {
    fn new(uri_string: &str, credentials: &Credentials) -> Self {
        let nonce = Utc::now().timestamp();
        let uri_string = format!("{}?apikey={}&nonce={}", uri_string, credentials.key, nonce);

        use sha2::Sha512;
        use hmac::{Hmac, Mac};
        let mut mac = Hmac::<Sha512>::new(credentials.secret.as_bytes()).unwrap();
        mac.input(uri_string.as_bytes());
        let signature = mac.result().code().to_hex();
        Request { uri: uri_string, signature }
    }

    fn fetch(&self) -> Result<String, AppError> {
        use reqwest::Client;
        header! { (ApiSignHeader, "apisign") => [String] }
        Client::new()
            .get(&self.uri)
            .header(ApiSignHeader(self.signature.to_owned()))
            .send()
            .and_then(|mut it| it.text())
            .map_err(|_e| AppError::RequestFailed(self.uri.to_owned()))
    }
}
