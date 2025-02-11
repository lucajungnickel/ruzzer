use core::str;
use std::process::Command;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum State {
    Fail,
    Pass,
    Unresolved,
    InternalError,
}

#[allow(unused)]
pub trait Runnable {
    fn run(&self, seed: &Vec<u8>) -> RunnerResult;
}

pub trait RunnableProgram {
    /*
     * (Maybe) sanitizes the given seed, depending on the given SUT.
     * For example, nul bytes are removed if they are not allowed by the SUT setup. 
     */
    fn sanitize_seed(&self, seed: &mut Vec<u8>);
    fn run(&self, seed: &Vec<u8>) -> RunnerProgramResult;
}

#[derive(Debug, Clone)]
pub struct RunnerResult {
    /* Result of the fuzzing run */
    pub state: State,
    pub seed: Vec<u8>,
}


pub struct RunnerPrinter;

impl RunnerPrinter {
    #[allow(unused)]
    pub fn init() -> RunnerPrinter {
        RunnerPrinter {

        }
    }
}

impl Runnable for RunnerPrinter {
    fn run(&self, seed: &Vec<u8>) -> RunnerResult {
        println!("{}", String::from_utf8_lossy(seed));
        RunnerResult { 
            state: (State::Pass),
            seed: seed.clone() }
    }
}


pub struct RunnerProgramResult {
    pub result: RunnerResult,
    /*
     * Defines the return_code of a run program.
     * Only valid if result != RunnerResult.Unresolved
     * 256 posix return codes posible 
     * https://www.gnu.org/savannah-checkouts/gnu/libc/manual/html_node/Exit-Status.html
     */
    pub return_code: u8,
    pub output_stdout: Vec<u8>,
    pub output_stderr: Vec<u8>,
}

#[allow(unused)]
pub fn print_runner_result(result: RunnerResult) {
    println!("State: {:?}", result.state);
}

#[allow(unused)]
pub fn print_runner_program_result(result: RunnerProgramResult) {
    print_runner_result(result.result);
    println!("Return code: {}", result.return_code);
    println!("Stdout: {:?}",result.output_stdout);
    println!("Stderr: {:?}",result.output_stderr);
    println!("Stdout Ascii: {:?}", String::from_utf8_lossy(&result.output_stdout));
    println!("Stderr Ascii: {:?}", String::from_utf8_lossy(&result.output_stderr));
}

fn evaluate_return_code(return_code: u8) -> State {
    match return_code {
        0 => State::Pass,
        _ => State::Fail,
    }
}
pub struct RunnerProgram {
    program_name: String,
}

impl RunnableProgram for RunnerProgram {

    fn sanitize_seed(&self, seed: &mut Vec<u8>) {
        seed.retain(|&byte| byte != 0);    
    }

    fn run(&self, seed: &Vec<u8>) -> RunnerProgramResult {
        let arg: String = seed.iter()
        .filter_map(|&b| if b.is_ascii() { Some(b as char) } else { None })
        .collect();

        let output_res = Command::new(self.program_name.clone())
        .arg(&arg)
        .output();
        
        //handle run program:
        match output_res {
            Ok(value) => {
                let return_code = value.status.code().unwrap() as u8;
                RunnerProgramResult {
                    result: RunnerResult { 
                        state: evaluate_return_code(return_code),
                        seed: seed.clone()
                     },
                    output_stdout: value.stdout,
                    output_stderr: value.stderr,
                    return_code: return_code,
                }
            },
            Err(e) => {
                eprint!("Error executing running of program: {}", e);
                RunnerProgramResult {
                    result: RunnerResult { 
                        state: State::InternalError,
                        seed: seed.clone()
                    },
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

