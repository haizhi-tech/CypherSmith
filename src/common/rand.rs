use rand::{distributions::Uniform, rngs::ThreadRng, thread_rng, Rng};

#[derive(Clone)]
pub struct RandomGenerator {
    rng: ThreadRng,
    uniform_d6: Uniform<i32>,
    uniform_d9: Uniform<i32>,
    uniform_d12: Uniform<i32>,
    uniform_d20: Uniform<i32>,
    uniform_d42: Uniform<i32>,
    uniform_d100: Uniform<i32>,
}

impl RandomGenerator {
    pub fn new() -> RandomGenerator {
        RandomGenerator {
            rng: thread_rng(),
            uniform_d6: Uniform::new(0, 6),
            uniform_d9: Uniform::new(0, 9),
            uniform_d12: Uniform::new(0, 12),
            uniform_d20: Uniform::new(0, 20),
            uniform_d42: Uniform::new(0, 42),
            uniform_d100: Uniform::new(0, 100),
        }
    }

    pub fn d6(&mut self) -> i32 {
        self.rng.sample(self.uniform_d6)
    }

    pub fn d9(&mut self) -> i32 {
        self.rng.sample(self.uniform_d9)
    }

    pub fn d12(&mut self) -> i32 {
        self.rng.sample(self.uniform_d12)
    }

    pub fn d20(&mut self) -> i32 {
        self.rng.sample(self.uniform_d20)
    }

    pub fn d42(&mut self) -> i32 {
        self.rng.sample(self.uniform_d42)
    }

    pub fn d100(&mut self) -> i32 {
        self.rng.sample(self.uniform_d100)
    }
}

impl Default for RandomGenerator {
    fn default() -> Self {
        RandomGenerator::new()
    }
}

#[cfg(test)]
mod test {
    use super::RandomGenerator;
    #[test]
    fn test_random_generator() {
        let mut gen_random = RandomGenerator::new();
        for i in 0..10 {
            eprintln!("random number {}: {}", i, gen_random.d6());
        }
        for i in 0..10 {
            eprintln!("random number {}: {}", i, gen_random.d9());
        }
        for i in 0..10 {
            eprintln!("random number {}: {}", i, gen_random.d12());
        }
        for i in 0..10 {
            eprintln!("random number {}: {}", i, gen_random.d20());
        }
        for i in 0..10 {
            eprintln!("random number {}: {}", i, gen_random.d42());
        }
        for i in 0..10 {
            eprintln!("random number {}: {}", i, gen_random.d100());
        }
    }
}
