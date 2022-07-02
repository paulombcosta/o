use o_core::config::{get_default_config, get_config_folder};
use o_core::command::create_command;
use clap::{command, Command};
use clap_complete::{generate_to, shells::Zsh};
use std::io::Error;

fn main() -> Result<(), Error> {
    println!("Generating completions");
    let config = get_default_config();
    let commands: Vec<Command> = config.cmds.iter().map(|c| create_command(c)).collect();
    let mut command = command!().subcommands(commands);

    let config_folder = get_config_folder();

    let path = generate_to(Zsh, &mut command, "o", config_folder)?;
    println!("cargo:warning=completion file is generated: {:?}", path);
    Ok(())
}
