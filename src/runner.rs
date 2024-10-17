use std::process::Command;

pub enum State {
    Fail,
    Pass,
    Unresolved,
    InternalError,
}

pub trait Runnable {
    fn run(&self, seed: &str) -> RunnerResult;
}

pub trait RunnableProgram {
    fn run(&self, seed: &str) -> RunnerProgramResult;
}

pub struct RunnerResult {
    /* Result of the fuzzing run */
    state: State,
}


pub struct RunnerPrinter;

impl RunnerPrinter {
    pub fn init() -> RunnerPrinter {
        RunnerPrinter {

        }
    }
}

impl Runnable for RunnerPrinter {
    fn run(&self, seed: &str) -> RunnerResult {
        println!("{}", seed);
        RunnerResult { state: (State::Pass) }
    }
}


pub struct RunnerProgramResult {
    result: RunnerResult,
    /*
     * Defines the return_code of a run program.
     * Only valid if result != RunnerResult.Unresolved
     * 256 posix return codes posible 
     * https://www.gnu.org/savannah-checkouts/gnu/libc/manual/html_node/Exit-Status.html
     */
    return_code: u8,
    output_stdout: Vec<u8>,
    output_stderr: Vec<u8>,
}

pub struct RunnerProgram {
    program_name: String,
}

impl RunnableProgram for RunnerProgram {
    fn run(&self, seed: &str) -> RunnerProgramResult {
        let output_res = Command::new(self.program_name.clone())
        .arg(seed)
        .output();
        
        //handle run program:
        match output_res {
            Ok(value) => {
                let return_code = value.status.code().unwrap() as u8;
                RunnerProgramResult {
                    result: RunnerResult { 
                        state: State::Pass,
                     },
                    output_stdout: value.stdout,
                    output_stderr: value.stderr,
                    return_code: return_code,
                }
            },
            Err(e) => {
                eprint!("Error executing running of program: {}", e);
                RunnerProgramResult {
                    result: RunnerResult { state: State::InternalError },
                    output_stdout: Vec::new(),
                    output_stderr: Vec::new(),
                    return_code: 0,
                }
                
            },
        }


    }
}

impl RunnerProgram {
    pub fn init(program_name: &str) -> RunnerProgram {
        RunnerProgram {
            program_name: program_name.to_string(),
        }
    }
}