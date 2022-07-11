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

    let last_command = env::args().last().unwrap();

    let matched_cmd = get_command_from_config_file(matches, config.cmds, last_command);


    match matched_cmd {
        Some(v) => webbrowser::open(&v)?,
        None => println!("command not found"),
    };

    Ok(())
}

fn get_command_from_config_file(cli_args: ArgMatches, config_file_cmds: Vec<Cmd>, last_cmd: String) -> Option<String> {
        config_file_cmds
        .iter()
        .map(|c| get_match_command(&cli_args, c, &last_cmd))
        .filter(|result| result.is_some())
        .next()
        .unwrap()
}

fn get_match_command(argmatch: &ArgMatches, cmd: &Cmd, last_cmd: &String) -> Option<String> {
    match argmatch.subcommand() {
        Some((name, sub_matches)) => {
            if name != cmd.name {
                return None;
            }

             if cmd.value.is_some() && cmd.name == last_cmd.to_string() {
                return cmd.value.clone();
            };

            match &cmd.children {
                Some(ch) => ch
                    .iter()
                    .map(|c| get_match_command(sub_matches, c, last_cmd))
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

    fn get_matches(cli_args: Vec<&str>) -> (ArgMatches, Vec<Cmd>, String) {
        let last_arg = cli_args.last().unwrap().to_string();
        let config = get_config(PathBuf::from("../test"));
        let commands: Vec<Command> = config.cmds.iter().map(|c| create_command(c)).collect();
        let command = command!().subcommands(commands);
        (command.get_matches_from(cli_args), config.cmds, last_arg)
    }

    #[test]
    fn test_should_extract_single_value() {
        let (matches, cmds, last_arg)= get_matches(vec!["o", "reddit"]);
        let matched_cmd = get_command_from_config_file(matches, cmds, last_arg);
        assert_eq!(matched_cmd, Some(String::from("https://reddit.com")));
    }

    #[test]
    fn test_should_get_value_from_nested_command() {
        let (matches, cmds, last_arg)= get_matches(vec!["o", "github", "druid"]);
        let matched_cmd = get_command_from_config_file(matches, cmds, last_arg);
        assert_eq!(matched_cmd, Some(String::from("https://github.com/linebender/druid")));
    }

    #[test]
    fn test_should_open_base_value_even_if_children_is_present() {
        let (matches, cmds, last_arg)= get_matches(vec!["o", "sample"]);
        let matched_cmd = get_command_from_config_file(matches, cmds, last_arg);
        assert_eq!(matched_cmd, Some(String::from("http://base.sample.com")));
    }
    
    #[test]
    fn test_should_use_children_value_even_if_parent_has_value() {
        let (matches, cmds, last_arg)= get_matches(vec!["o", "sample", "nested1"]);
        let matched_cmd = get_command_from_config_file(matches, cmds, last_arg);
        assert_eq!(matched_cmd, Some(String::from("http://nested1.com")));
    }

}