use chrono::{Datelike, Local, TimeZone, Timelike, Utc};

pub fn convert_timestamp_to_fmt_string(timestamp: i64, format: &str) -> String {
    let date_time = Utc.timestamp_millis_opt(timestamp).unwrap();
    let local_date_time = date_time.with_timezone(&Local);
    local_date_time.format(format).to_string()
}

pub fn convert_timestamp_to_separate_string(timestamp: i64) -> (i32, u32, u32) {
    let date_time = Utc.timestamp_millis_opt(timestamp).unwrap();
    let local_date_time = date_time.with_timezone(&Local);

    (
        local_date_time.year(),
        local_date_time.month(),
        local_date_time.day(),
    )
}

pub fn get_hour_from_timestamp(timestamp: i64) -> u32 {
    let datetime = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .expect("Invalid timestamp");

    datetime.hour()
}

pub fn update_hour_in_timestamp(timestamp: i64, new_hour: u32) -> i64 {
    let datetime = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .expect("Invalid timestamp");

    let updated_utc = datetime.with_hour(new_hour).expect("Invalid hour update");

    updated_utc.timestamp()
}

pub fn convert_timestamp_to_date(timestamp: i64) -> String {
    let datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
    datetime.format("%Y.%m.%d").to_string()
}

pub fn change_date_from_timestamp(timestamp: i64) -> String {
    let datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
    let formatted_date = datetime.format("%Y/%m/%d").to_string();

    formatted_date
}

pub fn format_remaining_time(target_timestamp: i64) -> String {
    let today = Local::now().date_naive();
    let target_date = Utc.timestamp_opt(target_timestamp, 0).unwrap().date_naive();

    if target_date <= today {
        return "0일".to_string();
    }

    let mut remaining_days = (target_date - today).num_days();
    let mut years = 0;
    let mut months = 0;

    while remaining_days >= 365 {
        years += 1;
        remaining_days -= 365;
    }

    while remaining_days >= 30 {
        months += 1;
        remaining_days -= 30;
    }

    let mut result = String::new();
    if years > 0 {
        result.push_str(&format!("{}년 ", years));
    }
    if months > 0 {
        result.push_str(&format!("{}개월 ", months));
    }
    if remaining_days > 0 || result.is_empty() {
        result.push_str(&format!("{}일", remaining_days));
    }

    result.trim().to_string()
}

pub fn current_timestamp() -> i64 {
    let now = Utc::now();
    let timestamp_millis = now.timestamp();
    timestamp_millis
}

pub fn current_timestamp_with_time(hour: i64, minute: i64, sec: i64) -> i64 {
    let now = Utc::now();
    let date = Utc
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
        .unwrap();

    let updated = date
        .with_hour(hour as u32)
        .and_then(|d| d.with_minute(minute as u32))
        .and_then(|d| d.with_second(sec as u32))
        .expect("Invalid time components");

    updated.timestamp()
}

pub fn parsed_timestamp_with_time(timestamp: i64, hour: i64, minute: i64, sec: i64) -> i64 {
    let dt = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .expect("Invalid timestamp");

    let updated = Utc
        .with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 0, 0, 0)
        .unwrap()
        .with_hour(hour as u32)
        .and_then(|d| d.with_minute(minute as u32))
        .and_then(|d| d.with_second(sec as u32))
        .expect("Invalid time components");

    updated.timestamp()
}

pub fn current_midnight_timestamp() -> i64 {
    let now = Utc::now();
    let timestamp_millis = Utc
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
        .unwrap();
    timestamp_millis.timestamp()
}

pub fn format_range_from_timestamp(started_at: i64, ended_at: i64) -> String {
    let start = Utc
        .timestamp_opt(started_at, 0)
        .single()
        .unwrap_or_else(Utc::now);

    let end = Utc
        .timestamp_opt(ended_at, 0)
        .single()
        .unwrap_or_else(Utc::now);

    if start.year() == end.year() {
        format!(
            "{}.{} - {}.{}.{}",
            start.day(),
            start.month(),
            end.day(),
            end.month(),
            end.year()
        )
    } else {
        format!(
            "{}.{}.{} - {}.{}.{}",
            start.day(),
            start.month(),
            start.year(),
            end.day(),
            end.month(),
            end.year()
        )
    }
}
