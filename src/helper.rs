use cursive::Cursive;
use cursive::views::Checkbox;

use crate::structs::{Config, Cmd};
use std::fs;
use std::io::Write;
use std::sync::mpsc;
use std::process::{Command, Stdio};

use std::fs::File;

pub fn deserialize(path: &str) -> Result<Config, std::io::Error> {
    // serde_json::from_reader to read from file
    match fs::read_to_string(path) {
        Ok(json) => Ok(Config::map_from_json(serde_json::from_str(json.as_str())?)),
        Err(e) => Err(e)
    }
}

pub fn execute_commands_in_tui(executions: Vec<(String, Vec<String>)>, tx: &mpsc::Sender<String>, cb_sink: cursive::CbSink) {
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

pub fn write_commands_to_file(executions: Vec<(String, Vec<String>)>) -> std::io::Result<()>{
    let mut file_content = String::new();
    executions.iter().for_each(|cmd| {
        let cmd_with_args = format!("{} {}\n", cmd.0, cmd.1.join(" "));
        file_content.push_str(cmd_with_args.as_str());
    });

    let mut file = File::create("commands.sh")?;
    file.write_all(file_content.as_bytes())?;
    Ok(())
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
