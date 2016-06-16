#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => ({
        let (a, b) = (&$a, &$b);
        assert!((*a - *b).abs() < 1.0e-6,
                "assertion failed: `(left !== right)` \
                           (left: `{:?}`, right: `{:?}`)",
                 *a, *b);
    })
}

#[test]
fn it_should_not_panic_if_values_are_approx_equal() {
  assert_approx_eq!(64f32.sqrt(), 8f32);
}
