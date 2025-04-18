use chrono::{TimeZone, Utc};
use dioxus_translate::Language;

pub fn format_prev_time(timestamp: i64) -> String {
    let now = Utc::now();

    let target_time = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .unwrap_or(Utc::now());

    let duration = now.signed_duration_since(target_time);

    if duration.num_seconds() < 60 {
        return format!("{}초 전", duration.num_seconds());
    } else if duration.num_minutes() < 60 {
        return format!("{}분 전", duration.num_minutes());
    } else if duration.num_hours() < 24 {
        return format!("{}시간 전", duration.num_hours());
    } else if duration.num_days() < 30 {
        return format!("{}일 전", duration.num_days());
    } else if duration.num_days() < 365 {
        let months = duration.num_days() / 30;
        return format!("{}개월 전", months);
    } else {
        let years = duration.num_days() / 365;
        return format!("{}년 전", years);
    }
}

pub fn formatted_timestamp(lang: Language, timestamp: i64) -> String {
    let datetime: chrono::DateTime<Utc> = if timestamp > 1_000_000_000_000_000_000 {
        Utc.timestamp_nanos(timestamp)
    } else if timestamp > 1_000_000_000_000_000 {
        Utc.timestamp_micros(timestamp)
            .single()
            .expect("Invalid timestamp")
    } else if timestamp > 1_000_000_000_000 {
        Utc.timestamp_millis_opt(timestamp)
            .single()
            .expect("Invalid timestamp")
    } else {
        Utc.timestamp_opt(timestamp, 0)
            .single()
            .expect("Invalid timestamp")
    };
    match lang {
        // Language::Ko => datetime.format("%-m월 %-d일 %Y년").to_string(),
        Language::Ko => datetime.format(" %Y년 %-m월 %-d일").to_string(),
        Language::En => datetime.format("%-m. %-d. %Y").to_string(),
    }
}

pub fn formatted_timestamp_to_sec(timestamp: i64) -> String {
    let datetime = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .map(|datetime| datetime.format("%Y년 %m월 %d일 %H:%M").to_string());

    datetime.unwrap_or_default()
}

pub fn format_time_range(started_at: i64, ended_at: i64) -> String {
    let start_date = Utc
        .timestamp_opt(started_at, 0)
        .single()
        .map(|datetime| datetime.format("%H:%M").to_string());

    let start = start_date.unwrap_or_default();

    let end_date = Utc
        .timestamp_opt(ended_at, 0)
        .single()
        .map(|datetime| datetime.format("%H:%M").to_string());

    let end = end_date.unwrap_or_default();

    format!("{} ~ {}", start, end)
}

pub fn current_timestamp() -> i64 {
    let now = Utc::now();
    let timestamp_millis = now.timestamp();
    timestamp_millis
}
