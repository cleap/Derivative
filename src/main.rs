
fn exp(val: f64) -> f64 {
    std::f64::consts::E.powf(val)
}

fn deriv(fun: fn(f64)->f64, val: f64, del: f64, method: String) -> f64 {

    match method.as_ref() {
        "forward" => (fun(val+del)-fun(del))/del,
        "central" => (fun(val+del)-fun(val-del))/(2.0*del),
        _ => {
            println!("Sorry, I don't know that method!");
            std::f64::NAN
        },
    }
}
/*
fn deriv(fun: fn(f64)->f64, val: f64, del: f64) -> f64 {
    (fun(val+del) - fun(val-del))/(2.0*del)
}
*/
fn main() {
    println!("Numeric Solution: {}", deriv(exp, 1.0, 0.00001, "central".to_string()));
    println!("  Exact solution: {}", exp(1.0));
}
