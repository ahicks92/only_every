use std::time::{Duration, Instant};

use only_every::only_every;

#[test]
fn simple() {
    let mut counter: u64 = 0;

    let now = Instant::now();
    while Instant::now() - now < Duration::from_millis(1100) {
        only_every!(Duration::from_millis(200), counter += 1);
        std::thread::yield_now();
    }

    assert_eq!(counter, 6);
}

#[test]
fn test_many_threads() {
    use std::sync::atomic::{AtomicU64, Ordering};

    let unshared_counter = &*Box::leak(Box::new(AtomicU64::new(0)));
    let unshared_start = Instant::now();

    let mut threads = vec![];
    for _ in 0..100 {
        let start = unshared_start;
        let counter = unshared_counter.clone();
        let handle = std::thread::spawn(move || {
            while Instant::now() - start < Duration::from_millis(1100) {
                only_every!(
                    Duration::from_millis(200),
                    counter.fetch_add(1, Ordering::Relaxed)
                );
                std::thread::yield_now();
            }
        });

        threads.push(handle);
    }

    for i in threads {
        i.join().unwrap();
    }

    assert_eq!(unshared_counter.load(Ordering::Relaxed), 6);
}

#[test]
fn limiters_are_distinct() {
    let mut c1 = 0;
    let mut c2 = 0;

    for _ in 0..2 {
        only_every!(Duration::from_secs(1), c1 += 1);
        only_every!(Duration::from_secs(1), c2 += 1);
    }

    assert_eq!(c1, 1);
    assert_eq!(c2, 1);
}
