use clap::{command, ArgMatches, Command};
use o_core::cmd::Cmd;
use o_core::command::create_command;
use o_core::config::{get_default_config};
use std::env;
use std::io::Error;
use webbrowser;

fn main() -> Result<(), Error> {
    let config = get_default_config();
    let commands: Vec<Command> = config.cmds.iter().map(|c| create_command(c)).collect();
    let command = command!().subcommands(commands);

    let matches = command.get_matches();

    let matched_cmd = get_command_from_config_file(matches, config.cmds);

    match matched_cmd {
        Some(v) => webbrowser::open(&v)?,
        None => println!("command not found"),
    };

    Ok(())
}

fn get_command_from_config_file(cli_args: ArgMatches, config_file_cmds: Vec<Cmd>) -> Option<String> {
        config_file_cmds
        .iter()
        .map(|c| get_match_command(&cli_args, c))
        .filter(|result| result.is_some())
        .next()
        .unwrap()
}

fn get_match_command(argmatch: &ArgMatches, cmd: &Cmd) -> Option<String> {
    match argmatch.subcommand() {
        Some((name, sub_matches)) => {
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;
    use o_core::config::{get_config};

    fn get_matches(cli_args: Vec<&str>) -> (ArgMatches, Vec<Cmd>) {
        let config = get_config(PathBuf::from("../test"));
        let commands: Vec<Command> = config.cmds.iter().map(|c| create_command(c)).collect();
        let command = command!().subcommands(commands);
        (command.get_matches_from(cli_args), config.cmds)
    }

    #[test]
    fn test_should_extract_single_value() {
        let (matches, cmds)= get_matches(vec!["o", "reddit"]);
        let matched_cmd = get_command_from_config_file(matches, cmds);
        assert_eq!(matched_cmd, Some(String::from("https://reddit.com")));
    }

    #[test]
    fn test_should_get_value_from_nested_command() {
        let (matches, cmds)= get_matches(vec!["o", "github", "druid"]);
        let matched_cmd = get_command_from_config_file(matches, cmds);
        assert_eq!(matched_cmd, Some(String::from("https://github.com/linebender/druid")));
    }

    #[test]
    fn test_should_open_base_value_even_if_children_is_present() {
        let (matches, cmds)= get_matches(vec!["o", "sample"]);
        let matched_cmd = get_command_from_config_file(matches, cmds);
        assert_eq!(matched_cmd, Some(String::from("http://base.sample.com")));
    }

}