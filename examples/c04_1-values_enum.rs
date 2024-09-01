// region:    --- Modules

use std::{fmt::Display, str::FromStr};

use clap::{arg, builder::PossibleValue, command, value_parser, ValueEnum};
pub use xp_clap::Result;

// endregion: --- Modules

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Mode {
    Energetic,
    Long,
    Hairy,
}

impl ValueEnum for Mode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Mode::Energetic, Mode::Long, Mode::Hairy]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Mode::Energetic => PossibleValue::new("energetic").help("The cockerspaniel"),
            Mode::Long => PossibleValue::new("long").help("the dachshund"),
            Mode::Hairy => PossibleValue::new("hairy").help("the havernese"),
        })
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no unskipped values")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant
                .to_possible_value()
                .expect("is not a possible value")
                .matches(s, false)
            {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

fn main() {
    let matches = cmd().get_matches();

    match matches
        .get_one::<Mode>("MODE")
        .expect("'MODE' is required and passing will fail if missing")
    {
        Mode::Energetic => println!("beanie"),
        Mode::Long => println!("sofus"),
        Mode::Hairy => println!("viggo"),
    }

    // Ok(())
}

fn cmd() -> clap::Command {
    command!().arg(
        arg!(<MODE>)
            .help("Which kind of dog are you looking for?")
            .value_parser(value_parser!(Mode)),
    )
}

#[test]
fn verify_cmd() {
    cmd().debug_assert();
}
