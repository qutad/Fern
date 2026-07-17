use std::{collections::HashMap, fs::File};

use chrono::NaiveDate;
use rusqlite::params;
use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

use crate::{
    database::{
        models::{CsvImportOptions, CsvPreview, ImportResult, TransactionFilter},
        queries, Database,
    },
    errors::{AppError, AppResult},
    services::parse_money_cents,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvPreviewInput {
    pub path: String,
    pub limit: Option<usize>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvExportInput {
    pub path: String,
    pub filter: Option<TransactionFilter>,
}

fn open_reader(path: &str) -> AppResult<::csv::Reader<File>> {
    if path.trim().is_empty() {
        return Err(AppError::Validation("path cannot be empty".into()));
    }
    Ok(::csv::ReaderBuilder::new().flexible(true).from_path(path)?)
}

#[tauri::command]
pub fn preview_csv(input: CsvPreviewInput) -> AppResult<CsvPreview> {
    let mut reader = open_reader(&input.path)?;
    let headers = reader
        .headers()?
        .iter()
        .map(String::from)
        .collect::<Vec<_>>();
    let limit = input.limit.unwrap_or(20).min(100);
    let mut rows = Vec::new();
    let mut total_rows = 0;
    for record in reader.records() {
        let record = record?;
        if rows.len() < limit {
            rows.push(
                headers
                    .iter()
                    .cloned()
                    .zip(record.iter().map(String::from))
                    .collect(),
            );
        }
        total_rows += 1;
    }
    Ok(CsvPreview {
        headers,
        rows,
        total_rows,
    })
}

fn column<'a>(
    headers: &::csv::StringRecord,
    record: &'a ::csv::StringRecord,
    name: &str,
) -> AppResult<&'a str> {
    let index = headers
        .iter()
        .position(|h| h == name)
        .ok_or_else(|| AppError::Validation(format!("CSV column not found: {name}")))?;
    Ok(record.get(index).unwrap_or(""))
}

#[tauri::command]
pub fn import_csv(
    database: State<'_, Database>,
    input: CsvImportOptions,
) -> AppResult<ImportResult> {
    let mut reader = open_reader(&input.path)?;
    let headers = reader.headers()?.clone();
    for name in [&input.mapping.date, &input.mapping.amount] {
        if !headers.iter().any(|h| h == name) {
            return Err(AppError::Validation(format!(
                "CSV column not found: {name}"
            )));
        }
    }
    database.with_connection(|connection|{
        let categories=queries::list_categories(connection)?.into_iter().map(|c|(c.name.to_lowercase(),c.id)).collect::<HashMap<_,_>>();let tx=connection.transaction()?;let mut result=ImportResult{imported:0,skipped_duplicates:0,errors:Vec::new()};
        for(row_index,record)in reader.records().enumerate(){let line=row_index+2;let parsed=(||->AppResult<_>{let record=record?;let raw_date=column(&headers,&record,&input.mapping.date)?;let date=NaiveDate::parse_from_str(raw_date.trim(),&input.date_format).map_err(|e|AppError::Validation(format!("invalid date '{raw_date}': {e}")))?.format("%Y-%m-%d").to_string();let mut signed=parse_money_cents(column(&headers,&record,&input.mapping.amount)?)?;if input.invert_amount{signed = -signed;}
            let type_value=input.mapping.transaction_type.as_deref().map(|name|column(&headers,&record,name)).transpose()?.unwrap_or("").trim().to_lowercase();let transaction_type=match type_value.as_str(){"expense"|"debit"|"withdrawal"=>"expense", "income"|"credit"|"deposit"=>"income", ""=>if signed < 0{"expense"}else{input.default_transaction_type.as_str()}, _=>return Err(AppError::Validation(format!("invalid transaction type '{type_value}'")))};
            if !matches!(transaction_type,"expense"|"income"){return Err(AppError::Validation("defaultTransactionType must be expense or income".into()));}
            let description=input.mapping.description.as_deref().map(|n|column(&headers,&record,n)).transpose()?.unwrap_or("").trim().to_string();let notes=input.mapping.notes.as_deref().map(|n|column(&headers,&record,n)).transpose()?.unwrap_or("").trim().to_string();let category_id=input.mapping.category.as_deref().map(|n|column(&headers,&record,n)).transpose()?.and_then(|name|categories.get(&name.trim().to_lowercase()).cloned()).or_else(||input.default_category_id.clone());
            let amount=signed.checked_abs().ok_or_else(||AppError::Validation("amount too large".into()))?;Ok((date,amount,transaction_type.to_string(),description,notes,category_id))})();
            match parsed{Ok((date,amount,kind,description,notes,category_id))=>{let material=format!("{}|{}|{}|{}|{}|{}",input.account_id,date,amount,kind,description,category_id.as_deref().unwrap_or(""));let fingerprint=Uuid::new_v5(&Uuid::NAMESPACE_X500,material.as_bytes()).to_string();let duplicate=tx.query_row("SELECT EXISTS(SELECT 1 FROM import_fingerprints WHERE fingerprint=?1)",[&fingerprint],|r|r.get::<_,bool>(0))?;if duplicate{result.skipped_duplicates+=1;}else{let id=Uuid::new_v4().to_string();let now=queries::now();tx.execute("INSERT INTO transactions(id,account_id,category_id,transaction_type,amount_cents,date,description,notes,created_at,updated_at) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?9)",params![id,input.account_id,category_id,kind,amount,date,description,notes,now])?;tx.execute("INSERT INTO import_fingerprints(fingerprint,transaction_id,imported_at) VALUES(?1,?2,?3)",params![fingerprint,id,now])?;result.imported+=1;}},Err(error)=>result.errors.push(format!("row {line}: {error}"))}
        }tx.commit()?;Ok(result)
    })
}

#[tauri::command]
pub fn export_csv(database: State<'_, Database>, input: CsvExportInput) -> AppResult<usize> {
    if input.path.trim().is_empty() {
        return Err(AppError::Validation("path cannot be empty".into()));
    }
    if let Some(parent) = std::path::Path::new(&input.path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    database.with_connection(|connection| {
        let transactions = queries::list_transactions(
            connection,
            &TransactionFilter {
                limit: Some(5000),
                ..input.filter.unwrap_or_default()
            },
        )?;
        let categories = queries::list_categories(connection)?
            .into_iter()
            .map(|c| (c.id, c.name))
            .collect::<HashMap<_, _>>();
        let mut writer = ::csv::Writer::from_path(&input.path)?;
        writer.write_record([
            "date",
            "description",
            "category",
            "type",
            "amountCents",
            "notes",
            "accountId",
        ])?;
        for transaction in &transactions {
            writer.write_record([
                transaction.date.as_str(),
                transaction.description.as_str(),
                transaction
                    .category_id
                    .as_ref()
                    .and_then(|id| categories.get(id))
                    .map(String::as_str)
                    .unwrap_or(""),
                transaction.transaction_type.as_str(),
                &transaction.amount_cents.to_string(),
                transaction.notes.as_str(),
                transaction.account_id.as_str(),
            ])?;
        }
        writer.flush()?;
        Ok(transactions.len())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preview_reads_headers_and_limit() {
        let path = std::env::temp_dir().join(format!("fern-{}.csv", Uuid::new_v4()));
        std::fs::write(&path, "Date,Amount\n2026-01-01,12.00\n2026-01-02,4.00\n").unwrap();
        let preview = preview_csv(CsvPreviewInput {
            path: path.to_string_lossy().into(),
            limit: Some(1),
        })
        .unwrap();
        assert_eq!(preview.headers, ["Date", "Amount"]);
        assert_eq!(preview.rows.len(), 1);
        assert_eq!(preview.total_rows, 2);
        std::fs::remove_file(path).unwrap();
    }
}
