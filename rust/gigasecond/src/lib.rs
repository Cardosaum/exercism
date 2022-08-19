use core::time::Duration;
use std::ops::Add;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.add(Duration::from_secs(10u64.pow(9)))
}
