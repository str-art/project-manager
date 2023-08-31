use clap::{Arg, ArgAction, Command};

use super::constants::{ADD, CONFIG, LIST, NAME, NEW_NAME, PATH, REMOVE, RENAME};
use crate::config;

fn add() -> Command {
    Command::new(ADD)
        .arg(
            Arg::new(NAME)
                .required(true)
                .value_name("PROJECT NAME")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new(PATH)
                .value_name("PATH")
                .default_value(".")
                .required(false)
                .action(ArgAction::Set),
        )
}
fn remove() -> Command {
    Command::new(REMOVE).arg(
        Arg::new(NAME)
            .required(true)
            .value_name("PROJECT NAME")
            .action(ArgAction::Set),
    )
}
fn rename() -> Command {
    Command::new(RENAME)
        .arg(
            Arg::new(NAME)
                .required(true)
                .value_name("PROJECT NAME")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new(NEW_NAME)
                .required(true)
                .value_name("NEW PROJECT NAME")
                .action(ArgAction::Set),
        )
}
fn pm() -> Command {
    Command::new("pm")
        .about("Managing projects")
        .arg(
            Arg::new(CONFIG)
                .short('c')
                .long("config")
                .default_value(config::constants::DEFAULT_CONFIG_PATH)
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new(NAME)
                .required(false)
                .value_name("PROJECT_NAME")
                .action(ArgAction::Set),
        )
}
fn list() -> Command {
    Command::new(LIST)
}
pub fn cli() -> Command {
    pm().subcommand(add())
        .subcommand(remove())
        .subcommand(rename())
        .subcommand(list())
}
