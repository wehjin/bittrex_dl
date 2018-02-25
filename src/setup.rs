use std::error::Error;
use super::*;

pub const REPORT_TYPE_DEPOSITS: &str = "deposits";
pub const REPORT_TYPE_WITHDRAWALS: &str = "withdrawals";

pub fn read_report_type() -> Result<ReportType, AppError> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        Err(AppError::MissingReportType)
    } else {
        let arg = args[1].as_str();
        match arg {
            REPORT_TYPE_DEPOSITS => Ok(ReportType::Deposits),
            REPORT_TYPE_WITHDRAWALS => Ok(ReportType::Withdrawals),
            _ => Err(AppError::InvalidReportType(arg.to_owned()))
        }
    }
}

pub fn read_credentials() -> Result<Credentials, AppError> {
    use yaml_rust::{YamlLoader, Yaml};
    read_yaml_string()
        .and_then(|yaml_string| {
            YamlLoader::load_from_str(&yaml_string).map_err(|_e| AppError::LoadYamlFailed(yaml_string))
        })
        .and_then(|it| {
            let doc = &it[0];
            let key_yaml = &doc["api"]["key"];
            let secret_yaml = &doc["api"]["secret"];
            if let (&Yaml::String(ref key), &Yaml::String(ref secret)) = (key_yaml, secret_yaml) {
                Ok(Credentials { key: key.to_owned(), secret: secret.to_owned() })
            } else {
                Err(AppError::MissingApiKeyOrSecret)
            }
        })
}

fn read_yaml_string() -> Result<String, AppError> {
    use std::fs::File;
    use std::io::prelude::*;
    if let Some(path_buf) = std::env::home_dir() {
        let file_path = path_buf.as_path().join(".bittrex_dl.yaml");
        File::open(&file_path)
            .map_err(|e| AppError::OpenFileFailed(file_path, e.description().to_owned()))
            .and_then(|mut it| {
                let mut contents = String::new();
                it.read_to_string(&mut contents)
                    .map_err(|e| AppError::ReadFileFailed(e.description().to_owned()))
                    .map(|_n| contents.to_owned())
            })
    } else {
        Err(AppError::NoHomeDirectory)
    }
}
