use rusqlite::{params, Connection};

use crate::errors::AppResult;

pub fn migrate(connection: &Connection) -> AppResult<()> {
    connection.execute_batch(
        "BEGIN;
         CREATE TABLE IF NOT EXISTS schema_migrations (
           version INTEGER PRIMARY KEY,
           applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );
         CREATE TABLE IF NOT EXISTS accounts (
           id TEXT PRIMARY KEY,
           name TEXT NOT NULL,
           account_type TEXT NOT NULL DEFAULT 'cash',
           currency TEXT NOT NULL DEFAULT 'USD',
           created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );
         CREATE TABLE IF NOT EXISTS categories (
           id TEXT PRIMARY KEY,
           name TEXT NOT NULL UNIQUE,
           color TEXT NOT NULL,
           icon TEXT,
           category_type TEXT NOT NULL DEFAULT 'expense' CHECK(category_type IN ('expense','income','both')),
           created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );
         CREATE TABLE IF NOT EXISTS transactions (
           id TEXT PRIMARY KEY,
           account_id TEXT NOT NULL REFERENCES accounts(id),
           category_id TEXT REFERENCES categories(id) ON DELETE SET NULL,
           transaction_type TEXT NOT NULL CHECK(transaction_type IN ('expense','income')),
           amount_cents INTEGER NOT NULL CHECK(amount_cents >= 0),
           date TEXT NOT NULL,
           description TEXT NOT NULL DEFAULT '',
           notes TEXT NOT NULL DEFAULT '',
           recurring_rule_id TEXT,
           created_at TEXT NOT NULL,
           updated_at TEXT NOT NULL
         );
         CREATE INDEX IF NOT EXISTS idx_transactions_date ON transactions(date DESC);
         CREATE INDEX IF NOT EXISTS idx_transactions_category_date ON transactions(category_id, date);
         CREATE TABLE IF NOT EXISTS budgets (
           id TEXT PRIMARY KEY,
           category_id TEXT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
           month TEXT NOT NULL,
           amount_cents INTEGER NOT NULL CHECK(amount_cents >= 0),
           created_at TEXT NOT NULL,
           updated_at TEXT NOT NULL,
           UNIQUE(category_id, month)
         );
         CREATE TABLE IF NOT EXISTS recurring_rules (
           id TEXT PRIMARY KEY,
           account_id TEXT NOT NULL REFERENCES accounts(id),
           category_id TEXT REFERENCES categories(id) ON DELETE SET NULL,
           transaction_type TEXT NOT NULL CHECK(transaction_type IN ('expense','income')),
           amount_cents INTEGER NOT NULL CHECK(amount_cents >= 0),
           description TEXT NOT NULL DEFAULT '',
           notes TEXT NOT NULL DEFAULT '',
           frequency TEXT NOT NULL CHECK(frequency IN ('daily','weekly','monthly','yearly')),
           interval_count INTEGER NOT NULL DEFAULT 1 CHECK(interval_count > 0),
           start_date TEXT NOT NULL,
           end_date TEXT,
           active INTEGER NOT NULL DEFAULT 1,
           created_at TEXT NOT NULL,
           updated_at TEXT NOT NULL
         );
         CREATE TABLE IF NOT EXISTS recurring_occurrences (
           id TEXT PRIMARY KEY,
           recurring_rule_id TEXT NOT NULL REFERENCES recurring_rules(id) ON DELETE CASCADE,
           occurrence_date TEXT NOT NULL,
           transaction_id TEXT NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
           created_at TEXT NOT NULL,
           UNIQUE(recurring_rule_id, occurrence_date)
         );
         CREATE TABLE IF NOT EXISTS settings (
           key TEXT PRIMARY KEY,
           value TEXT NOT NULL,
           updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );
         CREATE TABLE IF NOT EXISTS import_fingerprints (
           fingerprint TEXT PRIMARY KEY,
           transaction_id TEXT NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
           imported_at TEXT NOT NULL
         );
         INSERT OR IGNORE INTO schema_migrations(version) VALUES (1);
         COMMIT;",
    )?;

    connection.execute(
        "INSERT OR IGNORE INTO accounts(id,name,account_type,currency) VALUES (?1,'Main account','checking','USD')",
        ["account-default"],
    )?;
    let categories = [
        (
            "category-food",
            "Food & Dining",
            "#e76f51",
            "utensils",
            "expense",
        ),
        (
            "category-transport",
            "Transport",
            "#2a9d8f",
            "car",
            "expense",
        ),
        ("category-housing", "Housing", "#457b9d", "house", "expense"),
        (
            "category-shopping",
            "Shopping",
            "#9b5de5",
            "shopping-bag",
            "expense",
        ),
        ("category-health", "Health", "#f15bb5", "heart", "expense"),
        (
            "category-entertainment",
            "Entertainment",
            "#f4a261",
            "clapperboard",
            "expense",
        ),
        ("category-income", "Income", "#43aa8b", "wallet", "income"),
        ("category-other", "Other", "#6c757d", "circle", "both"),
    ];
    for category in categories {
        connection.execute(
            "INSERT OR IGNORE INTO categories(id,name,color,icon,category_type) VALUES (?1,?2,?3,?4,?5)",
            params![category.0, category.1, category.2, category.3, category.4],
        )?;
    }
    connection.execute(
        "INSERT OR IGNORE INTO settings(key,value) VALUES ('currency','USD'),('weekStart','monday')",
        [],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_is_idempotent_and_seeds_data() {
        let connection = Connection::open_in_memory().unwrap();
        migrate(&connection).unwrap();
        migrate(&connection).unwrap();
        let counts: (i64, i64) = connection
            .query_row(
                "SELECT (SELECT COUNT(*) FROM accounts), (SELECT COUNT(*) FROM categories)",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(counts, (1, 8));
    }
}
