use crate::{logger::{log_crash, log_internal_error}, runner::{RunnableProgram, RunnerProgramResult, RunnerResult}, seeder::Seedable};


/**
 * A fuzzer uses a runner and fuzzes the given input.
 */

const PRINT_STATUS_EVERY_RUN: u32 = 100;

#[allow(unused)]
 pub struct FuzzerProgram<R: RunnableProgram, S: Seedable> {
    pub runnable_instance: R,
    pub seedable_instance: S,
    pub count_run: u64,
    pub crash: Vec<RunnerResult>,
    pub no_crash: Vec<RunnerResult>,
    pub unknown_crash_status: Vec<RunnerResult>,
}    
impl<R: RunnableProgram, T: Seedable> FuzzerProgram<R, T> {
    pub fn init(runnable_instance: R, seedable_instance: T) -> Self {
        FuzzerProgram { 
            runnable_instance: runnable_instance, 
            seedable_instance: seedable_instance,
            count_run: 0,
            crash: Vec::new(),
            no_crash: Vec::new(),
            unknown_crash_status: Vec::new(),
        }
    }

    pub fn run_forever(&mut self) {
        loop {
            let result = self.run_one_time();
            match result.result.state {
                crate::runner::State::Fail => {
                    log_crash(&result);
                },
                crate::runner::State::InternalError => {
                    log_internal_error(&result);
                }
                _ => {
                }
            }
            
            //regular printing of status:
            if (self.count_run % (PRINT_STATUS_EVERY_RUN as u64)) == 0 {
                self.print_results();
            }
        }
    }


    #[allow(unused)]
    pub fn run_until_error(&mut self) {
        loop {
            let result = self.run_one_time();
            match result.result.state {
                crate::runner::State::Fail => {
                    log_crash(&result);
                    break;
                },
                _ => {

                }
            }
        }
    }

    pub fn run_one_time(&mut self) -> RunnerProgramResult {
        //generate seed
        let mut seed = self.seedable_instance.next_seed();
        //println!("Seed: {:?}", String::from_utf8_lossy(&seed));

        //sanitize seed to make it SUT ready
        self.runnable_instance.sanitize_seed(&mut seed);

        //feed it to the runner
        let result = self.runnable_instance.run(&seed);
        //process result
        self.count_run = self.count_run + 1;
        match result.result.state {
            crate::runner::State::Pass => {
                
            },
            crate::runner::State::Fail => {
                self.crash.push(result.result.clone());
            },
            _ => {
                self.unknown_crash_status.push(result.result.clone());
            }
        }
        result
    }

    pub fn print_results(&self) {
        println!("--------------------------REPORT--------------------------");
        println!("Total runs: {}", self.count_run);
        println!("Number crashes: {}", self.crash.len());
        
        if self.crash.len() != 0 {
            println!("Crashes:");
            for result in &self.crash {
                println!("  State: {:?}, Seed: {:?}", result.state, result.seed);
            }
        }
        if self.unknown_crash_status.len() != 0 {
            println!("Unknown Crash Statuses:");
            for result in &self.unknown_crash_status {
                println!("  State: {:?}, Seed: {:?}", result.state, result.seed);
            }
        }
    }
}
