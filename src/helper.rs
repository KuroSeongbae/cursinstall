use crate::structs::Configuration;
use std::fs;
use std::io::Error;
use std::process::{Command, Child};

pub fn deserialize(path: &str) -> Result<Configuration, Error> {
    // serde_json::from_reader to read from file
    match fs::read_to_string(path) {
        Ok(json) => Ok(serde_json::from_str(json.as_str())?),
        Err(e) => Err(e)
    }
}

pub fn install_packages(install_command: String, packages: Vec<String>) -> std::io::Result<Child>{
    Command::new(install_command).arg(packages.join(" ")).spawn()
}
