// Copyright (c) 2025 Wrale LTD <contact@wrale.com>

use log::error;
use std::process;

mod application;
mod cli;
mod domain;
mod infrastructure;
mod interfaces;

fn main() {
    // Run the CLI application
    match cli::run() {
        Ok(_) => process::exit(0),
        Err(err) => {
            error!("Error: {}", err);
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}
