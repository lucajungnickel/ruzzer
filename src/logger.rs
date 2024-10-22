use crate::runner::RunnerProgramResult;
use colored::*;

pub struct Logger {
}

impl Logger {
    pub fn log_info(message: &str) {
        println!("{}", message.green());
    }

    pub fn log_warning(message: &str) {
        println!("{}", message.yellow());
    }

    pub fn log_error(message: &str) {
        println!("{}", message.red());
    }

    pub fn log_internal_error(res: &RunnerProgramResult) {
        Logger::log_info("Internal Error while fuzzing");
        println!("Seed: \t\t\t{:?}", res.result.seed);
        println!("Seed Ascii: \t\t{:?}", String::from_utf8_lossy(&res.result.seed));
        println!("Return code: \t\t{:?}", res.return_code);
        println!("Stdout: \t\t{:?}", String::from_utf8_lossy(&res.output_stdout));
        println!("Stderr: \t\t{:?}", String::from_utf8_lossy(&res.output_stderr));
    }

    pub fn log_crash(res: &RunnerProgramResult) {
        Logger::log_info("Crash found!");
        println!("Seed: \t\t\t{:?}", res.result.seed);
        println!("Seed Ascii: \t\t{:?}", String::from_utf8_lossy(&res.result.seed));
        println!("Return code: \t\t{:?}", res.return_code);
        println!("Stdout: \t\t{:?}", String::from_utf8_lossy(&res.output_stdout));
        println!("Stderr: \t\t{:?}", String::from_utf8_lossy(&res.output_stderr));
    }

}