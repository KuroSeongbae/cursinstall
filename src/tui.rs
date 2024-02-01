use cursive::{Cursive, View};
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, TextView, LinearLayout, EditView, Checkbox, PaddedView, Button};

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
            .child(TextView::new(package.clone())).with_name("package"))
    }

    for group in config.grouped_packages.iter()
    {
        let mut group_view = LinearLayout::vertical().child(TextView::new(group.group_name.clone()));
        let mut packages_view = LinearLayout::vertical();

        for package in group.packages.iter() {
            packages_view.add_child(
                    LinearLayout::horizontal()
                        .child(Checkbox::new().checked())
                        .child(TextView::new(package.clone())).with_name("package")
                );
        }

        let packages_view = PaddedView::lrtb(5, 0, 0, 0, packages_view);

        group_view.add_child(packages_view);
        grouped_packages_view.add_child(group_view);
    }

    let buttons = LinearLayout::horizontal()
        .child(Button::new("Install", |s| create_install_dialog(s)))
        .child(Button::new("Back", |s| {
            s.pop_layer();
            create_path_dialog(s);
        }))
        .child(Button::new("Quit", |s| s.quit()));

    grouped_packages_view.add_child(buttons);

    s.add_layer(grouped_packages_view)
}

pub fn create_install_dialog(s: &mut Cursive) {
    let mut packages: String = String::new();

    // TODO: check for correct type
    s.call_on_all_named("package", |f: &mut LinearLayout| {
        // let view: &mut LinearLayout = f.0;

        match (f.get_child(0), f.get_child(1)) {
            (Some(checkbox), Some(textview)) => {
                let checkbox: &Checkbox = match checkbox.as_any().downcast_ref::<Checkbox>() {
                    Some(cb) => cb,
                    None => panic!("The Child is not a Checkbox"),
                };

                let textview: &TextView = match textview.as_any().downcast_ref::<TextView>() {
                    Some(tv) => tv,
                    None => panic!("The child is not a TextView"),
                };

                if checkbox.is_checked() {
                    packages.push_str(format!("{} ", textview.get_content().source()).as_str());
                }
            },
            _ => println!("meow"),
        }
    });

    s.add_layer(Dialog::around(
        TextView::new(packages)
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
