
extern crate rustyline;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write; 
use std::vec::Vec;
use std::process::Command;
use std::path::PathBuf;
use rustyline::error::ReadlineError;
use rustyline::Editor;

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
                r1.add_history_entry(&line);
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

        println!("{:?}", procs);
        
        for process in procs {
            let cmd = process[0];

            if cmd == "exit" {
                builtin::exit(1); 
            } else {
                Command::new(cmd)
                    .spawn()
                    .expect("failed to execute process.");
            }
        }
    } 

    r1.save_history(".rsh_history").unwrap();

    Ok(())
}

/*
 * parse
 *  @arg tokens vector formed from splitting the input string
 *  @ret vec<vec<&str>> formed from the tokens vector, where each entry represents a process.
 *                      also, sets out/in/err redirection per global variables (maybe, don't know
 *                      yet.)
 */
fn parse(tokens: Vec<&str>) -> Vec<Vec<&str>> {
    let mut procs: Vec<Vec<&str>> = Vec::new();
    let mut process: Vec<&str> = Vec::new();

    if (tokens.is_empty()) {
        return procs; 
    }
    
    for token in tokens {
        /* TODO: Manage redirection.
        if token == "<" {
                    
        } else if token == ">" {
        
        } */

        process.push(token);
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
