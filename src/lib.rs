use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write; 
use std::vec::Vec;
use std::process::Command;
use std::path::PathBuf;

mod builtin;
mod readline;

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
    let mut prompt: String = String::from(">>");
    let mut pwd: PathBuf;
    let mut running: bool = true;
    let stdin = io::stdin();
    
    // read config files

    // cmd loop
    while running {
        let pwd = builtin::pwd()?;
        
        print!("{} | {} ", pwd.display(), prompt);  
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        
        let tokens: Vec<&str> = input.split_whitespace().collect();
        let procs: Vec<Vec<&str>> = parse(tokens)?;

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

    Ok(())
}

/*
 * parse
 *  @arg tokens vector formed from splitting the input string
 *  @ret vec<vec<&str>> formed from the tokens vector, where each entry represents a process.
 *                      also, sets out/in/err redirection per global variables (maybe, don't know
 *                      yet.)
 */
fn parse(tokens: Vec<&str>) -> Result<(Vec<Vec<&str>>), Box<Error>> {
    let mut procs: Vec<Vec<&str>> = Vec::new();
    let mut process: Vec<&str> = Vec::new();
    
    for token in tokens {
        /* TODO: Manage redirection.
        if token == "<" {
                    
        } else if token == ">" {
        
        } */

        process.push(token);
    }

    procs.push(process);

    Ok((procs))
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
