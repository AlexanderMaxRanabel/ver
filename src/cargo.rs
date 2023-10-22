use colored::*;
use std::process::Command;

fn kill(argument_name: &str, error_message: &str) {
    println!("{}: {} {}", "Error".red(), error_message, argument_name.cyan());
}

pub fn cargo(main_argument: &str) {
    match main_argument {
        _ => {
            kill(&main_argument, "Unknown Command");
            std::process::exit(1);
        },
    }
}
