mod runner;
mod seeder;

use std::time::Duration;
use std::thread;
use runner::{print_runner_program_result, RunnableProgram, RunnerPrinter, RunnerProgram};

fn main() {
    println!("Hello, world!");


    thread::sleep(Duration::new(1, 0));
    let runner_program: RunnerProgram = RunnerProgram::init("./crashy");
    let result = runner_program.run("seed");
    print_runner_program_result(result);
}

// Test module
#[cfg(test)]
mod tests {
    use rand::rngs::{StdRng, ThreadRng};
    use runner::{Runnable, RunnerResult, State};
    use seeder::{RandomSeeder, Seedable};

    use super::*;

    #[test]
    fn test_runner_printer() {
        let runner_printer: RunnerPrinter = RunnerPrinter::init();
        let res: RunnerResult = runner_printer.run("test_seed");
        matches!(res.state, State::Pass);
    }

    #[test]
    fn test_random_seeder_random_setup() {
        let mut random_seeder: RandomSeeder<ThreadRng> = RandomSeeder::<ThreadRng>::init_random();
        let seed = random_seeder.next_seed();
        let seed2 = random_seeder.next_seed();
        assert_ne!(seed, seed2);
    }

    #[test]
    fn test_random_seeder_with_initial_seed() {
        let mut random_seeder1: RandomSeeder<StdRng> = RandomSeeder::<StdRng>::init(12345);
        let mut random_seeder2: RandomSeeder<StdRng> = RandomSeeder::<StdRng>::init(12345);
        let mut random_seeder3: RandomSeeder<StdRng> = RandomSeeder::<StdRng>::init(12346);
        let seed1 = random_seeder1.next_seed();
        let seed2 = random_seeder2.next_seed();
        let seed3 = random_seeder3.next_seed();
        assert_eq!(seed1, seed2);
        assert_ne!(seed1, seed3);
        assert_ne!(seed2, seed3);
    }

}