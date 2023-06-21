/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use advent_of_code::aoc_cli;
use std::process;

struct Args {
    day: u8,
    year: Option<u16>,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    Ok(Args {
        day: args.free_from_str()?,
        // Just overriding this since I don't want to have to specify '--year' all the time
        // yes, there's probably a better way to do this
        year: Some(2020),
    })
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Failed to process arguments: {e}");
            process::exit(1);
        }
    };

    if aoc_cli::check().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        process::exit(1);
    }

    match aoc_cli::read(args.day, args.year) {
        Ok(cmd_output) => {
            if !cmd_output.status.success() {
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("failed to spawn aoc-cli: {e}");
            process::exit(1);
        }
    }
}
