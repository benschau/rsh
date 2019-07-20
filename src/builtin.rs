use std::env;
/**
 * builtin.rs
 *  rsh builtin commands, based of bash builtins.
 */
use std::error::Error;
use std::path::PathBuf;

static HELP: &'static str = "
    exit - exits the shell.     
";

/**
 * Prints the help message.
 */
pub fn help() {
    println!("{}", HELP);
}

/**
 * exit - logout of the session
 */
pub fn exit(code: i32) {
    ::std::process::exit(code);
}

/**
 * cd - change directory
 */
pub fn cd(path: PathBuf) {
    assert!(env::set_current_dir(path).is_ok());
}

/**
 * export - assign or create environmental variables
 */
pub fn export(env: &[String]) -> Result<(), Box<Error>> {
    Ok(())
}

/**
 * unset - unset environmental variable
 */
pub fn unset() -> Result<(), Box<Error>> {
    Ok(())
}

/**
 * hash - create a hash
 */
pub fn hash(string: &str) -> Result<(u32), Box<Error>> {
    Ok((0))
}

/**
 * pwd - present working directory
 */
pub fn pwd() -> Result<(PathBuf), Box<Error>> {
    Ok((env::current_dir().unwrap()))
}
