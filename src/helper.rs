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

// pub fn install_packages(install_command: String, packages: Vec<String>, text_content: TextContent) -> std::io::Result<Child> {
//     // Command::new("sh").arg("-c").arg("ls").arg("&&").arg(install_command).arg(packages.join(" ")).spawn()
//     match Command::new("sh").arg("-c").arg(install_command).arg(packages.join(" ")).status() {
//         Ok(status) => {
//             if status.success() {
//                 text_content.set_content("Sucess!");
//             } else {
//                 text_content.set_content("Failed");
//             }
//         }
//         Err => text_content.set_content("No Status :(")
//     }
// }

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

pub fn get_checked_packages(s: &mut Cursive) -> Option<Vec<String>> {
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

    if packages.is_empty() {
        return None
    }

    Some(packages)
}
