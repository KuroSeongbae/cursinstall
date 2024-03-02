use serde::{Deserialize, Serialize};
use crate::json_structs;

#[derive(Clone)]
pub struct Config {
    pub title: String,
    pub comment: String,
    pub commands: Vec<(Cmd, bool)>
}

impl Config {
    pub fn map_from_json(json_config: json_structs::JsonConfig) -> Self {
        Self {
            title: json_config.title,
            comment: json_config.comment,
            commands: json_config.commands.iter().map(|x| (Cmd::map_from_json(x), true)).collect()
        }
    }
}

#[derive(Clone)]
pub struct Cmd {
    pub command: String,
    pub args: Vec<(String, bool)>,
    pub grouped_args: Vec<(ArgGroup, bool)>
}

impl Cmd {
    pub fn map_from_json(json_cmd: &json_structs::JsonCmd) -> Self {
        Self {
            command: json_cmd.command.clone(),
            args: json_cmd.args.iter().map(|x| (x.to_string(), true)).collect(),
            grouped_args: json_cmd.grouped_args.iter().map(|x| (ArgGroup::map_from_json(x), true)).collect(),
        }
    }
}

#[derive(Clone)]
pub struct ArgGroup {
    pub group_name: String,
    pub args: Vec<(String, bool)>,
}

impl ArgGroup {
    pub fn map_from_json(json_arg_group: &json_structs::JsonArgGroup) -> Self {
        Self {
            group_name: json_arg_group.group_name.clone(),
            args: json_arg_group.args.iter().map(|x| (x.to_string(), true)).collect(), 
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub add_repository_command: String,
    pub repositories: Vec<String>,
    pub sync_command: String,
    pub update_command: String,
    pub install_command: String,
    pub packages: Vec<String>,
    pub grouped_packages: Vec<PackageGroup>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self { 
            add_repository_command: String::new(),
            repositories: Vec::new(), 
            sync_command: String::new(),
            update_command: String::new(),
            install_command: String::new(),
            packages: Vec::new(),
            grouped_packages: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageGroup {
    pub group_name: String,
    pub packages: Vec<String>,
}
