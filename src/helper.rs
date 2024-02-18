use cursive::Cursive;
use cursive::views::{TextView, LinearLayout, Checkbox, TextContent,};

use crate::structs::Configuration;
use std::time::Duration;
use std::{fs, thread};
use std::sync::mpsc;
use std::process::{Command, Child, Stdio};

pub fn deserialize(path: &str) -> Result<Configuration, std::io::Error> {
    // serde_json::from_reader to read from file
    match fs::read_to_string(path) {
        Ok(json) => Ok(serde_json::from_str(json.as_str())?),
        Err(e) => Err(e)
    }
}

pub fn execute_commands(executions: Vec<(String, String)>, tx: &mpsc::Sender<String>, cb_sink: cursive::CbSink) {
    for execution in executions.iter() {
        tx.send(format!("{}...", execution.0.clone()));

        match Command::new(execution.1.clone()).stdout(Stdio::null()).status() {
            Ok(status) => {
                if status.success() {
                    tx.send("Success!".to_string());
                } else {
                    let code = match status.code() {
                        Some(code) => tx.send(format!("Failed: {}", code)),
                        None => tx.send("Failed!".to_string())
                    };
                }
            }
            Err(_) => {
                tx.send("Failed!".to_string());
                return;
            }
        }
    }
}

pub fn install_packages(install_command: String, packages: Vec<String>, tx: &mpsc::Sender<String>, cb_sink: cursive::CbSink) {
    // Command::new("sh").arg("-c").arg("ls").arg("&&").arg(install_command).arg(packages.join(" ")).spawn()

    tx.send("Installing...".to_string());

    match Command::new(install_command).arg(packages.join(" ")).stdout(Stdio::null()).status() {
        Ok(status) => {
            if status.success() {
                tx.send("Success!".to_string());
            } else {
                let code = match status.code() {
                    Some(code) => tx.send(format!("Failed: {}", code)),
                    None => tx.send("Failed!".to_string())
                };
            }
        }
        Err(_) => {
            tx.send("Failed!".to_string());
            return;
        }
    }

    // if tx.send("meow".to_string()).is_err() {
    //     return;
    // }

    cb_sink.send(Box::new(Cursive::noop)).unwrap();
}

pub fn validate_selection(s: &mut Cursive, config: &Configuration) -> Option<Vec<(String, String)>> {
    let mut executions: Vec<(String, String)> = Vec::new();

    let add_repos = match s.call_on_name("add_repo_check", |v: &mut Checkbox| {
        return v.is_checked();
    }) {
        Some(repo_selected) => repo_selected,
        None => false,
    };

    let repos = get_checked_repos(s);

    if add_repos && !repos.is_empty() {
        let command = format!("{} {}", config.add_repository_command, repos.join(" "));

        executions.push(("Adding Repos".to_string(), command));
    }

    s.call_on_name("sync_check", |v: &mut Checkbox| {
        if v.is_checked() {
             executions.push(("Sync".to_string(), format!("{}", config.sync_command)));   
        }
    });

    s.call_on_name("update_check", |v: &mut Checkbox| {
        if v.is_checked() {
             executions.push(("Update".to_string(), format!("{}", config.update_command)));   
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
        let command = format!("{} {}", config.install_command, packages.join(" "));

        executions.push(("Installing".to_string(), command));
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
