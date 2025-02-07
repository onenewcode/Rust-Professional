pub fn new_birthday_probability(n: u32) -> f64 {
    // p = 1 - (365! / (365 - n)! / 365^n)
    let mut p = 1.;
    for i in 0..n {
        p *= (365 - i) as f64 / 365.;
    }
    1. - p
}