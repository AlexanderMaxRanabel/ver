use colored::*;
use std::process::Command;

mod various;

pub fn git(main_argument: &str) {
    match main_argument {
        _ => {
            various::kill(&main_argument, "Unknown Command");
            std::process::exit(1);
        },
    }
} 
