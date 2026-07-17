use chrono::{Datelike, Duration, Months, NaiveDate};

use crate::errors::{AppError, AppResult};

pub fn recurrence_dates(
    start: NaiveDate,
    end: Option<NaiveDate>,
    through: NaiveDate,
    frequency: &str,
    interval: i64,
) -> AppResult<Vec<NaiveDate>> {
    if interval <= 0 {
        return Err(AppError::Validation(
            "intervalCount must be positive".into(),
        ));
    }
    let limit = end.map_or(through, |date| date.min(through));
    let mut dates = Vec::new();
    for index in 0..100_000_i64 {
        let amount = index
            .checked_mul(interval)
            .ok_or_else(|| AppError::Date("recurrence overflow".into()))?;
        let date = match frequency {
            "daily" => start.checked_add_signed(Duration::days(amount)),
            "weekly" => start.checked_add_signed(Duration::weeks(amount)),
            "monthly" => add_months_clamped(start, amount),
            "yearly" => add_months_clamped(
                start,
                amount
                    .checked_mul(12)
                    .ok_or_else(|| AppError::Date("recurrence overflow".into()))?,
            ),
            _ => {
                return Err(AppError::Validation(
                    "frequency must be daily, weekly, monthly, or yearly".into(),
                ))
            }
        }
        .ok_or_else(|| AppError::Date("recurrence date out of range".into()))?;
        if date > limit {
            break;
        }
        dates.push(date);
    }
    Ok(dates)
}

fn add_months_clamped(date: NaiveDate, months: i64) -> Option<NaiveDate> {
    let months = u32::try_from(months).ok()?;
    let first = NaiveDate::from_ymd_opt(date.year(), date.month(), 1)?
        .checked_add_months(Months::new(months))?;
    let next = first.checked_add_months(Months::new(1))?;
    let last_day = (next - Duration::days(1)).day();
    NaiveDate::from_ymd_opt(first.year(), first.month(), date.day().min(last_day))
}

pub fn parse_money_cents(value: &str) -> AppResult<i64> {
    let cleaned = value.trim().replace([',', '$', '€', '£'], "");
    if cleaned.is_empty() {
        return Err(AppError::Validation("empty amount".into()));
    }
    let negative = cleaned.starts_with('-') || (cleaned.starts_with('(') && cleaned.ends_with(')'));
    let unsigned = cleaned.trim_start_matches('-').trim_matches(['(', ')']);
    let mut parts = unsigned.split('.');
    let whole: i64 = parts.next().unwrap_or("0").parse()?;
    let fraction = parts.next().unwrap_or("");
    if parts.next().is_some() || fraction.len() > 2 {
        return Err(AppError::Validation(format!("invalid amount: {value}")));
    }
    let fraction: i64 = match fraction.len() {
        0 => 0,
        1 => fraction.parse::<i64>()? * 10,
        _ => fraction.parse()?,
    };
    let cents = whole
        .checked_mul(100)
        .and_then(|v| v.checked_add(fraction))
        .ok_or_else(|| AppError::Validation("amount too large".into()))?;
    Ok(if negative { -cents } else { cents })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monthly_schedule_preserves_end_of_month_intent() {
        let dates = recurrence_dates(
            NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            None,
            NaiveDate::from_ymd_opt(2024, 4, 30).unwrap(),
            "monthly",
            1,
        )
        .unwrap();
        assert_eq!(
            dates.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["2024-01-31", "2024-02-29", "2024-03-31", "2024-04-30"]
        );
    }

    #[test]
    fn parses_common_money_formats() {
        assert_eq!(parse_money_cents("$1,234.5").unwrap(), 123450);
        assert_eq!(parse_money_cents("(12.34)").unwrap(), -1234);
    }
}
