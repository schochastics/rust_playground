use calamine::{open_workbook_auto, Data, Reader};
use std::error::Error;
use std::path::Path;
use std::time::Instant;

fn read_ods<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut workbook = open_workbook_auto(path)?;
    let sheet_names = workbook.sheet_names().to_owned();

    // Assuming you want to read the first sheet
    let sheet_name = &sheet_names[1];
    let range = workbook.worksheet_range(sheet_name).expect("failed");

    let mut data: Vec<Vec<String>> = Vec::new();

    for row in range.rows() {
        let row_data: Vec<String> = row
            .iter()
            .map(|cell| {
                match cell {
                    Data::String(s) => s.clone(),
                    Data::Float(f) => f.to_string(),
                    Data::Int(i) => i.to_string(),
                    Data::Bool(b) => b.to_string(),
                    Data::DateTime(dt) => dt.to_string(),
                    Data::DateTimeIso(dti) => dti.to_string(),
                    Data::DurationIso(dtr) => dtr.to_string(),
                    Data::Error(err) => format!("Error: {:?}", err),
                    Data::Empty => "".to_string(),
                    // Add handling for other data types as needed
                }
            })
            .collect();
        data.push(row_data);
    }

    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "example.ods";
    let start_time = Instant::now();
    let data = read_ods(path)?;
    let duration = start_time.elapsed(); // End timing

    // Print the duration
    println!("Reading ODS took: {:?}", duration);

    // For demonstration: print the data
    // for row in &data {
    //     println!("{:?}", row);
    // }

    Ok(())
}
