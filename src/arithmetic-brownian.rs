pub struct ArithmeticBM {
    pub mu: f64,    // drift
    pub sigma: f64, // volatility
    pub dt: f64,
}

impl ArithmeticBM {
    pub fn simulate(&self, x0: f64, steps: usize, rng: &mut impl Rng) -> Vec<f64> {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut path = vec![x0; steps + 1];
        for i in 1..=steps {
            let dw = normal.sample(rng) * self.dt.sqrt();
            path[i] = path[i - 1] + self.mu * self.dt + self.sigma * dw;
        }
        path
    }
}
