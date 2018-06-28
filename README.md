# RSH (**R**ush **SH**ell)
--------------------------

## Purpose
A project I'm using to solidify my knowledge of Rust. 

Hopefully, this will be more intense than a homework project.

## Features:
* Colored, customizable output.
* tab-completion
* Versatile prompts, improved on the ZSH style.
* Shared command line histories
* Globbing 
* Spelling corrections
* Command scrolling
* Directory aliasing
* Startup/shutdown scripts (think .bashrc, .zshrc, etc)
    * Optional; can start without shell init
* Extra utilities:
    * Command line calculator
    * Mass file/directory renames based on regex.

## Installation:
Simply `cargo run` for now.

## Dependencies:
* rust >= 1.23.0
* cargo >= 0.24.0 
* nix >= 0.11.0
* libc >= 0.2
* rustyline >= 1.0.0
