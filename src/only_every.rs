use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Duration;

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
/// `check`, with behavior returning to normal once all threads are again
/// passing the same value.
pub struct OnlyEvery {
    time_source: TimeSource,
    last: AtomicI64,
}

impl OnlyEvery {
    pub fn new() -> OnlyEvery {
        let time_source = TimeSource::new();
        let last = AtomicI64::new(i64::MIN);
        OnlyEvery { time_source, last }
    }

    /// Check whether some code can execute, and record the time of the last
    /// successful check.
    ///
    /// If this function returns true, the update has already been recorded as
    /// taking place.
    ///
    /// interval is rounded up to the next ms.
    pub fn check(&self, interval: Duration) -> bool {
        let now = self.time_source.now_ms() as i64;
        let last = self.last.load(Ordering::Relaxed);
        let interval_ms_u128 = (interval + Duration::from_millis(1)).as_millis();
        debug_assert!(interval_ms_u128 <= i64::MAX as u128);
        let interval_ms = interval_ms_u128 as i64;
        let next = last.saturating_add(interval_ms);

        if now < next {
            return false;
        }

        // Exactly one thread can win this CAS.
        self.last
            .compare_exchange(last, now, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
    }
}
