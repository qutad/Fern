pub mod backup;
pub mod budgets;
pub mod csv;
pub mod recurring;
pub mod settings;
pub mod transactions;

use chrono::Datelike;
use serde::Deserialize;
use tauri::State;

use crate::{
    database::{
        models::{AnalyticsSummary, BootstrapData},
        queries, Database,
    },
    errors::AppResult,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardInput {
    pub month: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsInput {
    pub start_date: String,
    pub end_date: String,
    pub group_by: Option<String>,
}

#[tauri::command]
pub fn get_bootstrap_data(database: State<'_, Database>) -> AppResult<BootstrapData> {
    database.with_connection(|connection| {
        let today = chrono::Local::now().date_naive();
        let month = format!("{:04}-{:02}", today.year(), today.month());
        Ok(BootstrapData {
            accounts: queries::list_accounts(connection)?,
            categories: queries::list_categories(connection)?,
            settings: queries::settings(connection)?,
            dashboard: queries::dashboard(connection, &month)?,
        })
    })
}

#[tauri::command]
pub fn get_dashboard_summary(
    database: State<'_, Database>,
    input: Option<DashboardInput>,
) -> AppResult<crate::database::models::DashboardSummary> {
    let month = input
        .and_then(|v| v.month)
        .unwrap_or_else(|| chrono::Local::now().format("%Y-%m").to_string());
    database.with_connection(|connection| queries::dashboard(connection, &month))
}

#[tauri::command]
pub fn get_analytics_summary(
    database: State<'_, Database>,
    input: AnalyticsInput,
) -> AppResult<AnalyticsSummary> {
    database.with_connection(|connection| {
        queries::analytics(
            connection,
            &input.start_date,
            &input.end_date,
            input.group_by.as_deref().unwrap_or("month"),
        )
    })
}
