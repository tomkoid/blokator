use std::fs;
use std::process::exit;

use crate::initialize_colors::initialize_colors;

pub fn write_to_file(path: &str, contents: String) {
    let colors = initialize_colors();

    fs::write(path, contents).unwrap_or_else(|e| {
        println!(
            "{}error:{} Error occurred when writing to {}: {} (Kind: {})",
            colors.bold_red,
            colors.reset,
            path,
            e,
            e.kind()
        );
        exit(1);
    });
}
