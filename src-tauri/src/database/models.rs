use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: String,
    pub account_type: String,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
    pub category_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub account_id: String,
    pub category_id: Option<String>,
    pub transaction_type: String,
    pub amount_cents: i64,
    pub date: String,
    pub description: String,
    pub notes: String,
    pub recurring_rule_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInput {
    pub account_id: String,
    pub category_id: Option<String>,
    pub transaction_type: String,
    pub amount_cents: i64,
    pub date: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFilter {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub category_id: Option<String>,
    pub account_id: Option<String>,
    pub transaction_type: Option<String>,
    pub search: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub id: String,
    pub category_id: String,
    pub month: String,
    pub amount_cents: i64,
    pub spent_cents: i64,
    pub remaining_cents: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetInput {
    pub category_id: String,
    pub month: String,
    pub amount_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringRule {
    pub id: String,
    pub account_id: String,
    pub category_id: Option<String>,
    pub transaction_type: String,
    pub amount_cents: i64,
    pub description: String,
    pub notes: String,
    pub frequency: String,
    pub interval_count: i64,
    pub start_date: String,
    pub end_date: Option<String>,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringRuleInput {
    pub account_id: String,
    pub category_id: Option<String>,
    pub transaction_type: String,
    pub amount_cents: i64,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub notes: String,
    pub frequency: String,
    #[serde(default = "default_interval")]
    pub interval_count: i64,
    pub start_date: String,
    pub end_date: Option<String>,
    #[serde(default = "default_true")]
    pub active: bool,
}

fn default_interval() -> i64 {
    1
}
fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetProgress {
    pub category_id: String,
    pub category_name: String,
    pub color: String,
    pub budget_cents: i64,
    pub spent_cents: i64,
    pub remaining_cents: i64,
    pub percent_used: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryTotal {
    pub category_id: Option<String>,
    pub category_name: String,
    pub color: String,
    pub amount_cents: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodTotal {
    pub period: String,
    pub income_cents: i64,
    pub expense_cents: i64,
    pub net_cents: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardSummary {
    pub month: String,
    pub income_cents: i64,
    pub expense_cents: i64,
    pub net_cents: i64,
    pub transaction_count: i64,
    pub by_category: Vec<CategoryTotal>,
    pub budgets: Vec<BudgetProgress>,
    pub recent_transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsSummary {
    pub start_date: String,
    pub end_date: String,
    pub income_cents: i64,
    pub expense_cents: i64,
    pub net_cents: i64,
    pub by_category: Vec<CategoryTotal>,
    pub periods: Vec<PeriodTotal>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapData {
    pub accounts: Vec<Account>,
    pub categories: Vec<Category>,
    pub settings: std::collections::HashMap<String, String>,
    pub dashboard: DashboardSummary,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvColumnMapping {
    pub date: String,
    pub amount: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub transaction_type: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvImportOptions {
    pub path: String,
    pub mapping: CsvColumnMapping,
    pub account_id: String,
    pub default_category_id: Option<String>,
    #[serde(default = "default_expense")]
    pub default_transaction_type: String,
    #[serde(default = "default_date_format")]
    pub date_format: String,
    #[serde(default)]
    pub invert_amount: bool,
}

fn default_expense() -> String {
    "expense".into()
}
fn default_date_format() -> String {
    "%Y-%m-%d".into()
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvPreview {
    pub headers: Vec<String>,
    pub rows: Vec<std::collections::HashMap<String, String>>,
    pub total_rows: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    pub imported: usize,
    pub skipped_duplicates: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateResult {
    pub generated: usize,
    pub through_date: String,
}
