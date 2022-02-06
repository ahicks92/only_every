use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crate::compare_times::compare_times;
#[cfg(quanta)]
use crate::quanta_time_source::QuantaTimeSource as TimeSource;
#[cfg(not(quanta))]
use crate::std_time_source::StdTimeSource as TimeSource;

/// A simple rate limiter which allows one element through on a given interval.
///
/// To use, call `check` and do whatever you want to do when it returns true.
///
/// The interval can be changed, and this will do what you expect as long as
/// usage is only single-threaded.  In multi-threaded programs, it is guaranteed
/// that no execution can happen faster than the fastest interval passed to
/// `check` with predictable behavior once all threads are passing the same
/// value again.
pub struct OnlyEvery {
    time_source: TimeSource,
    last: AtomicU64,
}

impl OnlyEvery {
    pub fn new() -> OnlyEvery {
        let time_source = TimeSource::new();
        // This last is less than all other possible values unless the program
        // has been up for u64::MAX/2 seconds.
        let last = AtomicU64::new(u64::MAX / 2 + 1);
        OnlyEvery { time_source, last }
    }

    /// Check whether some code can execute, and record the time of the last
    /// successful check.
    ///
    /// If this function returns true the code *must* execute.
    ///
    /// interval is rounded up to the next ms.
    pub fn check(&self, interval: Duration) -> bool {
        let now = self.time_source.now_ms();
        let last = self.last.load(Ordering::Relaxed);
        let interval_ms_u128 = (interval + Duration::from_millis(1)).as_millis();
        debug_assert!(interval_ms_u128 <= u64::MAX as u128);
        let interval_ms = interval_ms_u128 as u64;
        let next = last.wrapping_add(interval_ms);

        if compare_times(now, next) {
            return false;
        }

        // Exactly one thread can win this CAS.
        self.last
            .compare_exchange(last, now, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
    }
}
