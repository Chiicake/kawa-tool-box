use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver};
use gtk::{TextBuffer, TextView};
use gtk::prelude::{TextBufferExt, TextViewExt};

pub fn append_to_output(output_buffer: &TextBuffer, output_text: &TextView, text: &str) {
    // Append to output buffer
    let mut end_iter = output_buffer.end_iter();
    output_buffer.insert(&mut end_iter, text);

    // Scroll to bottom
    let end_mark = output_buffer.create_mark(None, &output_buffer.end_iter(), false).unwrap();
    output_text.scroll_to_mark(&end_mark, 0.0, false, 0.0, 0.0);
}

pub fn print_loading(output_buffer: &Arc<Mutex<TextBuffer>>, output_text: &Arc<Mutex<TextView>>,  rx: Receiver<String>) {
    let output_buffer = output_buffer.lock().unwrap();
    let output_text = output_text.lock().unwrap();
    // Print percentage, add 0.5% per second, to 100% while rx is received
    let mut percentage = 0.0;
    while let Ok(_text) = rx.recv() {
        if percentage < 90.0 {
            percentage += 0.5;
        }
        let loading_text = format!("Loading... {:.2}%", percentage);
        append_to_output(&output_buffer, &output_text, &loading_text);
    }
}
