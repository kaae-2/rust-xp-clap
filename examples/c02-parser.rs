#[allow(unused_imports)]
use clap::{arg, command, ArgAction, Command};
use xp_clap::Result;

fn main() -> Result<()> {
    // let matches = Command::new("ParserTest")
    //     .version("0.1")
    //     .about("Does parser things")
    let matches = command!() // this pulls information from Cargo.toml
        .next_line_help(true)
        .arg(arg!(--two <VALUE>).required(true).action(ArgAction::Set))
        .arg(arg!(--one <VALUE>).required(true).action(ArgAction::Set))
        .get_matches();

    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );
    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );

    Ok(())
}
