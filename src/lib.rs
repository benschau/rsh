
extern crate rustyline;
extern crate libc;
extern crate nix;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write; 
use std::vec::Vec;
use std::process::{Command, Stdio};
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::path::PathBuf;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use libc::c_int;
use nix::unistd::pipe;

mod builtin;

pub struct Config {
    pub filetype: String,
    pub filepath: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("too few arguments"); 
        }

        let conf_type = args[1].clone();
        let conf_fpath = args[2].clone();

        return Ok(Config { 
                    filetype: conf_type, 
                    filepath: conf_fpath,
                  });
    }

    pub fn init_conf(&self) {
    
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let stdin = io::stdin();
    
    let prompt: String = String::from(">> ");
    let mut pwd: PathBuf;
    let mut running: bool = true;
    let mut r1 = Editor::<()>::new();

    if let Err(_) = r1.load_history(".rsh_history") {
        println!("No previous history!"); 
    }
    
    // read config files

    // cmd loop
    while running {
        pwd = builtin::pwd()?;
        
        // print!("{} | {} ", pwd.display(), prompt);  
        
        // let mut input = String::new();
        // stdin.read_line(&mut input)?;
        
        let input;
        let readline = r1.readline(&prompt);
        match readline {
            Ok(line) => {
                if line.len() != 0 {
                    r1.add_history_entry(&line);
                }
                input = line;
            }, 
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
        io::stdout().flush().unwrap();
        
        let tokens: Vec<&str> = input.split_whitespace().collect();
        let procs: Vec<Vec<&str>> = parse(tokens);
        // let procs: Vec<Process> = parse(tokens);

        println!("{:?}", procs);
        
        for process in procs {
            let cmd = process[0];
            let len = process.len();

            if cmd == "exit" {
                builtin::exit(1); 
            } else {
                let childproc = Command::new(cmd)
                                        .args(&process[1..len])
                                        .output();
                    
                match childproc { 
                    Ok(child) => {
                        println!("{}", String::from_utf8_lossy(&child.stdout));
                        println!("{}", String::from_utf8_lossy(&child.stderr));
                    },
                    Err(e) => {
                        println!("rsh: command not found: {}", cmd);
                    }
                };
            }
        }
    } 

    r1.save_history(".rsh_history").unwrap();

    Ok(())
}

struct Process {
    stdin_fd: c_int,
    stdout_fd: c_int,
    stderr_fd: c_int,
    cmd: Option<String>,
    args: Option<Vec<String>>,
}

impl Default for Process {
    fn default() -> Self {
        Process {
            stdin_fd: 0,
            stdout_fd: 1,
            stderr_fd: 2,
            cmd: None,
            args: None,
        }
    }
}

/*
 * parse
 *  @arg tokens vector formed from splitting the input string
 *  @ret vec<vec<&str>> formed from the tokens vector, where each entry represents a process.
 *                      also, sets out/in/err redirection per global variables (maybe, don't know
 *                      yet.)
 */
fn parse(tokens: Vec<&str>) -> Vec<Vec<&str>> {
    // let mut procs: Vec<Vec<&str>> = Vec::new();
    // let mut process: Vec<&str> = Vec::new();
    let mut procs: Vec<Process> = Vec::new();
    let mut process: Process = Default::default();

    if (tokens.is_empty()) {
        return procs; 
    }

    process.cmd = Some(tokens[0].to_string());
    
    // for token in tokens {
    //     // TODO: Manage redirection.
    //     match token {
    //         ">" => println!("truncate/redirect to file"),
    //         "<" => println!("take input from file"),
    //         ">>" => println!("append/redirect to file"),
    //         "|" => println!("stdout of this proc to stdin of the next proc"),
    //         _  => process.push(token)
    //     }
    // }
    
    let mut cmd_ptr = 1; // point to the token right after the command.
    let mut arg_ptr = 1; // point to the token before the next command (or next carriage character)
    for token in tokens {


    }

    procs.push(process);

    return procs;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config() {
        let filetype = "startup";
        let filename = "./rshrc";
    }
}
