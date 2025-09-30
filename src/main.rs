mod cli;

use cli::parse_cli_args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = parse_cli_args()?;

    println!("Is fluffy: {}\nPet name: {}\nFirst name: {}\nLast name: {}\nCommand: {:?}", m.is_fluffy.unwrap_or(false), m.pet_name.unwrap_or("".to_string()), m.first_name.unwrap_or("".to_string()), m.last_name.unwrap_or("".to_string()), m.cmd.unwrap_or(cli::Cmd::None));

    Ok(())
}
