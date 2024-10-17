use rand::{rngs::{StdRng, ThreadRng}, Rng, RngCore, SeedableRng};

/*
 * This is the maximum length of a generated seed in the configuration
 * of a RandomSeeder::init_random() or RandomSeeder::init(). 
 */
const MAX_RANDOM_SEEDER_LENGTH: u64 = 4096;

pub trait Seedable {
    fn next_seed(&mut self) -> Vec<u8>;
}

pub struct RandomSeeder<T: Rng> {
    initial_seed: u64,
    rng_generator: T,
    min_length: u64,
    max_length: u64,
}

impl<T: Rng> RandomSeeder<T> {
    pub fn init_random() -> RandomSeeder<ThreadRng> {
        let mut rng_generator = rand::thread_rng();
        let next_u64: u64 = rng_generator.next_u64();
        RandomSeeder {
            initial_seed: next_u64,
            rng_generator: rng_generator,
            min_length: 1,
            max_length: MAX_RANDOM_SEEDER_LENGTH,
        }
    }
    pub fn init(initial_seed: u64) -> RandomSeeder<StdRng> {
        let rng_generator = StdRng::seed_from_u64(initial_seed);
        RandomSeeder {
            initial_seed: initial_seed,
            rng_generator: rng_generator,
            min_length: 1,
            max_length: MAX_RANDOM_SEEDER_LENGTH,
        }
    }

    pub fn init_with_limit(initial_seed: u64,
        min_length: u64, max_length: u64,
        ) -> RandomSeeder<ThreadRng> {
        
        assert!(min_length <= max_length);

        let rng_generator = rand::thread_rng();
        RandomSeeder {
            initial_seed: initial_seed,
            rng_generator: rng_generator,
            min_length: min_length,
            max_length: max_length,
        }
    }
}

impl<T: Rng> Seedable for RandomSeeder<T> {
    fn next_seed(&mut self) -> Vec<u8> {
        let length = self.rng_generator.gen_range(self.min_length..self.max_length);
        (0..length)
        .map(|_| self.rng_generator.gen_range(0..=255) as u8)
        .collect()
    }
}   