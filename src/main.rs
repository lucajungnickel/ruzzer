mod runner;
mod seeder;
mod grammar;
mod fuzzer;
mod logger;


use clap::Parser;
use fuzzer::FuzzerProgram;
use grammar::create_cgi_grammar;
use rand::rngs::StdRng;
use rand::SeedableRng;
use runner::RunnerProgram;
use seeder::{GrammarSeeder, MutationSeedModifier};

#[derive(Parser)]
struct Cli {
    /// Set the logging level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}


fn main() {

    let cli = Cli::parse();
    std::env::set_var("RUST_LOG", cli.log_level);
    env_logger::init();
    
    let runner_program: RunnerProgram = RunnerProgram::init("./SUTs/CGI_crashy_asan");
    let rng = StdRng::from_entropy();
    let grammar_cgi = create_cgi_grammar();
    let grammar_seeder = GrammarSeeder::init(grammar_cgi, rng);
    let mutation_grammar_seeder = MutationSeedModifier::init(grammar_seeder);

    //let mut fuzzer = FuzzerProgram::init(runner_program, grammar_seeder);
    let mut fuzzer = FuzzerProgram::init(runner_program, mutation_grammar_seeder);
    
    fuzzer.run_forever();

}

#[cfg(test)]
mod tests {
    use rand::rngs::{StdRng, ThreadRng};
    use runner::{Runnable, RunnerPrinter, RunnerResult, State};
    use seeder::{RandomSeeder, Seedable};

    use super::*;

    #[test]
    fn test_runner_printer() {
        let runner_printer: RunnerPrinter = RunnerPrinter::init();
        let res: RunnerResult = runner_printer.run(&"test_seed".as_bytes().to_vec());
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