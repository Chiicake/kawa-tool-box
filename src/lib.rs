pub mod utils;

use std::collections::HashMap;
use std::error::Error;
use std::{fs};
use std::io::Write;
use calamine::{Reader, open_workbook, Xlsx};

/// Convert Excel file to json output, the first row of the Excel file is the header
///
/// # Arguments
///
/// * `file_path` - The path to the Excel file
/// * `target_path` - The path to the target json file
///
/// # Returns
///
/// * `Ok(())` - If the file is read and written successfully
/// * `Err(Box<dyn Error>)` - If the file is not read or written successfully

pub fn excel_to_json(file_path: &str, target_path: &str) -> Result<String, Box<dyn Error>> {
    let mut is_first_row = true;
    let mut header: Vec<String> = Vec::new();
    let mut workbook: Xlsx<_> = open_workbook(file_path)?;
    let range = workbook.worksheet_range("Sheet1")?;
    let mut output = String::new();
    
    range.rows().try_for_each(|row_result| {
        let row = row_result;
        if is_first_row {
            is_first_row = false;
            header = row.iter().map(|cell| cell.to_string()).collect();

            // Delete target file if exists and create new one
            if fs::metadata(target_path.trim()).is_ok() {
                fs::remove_file(target_path.trim())?;
            }
            fs::File::create(target_path.trim())?;
            return Ok::<(), Box<dyn Error>>(());
        }
        let mut row_map: HashMap<String, String> = HashMap::new();
        for (i, cell) in row.iter().enumerate() {
            row_map.insert(header[i].clone(), cell.to_string());
        }
        let json = serde_json::to_string(&row_map)? + ",\n";
        // Append to file
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(target_path.trim())?;
        file.write_all(json.as_bytes())?;
        output.push_str(json.as_str());
        Ok(())
    })?;
    Ok(output)
}
