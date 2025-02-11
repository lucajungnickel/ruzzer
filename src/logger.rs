use crate::runner::RunnerProgramResult;


pub fn log_internal_error(res: &RunnerProgramResult) {
    println!("Internal Error while fuzzing");
    println!("Seed: \t\t\t{:?}", res.result.seed);
    println!("Seed Ascii: \t\t{:?}", String::from_utf8_lossy(&res.result.seed));
    println!("Return code: \t\t{:?}", res.return_code);
    println!("Stdout: \t\t{:?}", String::from_utf8_lossy(&res.output_stdout));
    println!("Stderr: \t\t{:?}", String::from_utf8_lossy(&res.output_stderr));
}

pub fn log_crash(res: &RunnerProgramResult) {
    println!("🎉 Crash Found! 🎉");
    println!("Seed: \t\t\t{:?}", res.result.seed);
    println!("Seed Ascii: \t\t{:?}", String::from_utf8_lossy(&res.result.seed));
    println!("Return code: \t\t{:?}", res.return_code);
    //println!("Stdout: \t\t{:?}", String::from_utf8_lossy(&res.output_stdout));
    //println!("Stderr: \t\t{:?}", String::from_utf8_lossy(&res.output_stderr));
}
