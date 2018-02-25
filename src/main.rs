extern crate yaml_rust;
extern crate chrono;
extern crate hmac;
extern crate sha2;
extern crate rustc_serialize as serialize;
extern crate reqwest;
#[macro_use]
extern crate hyper;
extern crate json;

mod setup;
mod api;
mod convert;

use std::path::PathBuf;

fn main() {
    std::process::exit({
        let result = setup::read_report_type()
            .and_then(|report_type| {
                setup::read_credentials()
                    .and_then(|it| api::fetch_json(&it, &report_type))
                    .and_then(|it| convert::json_to_csv(&it, &report_type))
            });
        match result {
            Ok(it) => {
                println!("{}", it);
                0
            }
            Err(e) => {
                let message = match e {
                    AppError::MissingReportType => format!("Missing report-type.  Specify '{}' or '{}'.",
                                                           setup::REPORT_TYPE_DEPOSITS, setup::REPORT_TYPE_WITHDRAWALS),
                    _ => format!("{:?}", e)
                };
                eprintln!("\nERROR\n=====\n{}", message);
                1
            }
        }
    })
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
    MissingReportType,
    InvalidReportType(String),
    ParseJsonFailed(String),
    NoResultInJsonRoot(String),
    InvalidJsonField(String),
}


