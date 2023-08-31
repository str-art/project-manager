mod cli;
mod config;
mod utils;
use std::error::Error;

use cli::{actions::list, constants::LIST};

use crate::config::structs::Config;
use crate::{
    cli::{
        actions::{add, goto, remove, rename},
        command::cli,
        constants::{ADD, CONFIG, NAME, REMOVE, RENAME},
    },
    config::constants::YES,
};

fn handle_parse_error(error: Box<dyn Error>, config_path: &str) {
    if error.to_string() != config::constants::CONFIG_NOT_FOUND {
        println!("Failed parsing config. {:?}", error);
        return ();
    }
    println!("Config not found. Generate config? y/n");
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed reading input");
    if input.as_str().trim() == YES {
        Config::create(config_path).unwrap();
    }
}
fn handle_command(config: Config, command: clap::Command) {
    let matches = command.get_matches();
    match matches.get_one::<String>(NAME) {
        Some(_) => goto(config, &matches),
        None => match matches.subcommand() {
            Some((ADD, sub_matches)) => add(config, sub_matches),
            Some((REMOVE, sub_matches)) => remove(config, sub_matches),
            Some((RENAME, sub_matches)) => rename(config, sub_matches),
            Some((LIST, _)) => list(config),
            Some((&_, _)) => {
                cli().print_help().unwrap();
            }
            None => {
                cli().print_help().unwrap();
            }
        },
    }
}
fn main() {
    let command = cli();
    let matches = command.get_matches();
    let config_path = matches.get_one::<String>(CONFIG).unwrap();
    match config::structs::Config::new(config_path) {
        Ok(config) => handle_command(config, cli()),
        Err(err) => handle_parse_error(err, config_path),
    }
}
