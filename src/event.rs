//! Timetable event struct.

use chrono::{DateTime, Duration, Local};

/// Event info struct.
#[derive(Debug, Clone)]
pub struct Event {
    pub event_name: String,
    pub event_type: String,
    pub location: String,
    pub from: DateTime<Local>,
    pub to: DateTime<Local>,
}

impl Event {
    /// Checks if the event's end date lies between [now() - low, now() + high].
    pub fn in_day_range(&self, low: Duration, high: Duration) -> bool {
        let diff = (self.to - chrono::Local::now()).num_minutes();
        diff >= low.num_minutes() && diff <= high.num_minutes()
    }
}
