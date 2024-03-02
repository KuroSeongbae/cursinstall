mod structs;
mod json_structs;
mod tui;
mod helper;
mod buffer_view;

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    // siv.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
    //                      .title("Choose File")
    //                      .button("Quit", |s| s.quit())
    //                         .button("Choose", read_file());

    tui::create_path_dialog(&mut siv);

    // Starts the event loop.
    siv.run();
}
