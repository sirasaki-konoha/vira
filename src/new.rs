use std::fs;
use std::path::Path;
use std::process::exit;

use crossterm::style::Stylize;
use inquire::Confirm;

use crate::{err, size};


pub fn create_file<P: AsRef<Path>>(path: P, check: bool) {
    let path = path.as_ref();

    if path.exists() {
        err!("'{}' File already exists.", path.display());
        return;
    }
    
    if check {
        let confirm = Confirm::new(&format!(
            "File {} does not exist. Do you want to create it? (y/N): ",
            path.display()
        ))
        .prompt()
        .unwrap_or(false);
        
        if !confirm {
            println!("{} Aborted.", ">>>".cyan().dim());
            return;
        }
    }

    fs::File::create(&path).unwrap_or_else(|e| {
        err!("Failed to create {}: {}", path.display(), e);
        exit(1);
    });

    size::get_size(path.as_os_str().to_str().unwrap());
}


pub fn create_directory<P: AsRef<Path>>(path: P, check: bool) {
    let path = path.as_ref();

    if path.exists() {
        err!("'{}': Directory already exists", path.display());
        return;
    }

    if check {
        let confirm = Confirm::new(&format!(
            "Directory {} does not exist. Do you want to create it? (y/N): ",
            path.display()
        ))
        .prompt()
        .unwrap_or(false);
        
        if !confirm {
            println!("{} Aborted.", ">>>".cyan().dim());
            return;
        }
    }

    fs::create_dir_all(path).unwrap_or_else(|e| {
        err!("Failed to create directory {}: {}", path.display(), e);
        exit(1);
    });

    println!("Directory {} created.", path.display());
}

