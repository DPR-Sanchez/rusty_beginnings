use std::{error::Error, fs, path::PathBuf, thread, time::Duration};

use chrono::Local;
use csv::WriterBuilder;
use rayon::prelude::*;
use rexif::parse_file;

/// Return all files in `dir_path` whose extension matches `extension` (case‑insensitive).
fn find_files_by_extension(dir_path: &str, extension: &str) -> Vec<PathBuf> {
    fs::read_dir(dir_path)
        .into_iter()             // Option → iterator (empty if read_dir fails)
        .flatten()               // ReadDir → DirEntry values
        .filter_map(|entry_res| {
            let entry = entry_res.ok()?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.eq_ignore_ascii_case(extension) {
                        return Some(path);
                    }
                }
            }
            None
        })
        .collect()
}

/// Extract EXIF data for one file, returning a row of strings for CSV.
/// Logs and skips files whose EXIF cannot be read.
fn extract_exif(file_path: &PathBuf) -> Option<Vec<String>> {
    let path_str = file_path.to_string_lossy().into_owned(); // keep a String for the CSV row
    match parse_file(file_path) {
        Ok(exif) => {
            let mut row = Vec::new();
            row.push(path_str);                        // full path / filename
            row.push(exif.mime.to_string());           // MIME type
            row.push(exif.entries.len().to_string());  // # of tags
            for entry in &exif.entries {
                row.push(format!("{}: {}", entry.tag, entry.value_more_readable));
            }
            Some(row)
        }
        Err(e) => {
            eprintln!("Failed to parse EXIF in {}: {}", path_str, e);
            None
        }
    }
}

/// Write all rows to `exif_output.csv`, with a timestamp header line.
fn to_csv(rows: &[Vec<String>]) -> Result<(), Box<dyn Error>> {
    let now = Local::now();
    let mut wtr = WriterBuilder::new()
        .flexible(true)
        .from_path("exif_output.csv")?;

    // Comment‑style timestamp row (many CSV readers ignore lines that start with '#')
    wtr.write_record(&[format!("# csv_created_at: {}", now.to_rfc3339())])?;

    for row in rows {
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    println!("EXIF data written to exif_output.csv");

    Ok(())
    
}

fn main() {
    // Collect .jpeg and .jpg files from the current directory
    let mut files = find_files_by_extension(".", "jpeg");
    files.extend(find_files_by_extension(".", "jpg"));
    files.sort(); // deterministic ordering

    // Parallel EXIF extraction
    let exif_rows: Vec<Vec<String>> = files
        .par_iter()
        .filter_map(extract_exif)
        .collect();

    // Write results
    if let Err(e) = to_csv(&exif_rows) {
        eprintln!("Error writing CSV: {}", e);
    }
    //--- give users a moment to read the console output ---
    println!("Sleeping 30 seconds so you can read the message …");
    thread::sleep(Duration::from_secs(30));
}
