mod commands;
mod database;
mod errors;
mod services;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            let path = app.path().app_data_dir()?.join("fern.sqlite3");
            let database = database::Database::open(path)?;
            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            if let Err(error) = database.with_connection(|connection| {
                commands::recurring::generate(connection, &today).map(|_| ())
            }) {
                log::warn!("could not generate recurring transactions: {error}");
            }
            app.manage(database);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_bootstrap_data,
            commands::get_dashboard_summary,
            commands::get_analytics_summary,
            commands::transactions::list_transactions,
            commands::transactions::create_transaction,
            commands::transactions::update_transaction,
            commands::transactions::delete_transaction,
            commands::budgets::list_budgets,
            commands::budgets::upsert_budget,
            commands::budgets::delete_budget,
            commands::recurring::list_recurring_rules,
            commands::recurring::create_recurring_rule,
            commands::recurring::update_recurring_rule,
            commands::recurring::delete_recurring_rule,
            commands::recurring::generate_recurring_due,
            commands::csv::preview_csv,
            commands::csv::import_csv,
            commands::csv::export_csv,
            commands::settings::update_settings,
            commands::backup::create_backup,
            commands::backup::restore_backup,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Fern");
}
