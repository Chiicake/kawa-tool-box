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