use std::{fs, process::exit};

use crossterm::style::Stylize;
use pretty_bytes::converter::convert;

use crate::err;




pub fn get_size(path: &str) {
    let file = fs::metadata(path).unwrap_or_else(|e| {
        err!("Failed to get file metadata: {}", e);
        exit(1);
    });
    
    let size = file.len();
    
    println!("{} {}: {}", ">>>".cyan().bold(), path, convert(size as f64));
}