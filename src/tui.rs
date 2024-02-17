use cursive::Cursive;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, TextView, LinearLayout, EditView, Checkbox, PaddedView, Button};

use std::sync::mpsc;
use std::thread;

use crate::buffer_view::*;

use crate::helper;
use crate::structs::Configuration;

pub fn create_path_dialog(s: &mut Cursive) {
    s.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new("File path:"))
            .child(EditView::new().content("packages.json")// TODO: remove.. just for testing
                .with_name("file_dialog")
                .full_width()
                .max_width(50)) 
        )
        .button("choose", |s| {
            let path = s.call_on_name("file_dialog", |view: &mut EditView| view.get_content()).unwrap();
            // deserialize file
            s.pop_layer();
            match helper::deserialize(&path) {
                Ok(config) => create_menu_config(s, config),
                Err(e) => create_text_dialog(s, format!("File not found: {}", e.to_string())),
            }
        })
        .button("exit", |s| s.quit())
    )
}

pub fn create_menu_config(s: &mut Cursive, config: Configuration) {
    let mut grouped_packages_view = LinearLayout::vertical();

    for package in config.packages.iter() {
        grouped_packages_view.add_child(LinearLayout::horizontal()
            .child(Checkbox::new().checked())
            .child(TextView::new(package.clone()))
            .with_name("package"))
    }

    for group in config.grouped_packages.iter()
    {
        let mut group_view = LinearLayout::vertical().child(
                LinearLayout::horizontal()
                    .child(Checkbox::new().checked())
                    .child(TextView::new(group.group_name.clone()))
                );
        let mut packages_view = LinearLayout::vertical();

        for package in group.packages.iter() {
            packages_view.add_child(
                    LinearLayout::horizontal()
                        .child(Checkbox::new().checked())
                        .child(TextView::new(package.clone()))
                        .with_name("package")
                );
        }

        let packages_view = PaddedView::lrtb(5, 0, 0, 0, packages_view);

        group_view.add_child(packages_view);
        grouped_packages_view.add_child(group_view);
    }

    let buttons = LinearLayout::horizontal()
        .child(Button::new("Execute", |s| {
            match helper::get_checked_packages(s) {
                Some(packages) => create_confirm_dialog(s, packages),
                None => create_error_dialog(s, "No packages selected".to_string())
            }
        }))
        .child(Button::new("Back", |s| {
            s.pop_layer();
            create_path_dialog(s);
        }))
        .child(Button::new("Quit", |s| s.quit()));

    grouped_packages_view.add_child(buttons);

    s.add_layer(grouped_packages_view)
}

pub fn create_confirm_dialog(s: &mut Cursive, packages: Vec<String>) {
    s.add_layer(Dialog::around(TextView::new(packages.join("\n")))
        .title("Following packages will be installed")
        .button("Accept", move |s| {
            s.pop_layer();
            create_install_dialog(s, packages.clone());
        })
        .button("Cancel", |s| { s.pop_layer(); }))
}

pub fn create_error_dialog(s: &mut Cursive, message: String) {
   s.add_layer(Dialog::around(
        TextView::new(message)
    )
    .title("Error")
    .button("back", |s| { s.pop_layer(); }))
}

// pub fn create_install_dialog(s: &mut Cursive, packages: Vec<String>) {
//     // TODO: capture output
//     let command = helper::install_packages("emerge".to_string(), packages);
//
//     let status = match command {
//         Ok(mut child) => {
//             match child.try_wait() {
//                 Ok(status) => {
//                     match status {
//                         Some(stat) => {
//                             if stat.success() {
//                                 s.quit();
//                                 String::from("Finished!")
//                             }
//                             else {
//                                 String::from("Failed!")
//                             }
//                         },
//                         None => {
//                             s.quit();
//                             String::from("Installing...")
//                         }
//                     }
//                 },
//                 Err(e) => String::from(format!("Status error: {}", e.to_string()))
//             }
//         },
//         Err(e) => String::from(format!("Command error: {}", e.to_string()))
//     };
//
//     s.add_layer(Dialog::around(
//         TextView::new(status)
//     )
//     .button("retry", |s| {
//             s.pop_layer();
//             create_path_dialog(s)
//     })
//     .button("exit", |s| s.quit())) 
// }

pub fn create_install_dialog(s: &mut Cursive, packages: Vec<String>) {
    let cb_sink = s.cb_sink().clone();

    // We want to refresh the page even when no input is given.
    s.add_global_callback('q', |s| s.quit());

    // A channel will communicate data from our running task to the UI.
    let (tx, rx) = mpsc::channel();

    // Generate data in a separate thread.
    thread::spawn(move || {
        helper::install_packages("emerge".to_string(), packages, &tx, cb_sink);
    });

    // And sets the view to read from the other end of the channel.
    s.add_layer(Dialog::around(
        BufferView::new(200, rx)
    )
    .button("retry", |s| {
            s.pop_layer();
            create_path_dialog(s)
    })
    .button("exit", |s| s.quit())) 
}

pub fn create_text_dialog(s: &mut Cursive, text: String) {
    s.add_layer(Dialog::around(
        TextView::new(text)
    )
    .button("retry", |s| {
            s.pop_layer();
            create_path_dialog(s)
    })
    .button("exit", |s| s.quit()))
}
