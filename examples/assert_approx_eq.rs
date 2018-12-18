use assert_approx_eq::assert_approx_eq;

fn main() {
    let a = 3f64;
    let b = 4f64;

    assert_approx_eq!(a, a);
    assert_approx_eq!(a, b, 2f64);
}
