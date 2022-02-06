#![allow(dead_code)]
mod compare_times;
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
