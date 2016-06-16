/// Asserts that two expressions are approximately (~1.0e-6) equal to each other.
///
/// On panic, this macro will print the values of the expressions with their
/// debug representations.
///
/// # Examples
///
/// ```should panic
/// # #[macro_use] extern crate assert_approx_eq;
///
/// assert_ne!(a, b);
/// ```
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

#[test]
#[should_panic]
fn it_should_panic_if_values_are_not_approx_equal() {
  assert_approx_eq!(3 as f32, 4 as f32);
}
