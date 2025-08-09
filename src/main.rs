use std::cell::RefCell;
use std::rc::Rc;
use kawa_tool_box;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, TextBuffer, TextView};
use rig::{completion::{Completion, Prompt}, providers};
use rig::agent::Agent;
use rig::client::completion::{CompletionClientDyn, CompletionModelHandle};

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
        output_scrolled.set_min_content_width(800);
        output_scrolled.set_max_content_width(800);

        let output_buffer = gtk::TextBuffer::new(Some(&gtk::TextTagTable::new()));
        let output_text = gtk::TextView::new();
        output_text.set_buffer(Some(&output_buffer));
        output_text.set_editable(false);
        output_text.set_cursor_visible(false);
        output_text.set_wrap_mode(gtk::WrapMode::Word);
        output_scrolled.add(&output_text);

        let output_label = gtk::Label::new(Some("Output"));
        let output_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        output_box.pack_start(&output_label, false, false, 0);
        output_box.pack_start(&output_scrolled, true, true, 0);

        // Add notebook and output panel to main layout
        main_box.pack_start(&notebook, false, true, 0);
        main_box.pack_start(&output_box, true, true, 0);

        // Modify button click to output to panel instead of console
        let button = Button::with_label("Excel to JSON");
        excel_box.pack_start(&button, true, true, 0);
        // 修改按钮点击事件处理代码
        let output_buffer_clone = output_buffer.clone();
        let output_text_clone = output_text.clone();
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

            append_to_output(&output_buffer_clone, &output_text_clone, &output);
        });






        // AI Transfer tab
        let tool2_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        tool2_box.set_spacing(10);
        tool2_box.set_margin_top(20);
        tool2_box.set_margin_bottom(20);
        tool2_box.set_margin_start(20);
        tool2_box.set_margin_end(20);

        // Initial state with button
        let init_button = gtk::Button::with_label("Init");
        init_button.set_size_request(100, 40);
        tool2_box.pack_start(&init_button, false, false, 0);
        

        // Create success UI elements (initially hidden)
        let success_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        success_box.set_visible(false);

        let input_entry = gtk::Entry::new();
        input_entry.set_placeholder_text(Some("Input..."));

        let send_button = gtk::Button::with_label("Submit");

        let entry_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        entry_box.pack_start(&input_entry, true, true, 0);
        entry_box.pack_start(&send_button, false, false, 0);

        success_box.pack_start(&entry_box, false, false, 0);
        tool2_box.pack_start(&success_box, true, true, 0);


        let output_buffer_clone2 = output_buffer.clone();
        let output_text_clone2 = output_text.clone();
        let agent = Rc::new(RefCell::new(None));
        let agent_clone = Rc::clone(&agent);
        
        init_button.connect_clicked(move |_| {
            // Placeholder initialization logic
            let res = init();
            match res {
                Ok(agent1) => {
                    *agent_clone.borrow_mut() = Some(agent1);
                },
                Err(e) => {
                    append_to_output(&output_buffer_clone2, &output_text_clone2, &format!("Error: {}", e));
                    return;
                }
            };


            append_to_output(&output_buffer_clone2, &output_text_clone2, "Init success");

            // let mut message = String::new();
            // let res = chat(agent_instance, "Hi");
            // match res {
            //     Ok(res) => message = res,
            //     Err(e) => {
            //         append_to_output(&output_buffer_clone2, &output_text_clone2, &format!("Error: {}", e));
            //     }
            // }
            // append_to_output(&output_buffer_clone2, &output_text_clone2, &message);
        });

        let output_buffer_clone3 = output_buffer.clone();
        let output_text_clone3 = output_text.clone();
        send_button.connect_clicked(move |_| {
            let input = input_entry.text().to_string();
            if input.is_empty() {
                return;
            }
            let mut message = String::new();
            let agent_clone2 = Rc::clone(&agent);
            let agent_ref = agent_clone2.borrow();
            let agent_instance = match agent_ref.as_ref() {
                Some(agent) => agent,
                None => {
                    append_to_output(&output_buffer_clone3, &output_text_clone3, "Error: Agent not initialized\n");
                    return;
                }
            };

            let res = chat(agent_instance, &input);
            match res {
                Ok(res) => message = res,
                Err(e) => {
                    append_to_output(&output_buffer_clone3, &output_text_clone3, &format!("Error: {}", e));
                }
            }
            append_to_output(&output_buffer_clone3, &output_text_clone3, &message);
        });

        // Add AI Transfer tab
        let tab_label = gtk::Label::new(Some("\u{41}\u{49} transfer"));
        notebook.append_page(&tool2_box, Some(&tab_label));

        window.add(&main_box);
        window.show_all();
    });

    application.run();
}

fn append_to_output(output_buffer: &TextBuffer, output_text: &TextView, text: &str) {
    // Append to output buffer
    let mut end_iter = output_buffer.end_iter();
    output_buffer.insert(&mut end_iter, text);

    // Scroll to bottom
    let end_mark = output_buffer.create_mark(None, &output_buffer.end_iter(), false).unwrap();
    output_text.scroll_to_mark(&end_mark, 0.0, false, 0.0, 0.0);
}

#[tokio::main]
async fn init() ->Result<Agent<CompletionModelHandle<'static>>, Box<dyn core::error::Error>> {
    let client = providers::ollama::Client::builder().base_url("http://localhost:11434/").build().unwrap();
    let v1 = client.agent("qwen3:32B") // .agent("deepseek-r1:latest")
    // preamble
    .preamble("")
    .build();

    Ok(v1)
}

#[tokio::main]
async fn chat(agent: &Agent<CompletionModelHandle<'static>>, input: &str) -> Result<String, Box<dyn core::error::Error>> {
    let res = agent.prompt(input).await?;
    Ok(res)
}