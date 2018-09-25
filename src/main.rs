
fn exp(val: f64) -> f64 {
    std::f64::consts::E.powf(val)
}

fn deriv(fun: fn(f64)->f64, val: f64, del: f64) -> f64 {
    (fun(val+del) - fun(val))/del
}

fn main() {
    println!("Numeric Solution: {}", deriv(exp, 1.0, 0.00001));
    println!("  Exact solution: {}", exp(1.0));
}
