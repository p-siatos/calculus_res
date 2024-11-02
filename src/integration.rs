/// Integrates a function over an interval using the trapezoidal rule.
///
/// # Arguments
///
/// * `f` - The function to integrate.
/// * `a` - The Lower limit of integration.
/// * `b` - The Lower limit of integration.
/// * `n` - The number of subintervals.
///
/// # Returns
///
/// The approximate integral of `f` from `a` to `b`.
pub fn int_trap<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));

    for i in 1..n {
        sum += f(a+i as f64 * h);
    }
    sum * h
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_int_trap() {
        let f = |x: f64| x; // f(x) = x
        let a = 0.0;
        let b = 1.0;
        let n = 1_000;
        let integral = int_trap(f, a, b, n);
        let expected = 0.5;
        let error = (integral - expected).abs();
        assert!(error < 1e-5);
    }

}