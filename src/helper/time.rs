use std::time::{SystemTime, UNIX_EPOCH};

pub fn time_stump() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

pub fn time_stump_to_data(timestamp_ms: u128) -> String {
    let duration = std::time::Duration::from_millis(timestamp_ms as u64);
    let system_time = UNIX_EPOCH + duration;

    format!("SystemTime: {:?}", system_time).to_string()
}
