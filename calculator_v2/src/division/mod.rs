pub fn division(x: f64, y: f64) -> Option<f64> {
    if y != 0.0 {
        return Some(x / x);
    }

    return None
}