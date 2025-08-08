use kawa_tool_box;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let application = Application::builder()
        .application_id("com.kawa.kawatoolbox")
        .build();
    
    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("KawaToolBox")
            .default_width(500)
            .default_height(70)
            .build();

        // Create notebook container
        let notebook = gtk::Notebook::new();
        notebook.set_tab_pos(gtk::PositionType::Top);

        // First tab - Excel to JSON tool
        let excel_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        excel_box.set_homogeneous(true);

        let text_input = gtk::Entry::new();
        text_input.set_placeholder_text(Some("Excel file path"));
        excel_box.pack_start(&text_input, true, true, 0);

        let target_path = gtk::Entry::new();
        target_path.set_placeholder_text(Some("Target path"));
        excel_box.pack_start(&target_path, true, true, 0);


        // Add first tab
        let excel_label = gtk::Label::new(Some("Excel to JSON"));
        notebook.append_page(&excel_box, Some(&excel_label));

        // Create main horizontal box for layout
        let main_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        // Create output panel
        let output_scrolled = gtk::ScrolledWindow::new(Some(&gtk::Adjustment::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)), Some(&gtk::Adjustment::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)));
        output_scrolled.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        output_scrolled.set_min_content_width(500);

        let output_buffer = gtk::TextBuffer::new(Some(&gtk::TextTagTable::new()));
        let output_text = gtk::TextView::new();
        output_text.set_buffer(Some(&output_buffer));
        output_text.set_editable(false);
        output_text.set_cursor_visible(false);
        output_scrolled.add(&output_text);

        let output_label = gtk::Label::new(Some("Output"));
        let output_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        output_box.pack_start(&output_label, false, false, 0);
        output_box.pack_start(&output_scrolled, true, true, 0);

        // Add notebook and output panel to main layout
        main_box.pack_start(&notebook, true, true, 0);
        main_box.pack_start(&output_box, true, true, 0);

        // Modify button click to output to panel instead of console
        let button = Button::with_label("Excel to JSON");
        excel_box.pack_start(&button, true, true, 0);
        let output_buffer_clone = output_buffer.clone();
        button.connect_clicked(move |_| {
            let excel_path = text_input.text().to_string();
            let target_path = target_path.text().to_string();
            
            let mut output = String::new();
            
            if target_path.is_empty() {
                output.push_str("Error: Target path is empty\n");
            } else if excel_path.is_empty() {
                output.push_str("Error: Excel path is empty\n");
            } else {
                match kawa_tool_box::excel_to_json(&excel_path, &target_path) {
                    Ok(_) => output.push_str(&format!("Successfully converted: {}\n", excel_path)),
                    Err(e) => output.push_str(&format!("Error: {}\n", e))
                }
            }
            
            // Append to output buffer
            let mut end_iter = output_buffer_clone.end_iter();
            output_buffer_clone.insert(&mut end_iter, &output);
            
            // Scroll to bottom
            let end_mark = output_buffer_clone.create_mark(None, &output_buffer_clone.end_iter(), false).unwrap();
            output_text.scroll_to_mark(&end_mark, 0.0, false, 0.0, 0.0);
        });

        // // Second tab - Example tool
        // let tool2_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        // let tool2_label = gtk::Label::new(Some("Second tool (in development)"));
        // tool2_box.pack_start(&tool2_label, true, true, 0);
        //
        // // Add second tab
        // let tab_label = gtk::Label::new(Some("Tool 2"));
        // notebook.append_page(&tool2_box, Some(&tab_label));

        window.add(&main_box);
        window.show_all();
    });

    application.run();
}
