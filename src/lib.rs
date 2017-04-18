/// Asserts that two expressions are approximately (~1.0e-6) equal to each other.
///
/// On panic, this macro will print the values of the expressions with their
/// debug representations. You can optionally add an optional diff value. If you
/// don't supply a diff value as an argument, `1.0e-6` is the default used. 
///
/// # Examples
///
/// ```should panic
/// #[macro_use] 
/// extern crate assert_approx_eq;
///
/// let a = 3f64;
/// let b = 4f64;
///
/// assert_approx_eq!(a, b); //panics
/// assert_approx_eq!(a, b, 2f64); //does not panic
/// assert_approx_eq!(a, b, 1e-3f64); // panics
/// ```
#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => ({
        let eps = 1.0e-6;
        let (a, b) = (&$a, &$b);
        assert!((*a - *b).abs() < eps,
                "assertion failed: `(left !== right)` \
                           (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                 *a, *b, eps, (*a - *b).abs());
    });
    ($a:expr, $b:expr, $eps:expr) => ({
        let (a, b) = (&$a, &$b);
        assert!((*a - *b).abs() < $eps,
                "assertion failed: `(left !== right)` \
                           (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                 *a, *b, $eps, (*a - *b).abs());
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

#[test]
fn compare_with_explicit_eps() {
    assert_approx_eq!(3f64, 4f64, 2f64);
}

#[test]
#[should_panic]
fn bad_compare_with_explicit_eps() {
    assert_approx_eq!(3f64, 4f64, 1e-3f64);
}
