use calculus_res::differentiation::diff_cnt;
use calculus_res::integration::int_trap;

fn main() {
    let f = |x: f64| x.powi(3); // f(x) = x^3
    let x = 2.0;
    let h = 1e-5;

    let diff = diff_cnt(f, x, h);
    println!("{diff:}");

    let f = |x: f64| x.ln(); // f(x) = Ln(x)
    let a = 1.0;
    let b = 2.0;
    let n = 1_000;
    let int_trap = int_trap(f, a, b, n);
    println!("Actual integral: {}", (b * b.ln() - b) - (a * a.ln() - a));
    println!("{int_trap:}")

}