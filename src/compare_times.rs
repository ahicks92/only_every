/// Implements comparing sequence numbers assuming they are less than
/// `u64::MAX/2 apart`.
///
/// Returns true if `a < b` because that's all we need, and getting a full
/// ordering requires branches.
///
/// See https://en.wikipedia.org/wiki/Serial_number_arithmetic
///
/// Assumes the platform is twos complement, but at this point that's a given.
pub(crate) fn compare_times(a: u64, b: u64) -> bool {
    let wrapped_diff = a.wrapping_sub(b);
    // if a < b then we wrapped to above u64::MAX/2.
    return wrapped_diff > u64::MAX / 2;
}

#[test]
fn test_cmp() {
    let table: &[(u64, u64, bool)] = &[
        (0, 1, true),
        (1, 0, false),
        (0, 1, true),
        (5, 5, false),
        (u64::MAX - 5, 3, true),
        (3, u64::MAX - 5, false),
    ];

    for (a, b, res) in table.iter().cloned() {
        assert_eq!(compare_times(a, b), res, "{} < {}", a, b);
    }
}
