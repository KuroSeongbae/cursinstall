use cursive::Cursive;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, TextView, LinearLayout, EditView, ListView, Checkbox, PaddedView};

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
            .child(TextView::new(package.clone())))
    }

    for group in config.grouped_packages.iter()
    {
        let mut group_view = LinearLayout::vertical().child(TextView::new(group.group_name.clone()));
        let mut packages_view = LinearLayout::vertical();

        for package in group.packages.iter() {
            packages_view.add_child(
                    LinearLayout::horizontal()
                        .child(Checkbox::new().checked())
                        .child(TextView::new(package.clone())));
        }

        let packages_view = PaddedView::lrtb(5, 0, 0, 0, packages_view);

        group_view.add_child(packages_view);
        grouped_packages_view.add_child(group_view);
    }

    s.add_layer(grouped_packages_view)
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
