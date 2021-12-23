use rand::{distributions::Uniform, rngs::ThreadRng, thread_rng, Rng};

#[derive(Clone)]
pub struct RandomGenerator {
    rng: ThreadRng,
    uniform_d2: Uniform<i32>,
    uniform_d6: Uniform<i32>,
    uniform_d9: Uniform<i32>,
    uniform_d12: Uniform<i32>,
    uniform_d20: Uniform<i32>,
    uniform_d100: Uniform<i32>,
}

impl RandomGenerator {
    pub fn new() -> RandomGenerator {
        RandomGenerator {
            rng: thread_rng(),
            uniform_d2: Uniform::new(0, 2),
            uniform_d6: Uniform::new(0, 6),
            uniform_d9: Uniform::new(0, 9),
            uniform_d12: Uniform::new(0, 12),
            uniform_d20: Uniform::new(0, 20),
            uniform_d100: Uniform::new(0, 100),
        }
    }

    pub fn d2(&mut self) -> i32 {
        self.rng.sample(self.uniform_d2)
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

    pub fn d100(&mut self) -> i32 {
        self.rng.sample(self.uniform_d100)
    }

    /// generate i32 in range [0, number).
    pub fn under(&mut self, number: i32) -> i32 {
        if number <= 0 {
            return 0;
        }
        let uniform = Uniform::new(0, number);
        self.rng.sample(uniform)
    }

    /// generate i32 in range [lo, hi).
    pub fn range(&mut self, lo: i32, hi: i32) -> i32 {
        if lo == hi {
            return lo;
        }
        if lo > hi {
            self.rng.sample(Uniform::new(hi, lo))
        } else {
            self.rng.sample(Uniform::new(lo, hi))
        }
    }

    pub fn bool(&mut self) -> bool {
        self.d2() < 1
    }

    pub fn low_prob_bool(&mut self) -> bool {
        self.d12() < 1
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
            eprintln!("random number {}: {}", i, gen_random.d2());
        }
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
            eprintln!("random number {}: {}", i, gen_random.d100());
        }
    }
}
