pub struct GeometricBM {
    pub mu: f64,
    pub sigma: f64,
    pub dt: f64,
}

impl GeometricBM {
    pub fn simulate(&self, s0: f64, steps: usize, rng: &mut impl Rng) -> Vec<f64> {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let drift = (self.mu - 0.5 * self.sigma * self.sigma) * self.dt;
        let diffusion = self.sigma * self.dt.sqrt();
        let mut path = vec![s0; steps + 1];
        for i in 1..=steps {
            let z = normal.sample(rng);
            path[i] = path[i - 1] * (drift + diffusion * z).exp();
        }
        path
    }
}
