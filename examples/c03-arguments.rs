use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use xp_clap::Result;

fn main() -> Result<()> {
    let dog = "dog";
    let verbose = "verbose";
    let counter = "counter";
    let add = "add";
    let matches = command!()
        .arg(Arg::new(dog).short('d').long(dog).action(ArgAction::Append)) // each argument follows the arg name (eg -d viggo -d beanie)
        // .arg(Arg::new(dog).action(ArgAction::Append))
        .arg(
            Arg::new(verbose)
                .short('v')
                .long(verbose)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(counter)
                .short('c')
                .long(counter)
                .action(ArgAction::Count),
        )
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(add)
                .about("Adds files to command")
                .arg(arg!([NAME])),
        )
        .arg(
            arg!([PORT])
                .value_parser(value_parser!(u16))
                .default_value("2024"),
        )
        .get_matches();

    let args = matches
        .get_many::<String>(dog)
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    match matches.subcommand() {
        Some((add, sub_matches)) => println!(
            "used command {add}, name is {:?}",
            sub_matches
                .get_one::<String>("NAME")
                .expect("name provided")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    println!(
        "Port: {:2}",
        matches
            .get_one::<u16>("PORT")
            .expect("there should always be a value due to default")
    );

    // println!("dog: {:?}", matches.get_one::<String>(dog));
    println!("dogs: {:?}", &args);
    println!("verbose: {:?}", matches.get_flag(verbose));
    println!("counter: {:?}", matches.get_count(counter));

    Ok(())
}

// try to run this in another terminal:
// c03-arguments -d viggo -d beanie -v -c -c -c add tester
