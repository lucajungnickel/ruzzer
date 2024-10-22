use crate::{runner::{Runnable, RunnableProgram, RunnerProgramResult, RunnerResult}, seeder::Seedable};
use crate::logger::Logger;


/**
 * A fuzzer uses a runner and fuzzes the given input.
 */

const PRINT_STATUS_EVERY_RUN: u32 = 1000;
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
                    Logger::log_crash(&result);
                },
                crate::runner::State::InternalError => {
                    Logger::log_internal_error(&result);
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

    pub fn run_until_error(&mut self) {
        loop {
            let result = self.run_one_time();
            match result.result.state {
                crate::runner::State::Fail => {
                    Logger::log_crash(&result);
                    break;
                },
                _ => {

                }
            }
        }
    }

    pub fn run_one_time(&mut self) -> RunnerProgramResult {
        //generate seed
        let seed = self.seedable_instance.next_seed();
        //println!("Seed: {:?}", String::from_utf8_lossy(&seed));

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
        Logger::log_info("--------------------------REPORT--------------------------");
        println!("Total runs: {}", self.count_run);
        println!("Number crashes: {}", self.crash.len());
        
        if self.crash.len() != 0 {
            Logger::log_info("Crashes:");
            for result in &self.crash {
                println!("  State: {:?}, Seed: {:?}", result.state, result.seed);
            }
        }
        if self.unknown_crash_status.len() != 0 {
            Logger::log_warning("Unknown Crash Statuses:");
            for result in &self.unknown_crash_status {
                println!("  State: {:?}, Seed: {:?}", result.state, result.seed);
            }
        }
    }
}
