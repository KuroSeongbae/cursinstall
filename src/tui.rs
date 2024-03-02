use cursive::Cursive;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Button, Checkbox, Dialog, EditView, LinearLayout, PaddedView, ScrollView, TextView};

use std::sync::mpsc;
use std::thread;

use crate::buffer_view::*;

use crate::helper;
use crate::structs::{Cmd, Config};

pub fn create_path_dialog(s: &mut Cursive) {
    s.add_layer(Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new("File path:"))
            .child(EditView::new().content("commands.json")// TODO: remove.. just for testing
                .with_name("file_dialog")
                .full_width()
                .max_width(50)) 
        )
        .button("choose", |s| {
            let path = s.call_on_name("file_dialog", |view: &mut EditView| view.get_content()).unwrap();
            // deserialize file
            s.pop_layer();
            match helper::deserialize(&path) {
                // Ok(config) => create_menu_config(s, config),
                Ok(config) => create_command_selection(s, config),
                Err(e) => create_text_dialog(s, format!("{}", e.to_string())),
            }
        })
        .button("exit", |s| s.quit())
    )
}

pub fn create_command_selection(s: &mut Cursive, config: Config) {
    let mut view = LinearLayout::vertical()
        .child(TextView::new(config.title.clone()))
        .child(TextView::new(config.comment.clone()));

    for (i, command) in config.commands.iter().enumerate() {
        // TODO: There must be a better way than cloning the clone
        let c = config.clone();

        // TODO: maybe it's possible to use events to navigate without extra button
        let command_cb = LinearLayout::horizontal()
                    .child(Checkbox::new().checked().with_name(format!("cmd{}", i)))
                    .child(TextView::new(format!("{}", command.0.command.clone())));
        let cmd_config_btn = Button::new("Configure Args", move |s| { 
            s.pop_layer();
            create_arg_config(s, c.clone(), i.clone())
        });

        view.add_child(PaddedView::lrtb(
            0, 0, 1, 0, 
            LinearLayout::vertical().child(command_cb).child(cmd_config_btn)));
    }

    let scroll_view_with_button = LinearLayout::vertical()
        .child(ScrollView::new(view))
        .child(Button::new("Execute", move |s| {
            match helper::validate_selection(s, config.clone()) {
                Some(executions) => create_confirmation_dialog(s, executions),
                None => create_error_dialog(s, "No Commands to run".to_string())
            }
        }));

    s.add_layer(scroll_view_with_button);
}

pub fn create_arg_config(s: &mut Cursive, config: Config, cmd: usize) {
    let command: Cmd = config.commands[cmd].0.clone();

    let mut view = LinearLayout::vertical().child(TextView::new(command.command.clone()));

    let mut ungrouped_view = LinearLayout::vertical();

    for (i, arg) in command.args.iter().enumerate() {
        let cb = Checkbox::new().with_checked(arg.1).with_name(format!("arg{}", i));
        let text = TextView::new(arg.0.clone());

        ungrouped_view.add_child(LinearLayout::horizontal()
            .child(cb)
            .child(text));
    }

    let mut grouped_view = LinearLayout::vertical();

    let mut group_iter: usize = 0;

    for (i, group) in command.grouped_args.iter().enumerate() {
        let mut group_view = LinearLayout::vertical().child(TextView::new(group.0.group_name.clone()));
        
        for arg in group.0.args.iter() {
            let cb = Checkbox::new().with_checked(arg.1).with_name(format!("group{}arg{}", group_iter, i));
            let text = TextView::new(arg.0.clone());

            group_view.add_child(PaddedView::lrtb(3, 0, 0, 0, LinearLayout::horizontal()
                .child(cb)
                .child(text)));
        }

        grouped_view.add_child(group_view);

        group_iter = group_iter + 1;
    }

    view.add_child(ScrollView::new(LinearLayout::vertical().child(ungrouped_view).child(grouped_view)));

    view.add_child(Button::new("Accept", move |s| {
        let mut c = config.clone();
        c.commands[cmd].0 = helper::check_args_for_command(s, command.clone());

        s.pop_layer();
        create_command_selection(s, c);
    }));

    s.add_layer(view);
}

pub fn create_confirmation_dialog(s: &mut Cursive, executions: Vec<(String, Vec<String>)>) {
    let mut executions_view = LinearLayout::vertical();
    for (i, execution) in executions.iter().enumerate() {
        executions_view.add_child(TextView::new(format!("{}: {} {}", i, execution.0.clone(), execution.1.join(" "))));
    }

    s.add_layer(Dialog::around(ScrollView::new(executions_view))
        .title("Following commands will be executed in order:")
        .button("Accept", move |s| {
            s.pop_layer();
            create_executions_dialog(s, executions.clone());
        })
        .button("Cancel", |s| { s.pop_layer(); }))
}

pub fn create_executions_dialog(s: &mut Cursive, executions: Vec<(String, Vec<String>)>) {
    let cb_sink = s.cb_sink().clone();

    // We want to refresh the page even when no input is given.
    s.add_global_callback('q', |s| s.quit());

    // A channel will communicate data from our running task to the UI.
    let (tx, rx) = mpsc::channel();

    // Generate data in a separate thread.
    thread::spawn(move || {
        helper::execute_commands(executions, &tx, cb_sink);
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

pub fn create_error_dialog(s: &mut Cursive, message: String) {
   s.add_layer(Dialog::around(
        TextView::new(message)
    )
    .title("Error")
    .button("back", |s| { s.pop_layer(); }))
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
