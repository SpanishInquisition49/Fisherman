use colored::Colorize;
use std::process::exit;

/**
 * Print an error and exit if not specified with the and_exit flag
 */
pub fn log_error(message: &str, and_exit: bool) {
    eprintln!("{}{}", "[Error]:".red(), message);
    if and_exit {
        exit(1);
    }
}
