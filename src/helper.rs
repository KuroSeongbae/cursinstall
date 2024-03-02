use cursive::Cursive;
use cursive::views::{TextView, LinearLayout, Checkbox};

use crate::structs::{Config, Cmd, Configuration};
use crate::json_structs::JsonConfig;
use std::fs;
use std::sync::mpsc;
use std::process::{Command, Stdio};

pub fn deserialize(path: &str) -> Result<Config, std::io::Error> {
    // serde_json::from_reader to read from file
    match fs::read_to_string(path) {
        Ok(json) => Ok(Config::map_from_json(serde_json::from_str(json.as_str())?)),
        Err(e) => Err(e)
    }
}

pub fn execute_commands(executions: Vec<(String, Vec<String>)>, tx: &mpsc::Sender<String>, cb_sink: cursive::CbSink) {
    // TODO: handle tx results :)
    for execution in executions.iter() {
        let _ = tx.send(format!("{}...", execution.0.clone()));

        match Command::new(execution.0.clone()).args(execution.1.clone()).stdout(Stdio::null()).stderr(Stdio::null()).status() {
        // match Command::new("emerge").args(vec!["cowsay".to_string(), "sl".to_string()]).status() {
            Ok(status) => {
                if status.success() {
                    let _ = tx.send("Success!".to_string());
                } else {
                    let _ = match status.code() {
                        Some(code) => tx.send(format!("Failed with status code: {}", code)),
                        None => tx.send("Failed without status code :(".to_string())
                    };
                }
            }
            Err(_) => {
                let _ = tx.send("Failed to execute command!".to_string());
                return;
            }
        }

        cb_sink.send(Box::new(Cursive::noop)).unwrap();
    }
}

pub fn validate_selection(s: &mut Cursive, config: Config) -> Option<Vec<(String, Vec<String>)>> {
    let mut executions: Vec<(String, Vec<String>)> = Vec::new();

    for (i, cmd) in config.commands.iter().enumerate() {
        let mut checked = false;
        s.call_on_name(format!("cmd{}", i).as_str(), |cb: &mut Checkbox| {
            checked = cb.is_checked();
        });

        if !checked {
            continue;
        }

        let mut cmd_with_args = separate_args_from_command(cmd.0.command.clone());

        cmd.0.args.iter().for_each(|arg| {
            if arg.1 {
                cmd_with_args.1.push(arg.0.clone());
            }
        });

        for garg in cmd.0.grouped_args.iter() {
            garg.0.args.iter().for_each(|arg| {
                if arg.1 {
                    cmd_with_args.1.push(arg.0.clone());
                }
            });
        }

        executions.push(cmd_with_args);
    }

    if executions.is_empty() {
        return None;
    }

    Some(executions)
}

// pub fn validate_selection(s: &mut Cursive, config: &Configuration) -> Option<Vec<(String, String, Vec<String>)>> {
//     let mut executions: Vec<(String, String, Vec<String>)> = Vec::new();
//
//     let add_repos = match s.call_on_name("add_repo_check", |v: &mut Checkbox| {
//         return v.is_checked();
//     }) {
//         Some(repo_selected) => repo_selected,
//         None => false,
//     };
//
//     let mut repos = get_checked_repos(s);
//
//     if add_repos && !repos.is_empty() {
//         let cmd_with_args = separate_args_from_command(config.add_repository_command.clone());
//         repos.splice(0..0, cmd_with_args.1.clone());
//         executions.push(("Adding Repos".to_string(), cmd_with_args.0, repos));
//     }
//
//     s.call_on_name("sync_check", |v: &mut Checkbox| {
//         if v.is_checked() {
//             let cmd_with_args = separate_args_from_command(config.sync_command.clone());
//             executions.push(("Sync".to_string(), cmd_with_args.0, cmd_with_args.1));   
//         }
//     });
//
//     s.call_on_name("update_check", |v: &mut Checkbox| {
//         if v.is_checked() {
//             let cmd_with_args = separate_args_from_command(config.update_command.clone());
//             executions.push(("Update".to_string(), cmd_with_args.0, cmd_with_args.1));   
//         }
//     });
//
//     let install = match s.call_on_name("install_check", |v: &mut Checkbox| {
//         return v.is_checked();
//     }) {
//         Some(install_selected) => install_selected,
//         None => false,
//     };
//
//     let mut packages = get_checked_packages(s);
//
//     if install && !packages.is_empty() {
//         let cmd_with_args = separate_args_from_command(config.install_command.clone());
//         packages.splice(0..0, cmd_with_args.1.clone());
//         executions.push(("Installing".to_string(), cmd_with_args.0, packages));
//     }
//
//     if executions.is_empty() {
//         return None;
//     }
//
//     Some(executions)
// }

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

pub fn check_args_for_command(s: &mut Cursive, mut cmd: Cmd) -> Cmd {
    for (i, arg) in cmd.args.iter_mut().enumerate() {
        s.call_on_name(format!("arg{}", i).as_str(), |cb: &mut Checkbox| {
            arg.1 = cb.is_checked();
        });
    }

    for (gi, group) in cmd.grouped_args.iter_mut().enumerate() {
        for (i, arg) in group.0.args.iter_mut().enumerate() {
             s.call_on_name(format!("group{}arg{}", gi, i).as_str(), |cb: &mut Checkbox| {
                arg.1 = cb.is_checked();
            });
        }
    }

    cmd
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

fn separate_args_from_command(command: String) -> (String, Vec<String>) {
    let split_cmd: Vec<String> = command.split(' ').map(|x| x.to_string()).collect();
    let mut cmd_with_args: (String, Vec<String>) = ("".to_string(), Vec::new());

    let mut count = 0;
    for split in split_cmd.iter() {
        if count == 0 {
            cmd_with_args.0 = split.clone();
        } else {
            cmd_with_args.1.push(split.clone());
        }

        count = count + 1;
    }

    cmd_with_args
}
