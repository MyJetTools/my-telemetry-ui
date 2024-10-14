use rust_extensions::date_time::{DateTimeAsMicroseconds, HourKey, IntervalKey};

pub fn calc_hour_key(hours_ago: i64) -> i64 {
    let mut dt = DateTimeAsMicroseconds::now();

    if hours_ago > 0 {
        dt.add_hours(-hours_ago);
    }

    if hours_ago < 0 {
        dt.add_hours(hours_ago);
    }

    let interval: IntervalKey<HourKey> = dt.into();

    interval.to_i64()
}
