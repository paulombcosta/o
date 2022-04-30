use crate::cmd::Cmd;
use clap::{command, Command};

pub fn create_command(cmd: &Cmd) -> Command {
    let command = command!().name(&cmd.name);
    match &cmd.children {
        None => command,
        Some(c) => {
            let cmds: Vec<Command> = c.iter().map(|c| create_command(c)).collect();
            command.subcommands(cmds)
        }
    }
}
