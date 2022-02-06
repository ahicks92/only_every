use std::sync::Once;

use quanta::Clock;

fn get_clock() -> &'static Clock {
    static mut CLOCK: *const Clock = std::ptr::null();
    static CLOCK_ONCE: Once = Once::new();

    CLOCK_ONCE.call_once(|| {
        let c = Box::new(Clock::new());
        unsafe {
            CLOCK = Box::into_raw(c);
        }
    });

    unsafe { &*CLOCK }
}

struct QuantaTimeSource {
    clock: &'static Clock,
    epoch: quanta::Instant,
}

impl QuantaTimeSource {
    pub(crate) fn new() -> QuantaTimeSource {
        let clock = get_clock();
        let epoch = clock.now();
        QuantaTimeSource { clock, epoch }
    }

    fn now_ms(&self) -> u64 {
        let dur = self.clock.now() - self.epoch;
        let ms = dur.as_millis();
        debug_assert!(ms <= u64::MAX as u128);
        ms as u64
    }
}
