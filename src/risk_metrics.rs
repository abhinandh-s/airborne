pub struct Beta(f64);

impl Beta {
    pub fn new(series: &[f64], market: &[f64]) -> Self {
        Self(0.0f64)
    }
}
