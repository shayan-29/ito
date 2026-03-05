use rand_distr::{Distribution, Normal};

pub struct BrownianMotion {
    pub dt: f64,
}

impl BrownianMotion {
    pub fn simulate(&self, steps: usize, rng: &mut impl Rng) -> Vec<f64> {
        let normal = Normal::new(0.0, self.dt.sqrt()).unwrap();
        let mut path = vec![0.0f64; steps + 1];
        for i in 1..=steps {
            path[i] = path[i - 1] + normal.sample(rng);
        }
        path
    }
}
