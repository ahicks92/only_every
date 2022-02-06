use std::sync::Once;
use std::time::Instant;

/// Get ms since an undefined epoch which is valid only for this process.

pub(crate) struct StdTimeSource;

impl StdTimeSource {
    pub(crate) fn new() -> StdTimeSource {
        StdTimeSource
    }

    pub(crate) fn now_ms(&self) -> u64 {
        static mut EPOCH: Option<Instant> = None;
        static EPOCH_ONCE: Once = Once::new();

        EPOCH_ONCE.call_once(|| {
            unsafe { EPOCH = Some(Instant::now()) };
        });

        // Let's work with old versions of rust, and implement unchecked unwrapping
        // ourselves.
        let ep = unsafe {
            match EPOCH.clone() {
                Some(x) => x,
                None => std::hint::unreachable_unchecked(),
            }
        };

        let dur = Instant::now() - ep;
        let ms = dur.as_millis();
        debug_assert!(ms <= u64::MAX as u128);
        ms as u64
    }
}
