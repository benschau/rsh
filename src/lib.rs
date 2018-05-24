use std::error::Error;
// use std::fs::File;
use std::io;
use std::io::Write; 
use std::vec::Vec;
// use std::io::prelude::*;
// use std::process::Command;
use std::path::PathBuf;

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
        
        let tokens: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();

        for token in tokens.iter() {
            print!("{} ", token); 
        }
    } 

    Ok(())
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
