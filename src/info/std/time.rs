use std::time::{Duration, Instant, SystemTime};

use super::PayloadInfo;

impl PayloadInfo for Duration {
    const TYPE: &'static str = "Duration";
    const SIZE: Option<usize> = Some(std::mem::size_of::<Duration>());
}

impl PayloadInfo for Instant {
    const TYPE: &'static str = "Instant";
    const SIZE: Option<usize> = Some(std::mem::size_of::<Instant>());
}

impl PayloadInfo for SystemTime {
    const TYPE: &'static str = "SystemTime";
    const SIZE: Option<usize> = Some(std::mem::size_of::<SystemTime>());
}
