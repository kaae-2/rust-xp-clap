use clap::{arg, command};
pub use xp_clap::Result;

fn main() -> Result<()> {
    let energetic: &str = "energetic";
    let long = "long";
    let hairy = "hairy";
    let matches = command!()
        .arg(
            arg!(<MODE>)
                .help("Which kind of dog are you looking for?")
                .value_parser([energetic, hairy, long]),
        )
        .get_matches();

    match matches
        .get_one::<String>("MODE")
        .expect("'MODE' is required and parsing will fail if it is missing!")
        .as_str()
    {
        "energetic" => println!("beanie"),
        "long" => println!("sofus"),
        "hairy" => println!("viggo"),
        _ => unreachable!(),
    }

    Ok(())
}
