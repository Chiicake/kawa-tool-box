use std::collections::HashMap;
use std::error::Error;
use calamine::{Reader, open_workbook, Xlsx, Data};

/// Convert excel file to json output, the first row of the excel file is the header
///
/// # Arguments
///
/// * `file_path` - The path to the excel file
///
/// # Returns
///
/// * `Ok(())` - If the file is read and written successfully
/// * `Err(Box<dyn Error>)` - If the file is not read or written successfully

pub fn excel_to_json(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut is_first_row = true;
    let mut header: Vec<String> = Vec::new();
    let mut workbook: Xlsx<_> = open_workbook(file_path).expect("Cannot open file");
    workbook.worksheet_range("Sheet1").expect("Cannot open sheet").rows().into_iter().for_each(|row| {
        if is_first_row {
            is_first_row = false;
            header = row.iter().map(|cell| cell.to_string()).collect();
            return;
        }
        let mut row_map: HashMap<String, String> = HashMap::new();
        for (i, cell) in row.iter().enumerate() {
            row_map.insert(header[i].clone(), cell.to_string());
        }
        let json = serde_json::to_string(&row_map).unwrap();
        println!("{},", json);
    });
    Ok(())
}
