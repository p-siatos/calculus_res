/// Calculates the derivative of a function at a point using the forward
/// difference method.
///
/// # Arguments
///
/// * `f` - The function to differentiate.
/// * `x` - The point at which to evaluate the derivative.
/// * `h` - The step size
/// # Returns
///
/// The approximate derivative at point `x`
pub fn diff_fwd<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x)) / h
}

/// Calculates the derivative of a function at a point using the central
/// difference method.
pub fn diff_cnt<F>(f: F, x: f64, h: f64) ->f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x - h)) / (2.0 * h)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_fwd() {
        let f = |x: f64| x.powi(2); // f(x) = x^2
        let x = 2.0;
        let h = 1e-5;

        let derivative = diff_fwd(f, x, h);
        let expected = 2.0 * x;

        let error = (derivative - expected).abs();
        println!("Derivative: {}", derivative);
        println!("Expected: {}", expected);
        assert!(error < 1e-4);
    }

    #[test]
    fn test_diff_cnt() {
        let f = |x: f64| x.powi(2); // f(x) = x^2
        let x = 2.0;
        let h = 1e-4;

        let derivative = diff_cnt(f, x, h);
        let expected = 2.0 * x;

        let error = (derivative - expected).abs();
        println!("Derivative: {}", derivative);
        println!("Expected: {}", expected);
        assert!(error < 1e-4);

    }
}

