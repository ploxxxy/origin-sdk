use tracing::debug;

const RAND_MAX: u32 = 32767;
const MULTIPLIER: u32 = 214013;
const INCREMENT: u32 = 2531011;

pub struct Random {
    seed: u32,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn next(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        (self.seed >> 16) & RAND_MAX
    }

    pub fn set_seed(&mut self, seed: u32) {
        debug!("Setting new random seed: {}", seed);
        self.seed = seed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rng = Random::new(1337);

        let random_number = rng.next();
        assert_eq!(random_number, 4404);

        let random_number = rng.next();
        assert_eq!(random_number, 25954);
    }
}
