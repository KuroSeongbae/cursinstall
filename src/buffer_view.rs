use cursive::traits::*;
use cursive::Vec2;
use cursive::Printer;
use std::collections::VecDeque;
use std::sync::mpsc;

// Let's define a buffer view, that shows the last lines from a stream.
pub struct BufferView {
    // We'll use a ring buffer
    pub buffer: VecDeque<String>,
    // Receiving end of the stream
    pub rx: mpsc::Receiver<String>,
}

impl BufferView {
    // Creates a new view with the given buffer size
    pub fn new(size: usize, rx: mpsc::Receiver<String>) -> Self {
        let mut buffer = VecDeque::new();
        buffer.resize(size, String::new());
        BufferView { buffer, rx }
    }

    // Reads available data from the stream into the buffer
    pub fn update(&mut self) {
        // Add each available line to the end of the buffer.
        while let Ok(line) = self.rx.try_recv() {
            self.buffer.push_back(line);
            self.buffer.pop_front();
        }
    }
}

impl View for BufferView {
    fn layout(&mut self, _: Vec2) {
        // Before drawing, we'll want to update the buffer
        self.update();
    }

    fn draw(&self, printer: &Printer) {
        // Print the end of the buffer
        for (i, line) in self.buffer.iter().rev().take(printer.size.y).enumerate() {
            printer.print((0, printer.size.y - 1 - i), line);
        }
    }
}
