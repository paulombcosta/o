use clap::{command, ArgMatches, Command};
use o_core::cmd::Cmd;
use o_core::command::create_command;
use o_core::config::get_config;
use std::env;
use std::io::Error;
use webbrowser;

fn main() -> Result<(), Error> {
    let config = get_config();
    let commands: Vec<Command> = config.cmds.iter().map(|c| create_command(c)).collect();
    let command = command!().subcommands(commands);

    let matches = command.get_matches();

    let value: Option<String> = config
        .cmds
        .iter()
        .map(|c| get_match_command(&matches, c))
        .filter(|result| result.is_some())
        .next()
        .unwrap();

    match value {
        Some(v) => webbrowser::open(&v)?,
        None => println!("command not found"),
    };

    Ok(())
}

fn get_match_command(argmatch: &ArgMatches, cmd: &Cmd) -> Option<String> {
    match argmatch.subcommand() {
        Some((name, sub_matches)) => {
            println!("cmd name {:?}", cmd.name);
            if name != cmd.name {
                return None;
            }

            if cmd.value.is_some() {
                return cmd.value.clone();
            };

            match &cmd.children {
                Some(ch) => ch
                    .iter()
                    .map(|c| get_match_command(sub_matches, c))
                    .filter(|m| m.is_some())
                    .next()
                    .unwrap(),
                None => None,
            }
        }
        _ => None,
    }
}
