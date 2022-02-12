//! # only_every
//!
//! There are lots of rate-limiting crates that do lots of things, but sometimes
//! you just want to evaluate an expression once every whatever interval, for
//! example rate-limited logging.  This crate exposes a macro:
//!
//! ```
//! only_every!(Duration::from_millis(100), expensive_thing)
//! ```
//!
//! This macro will evaluate the given expression at most every duration rounded
//! up to the next ms.  The limiter is global: if you use 20 threads, it's still
//! only going to happen once every duration.  The expression is not inside a
//! closure and is consequently somewhat more transparent to the borrow checker
//! (This probably doesn't matter to you; you can read it as "there aren't edge
//! cases").
//!
//! Enable the `quanta` feature for faster time-keeping at the cost of using the
//! quanta crate, which requires a calibration period when executing the first
//! rate-limited expression and an O(1) heap allocation.  If you use quanta for
//! other things, whoever gets there first handles the calibration.
//!
//! If you need a bit more, e.g. storing these in structs instead of using the
//! macro, there is also a `OnlyEvery` type.  I suggest something like
//! [governor](https://docs.rs/governor/latest/governor/) if you need more than
//! "execute this once every x".
//!
//! For completeness, internally we hold times in an i64 as ms since the process
//! started.  Behavior is undefined if your process runs for long enough that
//! `pt + interval > i64::MAX` where `pt` is the uptime of the process and units
//! are in ms.  In other words, let me know if you have a billion years of
//! continuous uptime and I'll fix it for you.
#![allow(dead_code)]
mod only_every;
#[cfg(quanta)]
mod quanta_time_source;
#[cfg(not(quanta))]
mod std_time_source;

pub use crate::only_every::OnlyEvery;

#[macro_export]
macro_rules! only_every {
    ($interval: expr, $expression: expr) => {
        // open a scope so that our statics are unique.
        {
            use std::mem::MaybeUninit;
            use std::sync::Once;
            use $crate::OnlyEvery;

            static mut OE: MaybeUninit<OnlyEvery> = MaybeUninit::uninit();
            static OE_ONCE: Once = Once::new();
            OE_ONCE.call_once(|| unsafe {
                OE = MaybeUninit::new(OnlyEvery::new());
            });

            if unsafe { (*OE.as_ptr()).check($interval) } {
                $expression;
            }
        }
    };
}
