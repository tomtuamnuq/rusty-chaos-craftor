use serde::{Deserialize, Serialize};
use web_time::{Duration, Instant};
#[derive(PartialEq, Eq, Clone, Copy)]
struct Trigger {
    pub time: Instant,
}
impl Trigger {
    fn now() -> Self {
        Self {
            time: Instant::now(),
        }
    }
}
impl Default for Trigger {
    fn default() -> Self {
        Self::now()
    }
}
/// A simple timer for managing time intervals and execution timing.
#[derive(PartialEq, Eq, Deserialize, Serialize)]
pub struct Timer {
    interval: Duration,
    #[serde(skip)] // always start with a fresh instant
    last_triggered: Trigger,
}

impl Timer {
    /// Creates a new timer with the specified interval.
    ///
    /// # Arguments
    ///
    /// * `interval` - The time interval between trigger events.
    pub fn new(seconds: usize) -> Self {
        Self {
            interval: Duration::from_secs(seconds.try_into().unwrap()),
            last_triggered: Trigger::now(),
        }
    }

    /// Sets the frequency of the timer by specifying how many triggers per second are desired.
    ///
    /// # Arguments
    ///
    /// * `frequency` - The desired frequency in triggers per second.
    ///
    /// # Notes
    ///
    /// - If `frequency` is greater than 0, the timer's interval is set accordingly.
    /// - If `frequency` is 0 or negative, the timer interval is set to a default value of 1 second.
    pub fn set_frequency(&mut self, frequency: f64) {
        if frequency > 0.0 {
            let interval = Duration::from_secs_f64(1.0 / frequency);
            self.interval = interval;
        } else {
            // Here, we set a default interval of 1 second.
            self.interval = Duration::from_secs(1);
        }
    }

    /// Checks if the timer interval has elapsed and triggers the timer.
    ///
    /// # Returns
    ///
    /// Returns `true` if the interval has elapsed and `false` otherwise.
    pub fn check_elapsed(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_triggered.time) >= self.interval {
            self.last_triggered.time = now;
            return true;
        }
        false
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new(1)
    }
}
