mod runner;
use runner::{Runnable, RunnableProgram, RunnerPrinter, RunnerProgram};

fn main() {
    println!("Hello, world!");

    let runner_printer: RunnerPrinter = RunnerPrinter::init();
    runner_printer.run("test_seed");

    let runner_program: RunnerProgram = RunnerProgram::init("./crashy");
    runner_program.run("seed");
}
