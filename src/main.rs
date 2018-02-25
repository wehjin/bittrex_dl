extern crate yaml_rust;
extern crate chrono;
extern crate hmac;
extern crate sha2;
extern crate rustc_serialize as serialize;
extern crate reqwest;
#[macro_use]
extern crate hyper;

mod setup;

use std::path::PathBuf;

fn main() {
    let result = setup::read_credentials()
        .and_then(|it| api::fetch_withdrawals(&it));
    match result {
        Ok(it) => println!("{}", it),
        Err(e) => eprintln!("\nERROR\n=====\n{:?}", e),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Credentials {
    pub key: String,
    pub secret: String,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AppError {
    NoHomeDirectory,
    OpenFileFailed(PathBuf, String),
    ReadFileFailed(String),
    LoadYamlFailed(String),
    MissingApiKeyOrSecret,
    RequestFailed(String),
}


mod api {
    use super::*;
    use chrono::prelude::*;
    use serialize::hex::ToHex;

    pub fn fetch_deposits(credentials: &Credentials) -> Result<String, AppError> {
        api::Request::new(api::DEPOSITS_URI_STRING, credentials).fetch()
    }

    pub fn fetch_withdrawals(credentials: &Credentials) -> Result<String, AppError> {
        api::Request::new(api::WITHDRAWALS_URI_STRING, credentials).fetch()
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
}