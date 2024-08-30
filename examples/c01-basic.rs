use std::{intrinsics::mir::place, path::PathBuf};

use clap::{arg, command, value_parser, ArgAction, Command};
use rust_xp_clap::error::Result;

fn main() -> Result<()> {
    let matches = command!()
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
             -c --config <FILE> "Sets a custom config file"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debug information on"

        ))
        .subcommand(
            Command::new("test")
                .about("Does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();
    if let Some(name) = matches.get_one::<String>("config") {
        println!("Value for name: {name}");
    }
    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }

    match matches.get_one::<u8>("debug") {
        Some(0) => println!("Debug mode is off"),
        Some(1) => println!("Debug mode is kind of on"),
        Some(2) => println!("Debug mode is on"),
        _ => println!("Whoah, slow down cowboy"),
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.get_flag("list") {
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...")
        }
    }
    Ok(())
}
