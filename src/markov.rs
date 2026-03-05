pub struct MarkovChain {
    pub states: usize,
    pub transition: Vec<Vec<f64>>, // transition[i][j] = P(state i → state j)
}

impl MarkovChain {
    pub fn new(transition: Vec<Vec<f64>>) -> Result<Self, String> {
        let states = transition.len();

        // Validation of transition matrix 
        for (i, row) in transition.iter().enumerate() {
            if row.len() != states {
                return Err(format!("Row {i} has length {}, expected {states}", row.len()));
            }
            let sum: f64 = row.iter().sum();
            
            // sum of rows of transition matrix should be 1

            if (sum - 1.0).abs() > 1e-9 {
                return Err(format!("Row {i} sums to {sum}, expected 1.0"));
            }
        }

        Ok(Self { states, transition })
    }

    pub fn next_state(&self, current: usize, rng: &mut impl Rng) -> usize {
        
        // See https://en.wikipedia.org/wiki/Inverse_transform_sampling

        // u chosen at random from [0, 1)
        let u: f64 = rng.gen();

        let mut cumsum = 0.0;

        for (j, &prob) in self.transition[current].iter().enumerate() {
            cumsum += prob;
            if u < cumsum {
                return j;
            }
        }
        // if the above loop fails (maybe u close to 1, float point addition does weird things and ends up being less than u for the entire row)
        // then just return the state with last index as fallback. If u was that large, last state is the correct choice under the logic of the
        //sampling anyway       
        self.states - 1
    }

    pub fn simulate(&self, start: usize, steps: usize, rng: &mut impl Rng) -> Vec<usize> {
        let mut path = vec![start; steps + 1];
        for i in 1..=steps {
            path[i] = self.next_state(path[i - 1], rng);
        }
        path
    }
}
