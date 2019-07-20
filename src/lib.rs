extern crate libc;
extern crate nix;
extern crate rustyline;
extern crate termion;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Write;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::vec::Vec;
use std::collections::HashMap;

use libc::c_int;
use nix::unistd::pipe;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use termion::{color, style};

mod builtin;

pub struct Config {
    pub filetype: String,
    pub filepath: String,
    
    //colors: HashMap<String, termion::color>,
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
            //colors: HashMap::new(),
        });
    }

    /// read config files and populate fields
    /// always uses the current self.filepath to 
    /// update fields
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
        println!("{}No previous history!{}", color::Fg(color::Red), style::Reset);
    }

    config.init_conf();

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
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        io::stdout().flush().unwrap();

        let tokens: Vec<&str> = input.split_whitespace().collect();
        let procs: Vec<Process> = parse(tokens);

        println!("{}{:?}{}", color::Fg(color::Green), procs, style::Reset);

        for process in procs {
            let cmd = process.cmd.unwrap();
            let args = process.args.unwrap();

            if cmd == "exit" {
                builtin::exit(1);
            } else {
                let childproc = Command::new(&cmd).args(args).output();

                match childproc {
                    Ok(child) => {
                        print!("{}", String::from_utf8_lossy(&child.stdout));
                        print!("{}", String::from_utf8_lossy(&child.stderr));
                    }
                    Err(e) => {
                        print!("rsh: command not found: {}", cmd);
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

impl fmt::Debug for Process {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Process")
            .field("cmd", &self.cmd)
            .field("args", &self.args)
            .finish()
    }
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

const REDIRECTION: [&str; 4] = [">", "<", ">>", "|"];

/*
 * parse
 *  @arg tokens vector formed from splitting the input string
 *  @ret vec<vec<&str>> formed from the tokens vector, where each entry represents a process.
 *                      also, sets out/in/err redirection per global variables (maybe, don't know
 *                      yet.)
 */
fn parse(tokens: Vec<&str>) -> Vec<Process> {
    let mut procs: Vec<Process> = Vec::new();
    let mut process: Process = Default::default();

    if tokens.is_empty() {
        return procs;
    }

    let mut cmd_ptr = 1; // point to the token right after the command.
    let mut arg_ptr = 1; // point to the token before the next command (or next carriage character)

    let mut args: Vec<String> = Vec::new();

    for (i, token) in tokens.iter().enumerate() {
        if cmd_ptr == arg_ptr {
            process.cmd = Some(token.to_string());
        } else if !(REDIRECTION.contains(&token)) {
            args.push(token.to_string());
        }

        if REDIRECTION.contains(&token) {
            // TODO: Set process stdout, stdin, stderr respectively.

            process.args = Some(args);
            procs.push(process);

            args = Vec::new();
            process = Default::default();

            cmd_ptr = arg_ptr + 1;
            arg_ptr = cmd_ptr;
            continue;
        }

        arg_ptr += 1;
    }

    process.args = Some(args);
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
