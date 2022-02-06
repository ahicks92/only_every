# only_every

There are lots of rate-limiting crates that do lots of things, but sometimes you
just want to evaluate an expression once every whatever interval, for example
rate-limited logging.  This crate exposes a macro:

```
only_every!(Duration::from_millis(100), expensive_thing)
```

This macro will evaluate the given expression at most every duration rounded up
to the next ms.  The limiter is global: if you use 20 threads, it's still only
going to happen once every duration.  The expression is not inside a closure and
is consequently somewhat more transparent to the borrow checker (This probably
doesn't matter to you; you can read it as "there aren't edge cases").

Enable the `quanta` feature for faster time-keeping at the cost of using the
quanta crate, which requires a calibration period when executing the first
rate-limited expression and an O(1) heap allocation.  If you use quanta for
other things, whoever gets there first handles the calibration.

If you need a bit more, e.g. storing these in structs instead of using the
macro, there is also a `OnlyEvery` type.  I suggest
something like [governor](https://docs.rs/governor/latest/governor/) if you need
more than "execute this once every x".

A caveat that probably doesn't matter: if the expression executes less
frequently than `u64::MAX / 2 - 1` ms, behavior is undefined due to internal
wrapping.
