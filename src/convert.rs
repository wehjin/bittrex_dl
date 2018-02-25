use super::*;
use json::JsonValue;

pub fn json_to_csv(json_str: &str, report_type: &ReportType) -> Result<String, AppError> {
    json::parse(json_str)
        .map_err(|_e| AppError::ParseJsonFailed(json_str.to_owned()))
        .and_then(|root_value| extract_vec_from_json_value_with_result(&root_value))
        .and_then(|item_values| json_values_to_csv(&item_values, report_type))
}

fn json_values_to_csv(json_values: &Vec<JsonValue>, report_type: &ReportType) -> Result<String, AppError> {
    let field_names = match report_type {
        &ReportType::Deposits => vec!["Id", "Amount", "Currency", "Confirmations", "LastUpdated", "TxId", "CryptoAddress"],
        &ReportType::Withdrawals => vec!["PaymentUuid", "Currency", "Amount", "Address", "Opened", "Authorized", "PendingPayment", "TxCost", "TxId", "Canceled", "InvalidAddress"],
    };
    json_values.iter()
        .map(|it| json_value_to_csv_row(it, &field_names))
        .collect::<Result<Vec<_>, _>>()
        .map(|it| {
            let mut all_rows = vec![field_names.join(",")];
            all_rows.extend(it);
            all_rows.join("\n")
        })
}

fn json_value_to_csv_row(json_value: &JsonValue, field_names: &Vec<&str>) -> Result<String, AppError> {
    field_names.iter()
        .map(|field_name| json_value_to_csv_string(&json_value[*field_name]))
        .collect::<Result<Vec<_>, _>>()
        .map(|it| it.join(","))
}

fn json_value_to_csv_string(json_value: &JsonValue) -> Result<String, AppError> {
    match json_value {
        &JsonValue::String(ref string) => Ok(string.to_owned()),
        &JsonValue::Short(ref short) => Ok(short.as_str().to_owned()),
        &JsonValue::Number(ref number) => Ok(number.to_string()),
        &JsonValue::Boolean(ref boolean) => Ok(boolean.to_string()),
        _ => Err(AppError::InvalidJsonField(json_value.dump())),
    }
}

fn extract_vec_from_json_value_with_result(result_holder: &JsonValue) -> Result<Vec<JsonValue>, AppError> {
    if let &JsonValue::Array(ref results) = &result_holder["result"] {
        Ok(results.to_owned())
    } else {
        Err(AppError::NoResultInJsonRoot(result_holder.dump()))
    }
}
