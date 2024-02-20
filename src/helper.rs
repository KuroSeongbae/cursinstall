use cursive::Cursive;
use cursive::views::{TextView, LinearLayout, Checkbox};

use crate::structs::Configuration;
use std::fs;
use std::sync::mpsc;
use std::process::{Command, Stdio};

pub fn deserialize(path: &str) -> Result<Configuration, std::io::Error> {
    // serde_json::from_reader to read from file
    match fs::read_to_string(path) {
        Ok(json) => Ok(serde_json::from_str(json.as_str())?),
        Err(e) => Err(e)
    }
}

pub fn execute_commands(executions: Vec<(String, String, Vec<String>)>, tx: &mpsc::Sender<String>, cb_sink: cursive::CbSink) {
    // TODO: handle tx results :)
    for execution in executions.iter() {
        let _ = tx.send(format!("{}...", execution.0.clone()));

        match Command::new(execution.1.clone()).args(execution.2.clone()).stdout(Stdio::null()).status() {
        // match Command::new("emerge").args(vec!["cowsay".to_string(), "sl".to_string()]).status() {
            Ok(status) => {
                if status.success() {
                    let _ = tx.send("Success!".to_string());
                } else {
                    let _ = match status.code() {
                        Some(code) => tx.send(format!("Failed: {}", code)),
                        None => tx.send("Failed without status code :(".to_string())
                    };
                }
            }
            Err(_) => {
                let _ = tx.send("Failed to execute command!".to_string());
                return;
            }
        }
    }

    cb_sink.send(Box::new(Cursive::noop)).unwrap();
}

pub fn validate_selection(s: &mut Cursive, config: &Configuration) -> Option<Vec<(String, String, Vec<String>)>> {
    let mut executions: Vec<(String, String, Vec<String>)> = Vec::new();

    let add_repos = match s.call_on_name("add_repo_check", |v: &mut Checkbox| {
        return v.is_checked();
    }) {
        Some(repo_selected) => repo_selected,
        None => false,
    };

    let repos = get_checked_repos(s);

    if add_repos && !repos.is_empty() {
        executions.push(("Adding Repos".to_string(), config.add_repository_command.clone(), repos));
    }

    s.call_on_name("sync_check", |v: &mut Checkbox| {
        if v.is_checked() {
             executions.push(("Sync".to_string(), config.sync_command.clone(), Vec::new()));   
        }
    });

    s.call_on_name("update_check", |v: &mut Checkbox| {
        if v.is_checked() {
             executions.push(("Update".to_string(), config.update_command.clone(), Vec::new()));   
        }
    });

    let install = match s.call_on_name("install_check", |v: &mut Checkbox| {
        return v.is_checked();
    }) {
        Some(install_selected) => install_selected,
        None => false,
    };

    let packages = get_checked_packages(s);

    if install && !packages.is_empty() {
        executions.push(("Installing".to_string(), config.install_command.clone(), packages));
    }

    if executions.is_empty() {
        return None;
    }

    Some(executions)
}

pub fn get_checked_repos(s: &mut Cursive) -> Vec<String> {
    let mut repos: Vec<String> = Vec::new();

    // TODO: check for correct type
    s.call_on_all_named("repository", |f: &mut LinearLayout| {
        // let view: &mut LinearLayout = f.0;
        // TODO: the panics need to be replaced and ignored
        match (f.get_child(0), f.get_child(1)) {
            (Some(checkbox), Some(textview)) => {
                let checkbox: &Checkbox = match checkbox.as_any().downcast_ref::<Checkbox>() {
                    Some(cb) => cb,
                    None => return (),
                };

                let textview: &TextView = match textview.as_any().downcast_ref::<TextView>() {
                    Some(tv) => tv,
                    None => return (),
                };

                if checkbox.is_checked() {
                    repos.push(textview.get_content().source().to_string());
                }
            },
            _ => println!("meow"),
        }
    });

    repos
}

pub fn get_checked_packages(s: &mut Cursive) -> Vec<String> {
    let mut packages: Vec<String> = Vec::new();

    // TODO: check for correct type
    s.call_on_all_named("package", |f: &mut LinearLayout| {
        // let view: &mut LinearLayout = f.0;
        // TODO: the panics need to be replaced and ignored
        match (f.get_child(0), f.get_child(1)) {
            (Some(checkbox), Some(textview)) => {
                let checkbox: &Checkbox = match checkbox.as_any().downcast_ref::<Checkbox>() {
                    Some(cb) => cb,
                    None => return (),
                };

                let textview: &TextView = match textview.as_any().downcast_ref::<TextView>() {
                    Some(tv) => tv,
                    None => return (),
                };

                if checkbox.is_checked() {
                    packages.push(textview.get_content().source().to_string());
                }
            },
            _ => println!("meow"),
        }
    });

    packages
}
