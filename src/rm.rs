use std::{fs, path::Path};
use crate::err;

use crossterm::style::Stylize;




pub fn remove_file<P: AsRef<Path>>(path: P, check: bool) {
    let path = path.as_ref();
    
    if !path.exists() {
        err!("{}: No such file or directory", path.display());
        return;
    }

    if path.is_dir() {
        let p = inquire::Confirm::new(&format!(
                "{} is a directory, do you want to remove it? (y/N): ",
                path.display()
            ))
            .with_default(false)
            .prompt()
            .unwrap_or(false);
        
        if !p {
            println!("{} Aborted.", ">>>".cyan().dim());
            return;
        }

        fs::remove_dir_all(path).unwrap_or_else(|e| {
            err!("Failed to remove {}: {}", path.display(), e);
            return;
        });
    }        
    
    if path.is_file() {
        if check {
            let p = inquire::Confirm::new(&format!("Do you want to remove {}?", path.display()))
                .with_default(false)
                .prompt()
                .unwrap_or(false);
            if !p {
                println!("{} Aborted.", ">>>".cyan().dim());
                return;
            }
        }
        fs::remove_file(path).unwrap_or_else(|e| {
            err!("Failed to remove {}: {}", path.display(), e);
            return;
        });
    }
} 

