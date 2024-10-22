use rand::{rngs::{StdRng, ThreadRng}, Rng, RngCore, SeedableRng};

use crate::grammar::{generate, Grammar, GRAMMAR_ENTRY};

/*
 * This is the maximum length of a generated seed in the configuration
 * of a RandomSeeder::init_random() or RandomSeeder::init(). 
 */
#[allow(unused)]
const MAX_RANDOM_SEEDER_LENGTH: u64 = 4096;

pub trait Seedable {
    fn next_seed(&mut self) -> Vec<u8>;
}

#[allow(unused)]
pub struct RandomSeeder<T: Rng> {
    initial_seed: u64,
    rng_generator: T,
    min_length: u64,
    max_length: u64,
}

#[allow(unused)]
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


pub struct GrammarSeeder<T: Rng> {
    pub grammar: Grammar,
    pub rng: T
}

impl<T: Rng> GrammarSeeder<T> {
    pub fn init(grammar: Grammar, rng: T) -> GrammarSeeder<T> {
        GrammarSeeder { grammar: grammar, rng: rng }
    }
}

impl<T: Rng> Seedable for GrammarSeeder<T> {
    fn next_seed(&mut self) -> Vec<u8> {
        generate(&self.grammar, GRAMMAR_ENTRY, &mut self.rng).into_bytes()
    }
}

pub struct MutationSeedModifier<T: Rng> {
    grammar_seeder: GrammarSeeder<T>
}

impl<T: Rng> MutationSeedModifier<T> {
    pub fn init(grammar_seeder: GrammarSeeder<T>) -> MutationSeedModifier<T> {
        MutationSeedModifier {
            grammar_seeder: grammar_seeder
        }
    }

    fn insert_byte(&mut self, input: &mut Vec<u8>) {
        let byte = self.grammar_seeder.rng.gen::<u8>();
        let position = self.grammar_seeder.rng.gen_range(0..=input.len()); // Can insert at the end as well
        input.insert(position, byte);
    }
    
    fn remove_byte(&mut self, input: &mut Vec<u8>) {
        if !input.is_empty() {
            let position = self.grammar_seeder.rng.gen_range(0..input.len());
            input.remove(position);
        }
    }
    
    fn modify_byte(&mut self, input: &mut Vec<u8>) {
        if !input.is_empty() {
            let position = self.grammar_seeder.rng.gen_range(0..input.len());
            let new_byte = self.grammar_seeder.rng.gen::<u8>();
            input[position] = new_byte;
        }
    }
}

impl<T: Rng> Seedable for MutationSeedModifier<T> {
    fn next_seed(&mut self) -> Vec<u8> {
        //get seed
        let mut seed = self.grammar_seeder.next_seed();
        //modifiy seed sometimes
        let operation = self.grammar_seeder.rng.gen_range(0..4);

        match operation {
            0 => self.insert_byte(&mut seed),
            1 => self.remove_byte(&mut seed),
            2 => self.modify_byte(&mut seed),
            _ => { 
                //dont modify
            },
        }

        seed
    }
}

