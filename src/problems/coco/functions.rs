pub fn sphere(x: &[f64]) -> f64 {
    return x.iter().map(|xi| xi * xi).sum();
}
