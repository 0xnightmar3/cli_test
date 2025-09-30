use clap::{command, Arg, Command, value_parser, ArgMatches, error::Error};

#[derive(Debug)]
pub enum Cmd {
    Pet,
    None,
    Person,
}

#[derive(Debug)]
pub struct Config {
    pub cmd: Option<Cmd>,
    pub is_fluffy: Option<bool>,
    pub pet_name: Option<String>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
}

fn to_config(m: &ArgMatches) -> Result<Config, Error> {
    let cmd = match m.subcommand() {
        Some(("person", _m)) => Some(Cmd::Person),
        Some(("pet", _m)) => Some(Cmd::Pet),
        _ => None,
    };
    let is_fluffy = m.get_one::<bool>("fluffy").copied();
    let pet_name = match m.subcommand() {
        Some(("pet", m)) => m.get_one::<String>("pet-name").cloned(),
        _ => None,
    };
    let first_name = match m.subcommand() {
        Some(("person", m)) => m.get_one::<String>("firstname").cloned(),
        _ => None,
    };
    let last_name = match m.subcommand() {
        Some(("person", m)) => m.get_one::<String>("lastname").cloned(),
        _ => None,
    };

    Ok(Config { cmd, is_fluffy, pet_name, last_name, first_name })
}

fn build_cli() -> Command {
    command!()
        .about("This application takes your name and registers you as a pedo.")
        .subcommand(
            Command::new("person")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .aliases(["fname", "firstname"])
                        .required(true)
                        .help("The persons first name")
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .aliases(["lname", "lastname"])
                        .required(true)
                        .help("Takes the persons last name")
                )
                )
        .subcommand(
            Command::new("pet")
                .arg(
                    Arg::new("petname")
                        .short('p')
                        .long("pet-name")
                        .aliases(["pname", "petname"])
                        .help("The name of your pet")
                )
        )
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
                .value_parser(value_parser!(bool))
        )
}

pub fn parse_cli_args() -> Result<Config, Error> {
    to_config(&build_cli().get_matches())
}

#[cfg(test)]
mod tests {
    #[test]
    fn matches_fluffy() {
        let matches = super::build_cli().try_get_matches_from(
            ["cmd", "--fluffy", "true"]
        ).unwrap();

        assert_eq!(true, *matches.get_one::<bool>("fluffy").unwrap());
        assert_ne!(false, *matches.get_one::<bool>("fluffy").unwrap());
    }

    #[test]
    fn matches_first_name() {
        let matches = super::build_cli().try_get_matches_from(
            ["cmd", "person", "-f", "Texhno", "-l", "Lain"]
        ).unwrap();

        assert_eq!("Texhno", matches.subcommand_matches("person").unwrap().get_one::<String>("firstname").unwrap());
    }
}
