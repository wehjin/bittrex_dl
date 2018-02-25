extern crate yaml_rust;
extern crate chrono;
extern crate hmac;
extern crate sha2;
extern crate rustc_serialize as serialize;
extern crate reqwest;
#[macro_use]
extern crate hyper;

mod setup;
mod api;

use std::path::PathBuf;

fn main() {
    let result = setup::read_report_type()
        .and_then(|report_type| {
            setup::read_credentials()
                .and_then(|it| api::fetch_json(&it, &report_type))
        });
    match result {
        Ok(it) => println!("{}", it),
        Err(e) => eprintln!("\nERROR\n=====\n{:?}", e),
    }
}


#[derive(Clone, PartialEq, Debug)]
pub enum ReportType {
    Deposits,
    Withdrawals,
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
    MissingOrInvalidReportType(String),
}


