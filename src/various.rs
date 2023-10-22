use colored::*;

pub fn kill(argument_name: &str, error_message: &str) {
    println!("{}: {} {}", "Error".red(), error_message, argument_name.cyan());
}
