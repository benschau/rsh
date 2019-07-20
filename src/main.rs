extern crate libc;
extern crate rsh;

use std::env;
use std::process;

use rsh::Config;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        args.push(String::from("startup"));
        args.push(String::from("~/rshrc.conf"));
    }

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("could not parse arguments: {}", err);
        process::exit(1);
    });

    match rsh::run(config) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("shell encountered a problem: {}", e);
            process::exit(1);
        }
    }
}
