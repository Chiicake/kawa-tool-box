use std::env;
use kawa_tool_box;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tool_name = &args[1];
    match &tool_name[..] {
        "excel2json" => {
            kawa_tool_box::excel_to_json(&args[2]).unwrap();
        }
        _ => {
            println!("Unknown tool name");
        }
    }
}
