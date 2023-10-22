use std::env;
use colored::*;

mod git;
mod cargo;
mod various;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let main_argument = args.get(1).cloned().unwrap_or_else(|| {
            various::kill("<No Argument>", "Git Argument is not provided");
            std::process::exit(1);
        }); 

        match main_argument.as_str() {
            "--git" => {
                let git_main_argument = args.get(2).cloned().unwrap_or_else(|| {
                    various::kill("<No Argument>", "Git Argument is not provided");
                    std::process::exit(1);
                });
            },

            "--cargo" => {
                let git_main_argument = args.get(2).cloned().unwrap_or_else(|| {
                    various::kill("<No Argument>", "Git Argument is not provided");
                    std::process::exit(1);
                });
            },

            _ => {
                various::kill(&main_argument, "Unknown command");
            },
        }
    } else {
        println!("{}: A CLI Helper for Rust I made for myself", "ver".red());
        println!("{}: No Arguments were provided.", "Error".red());
        println!("Try: {} or {} for help", "--help".magenta(), "-h".magenta());
    }
}
