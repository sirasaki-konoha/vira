#![allow(unused_assignments)]
use std::process::exit;
use clap::Parser;
mod raw;
mod macros;
mod write;
mod stdin;
mod size;
mod rm;

/// Visual Fire Read System vira
#[derive(Parser)]
struct Cli {
    pub file: Option<String>,

    #[arg(short, long)]
    pub line: bool,

    /// Writes the following content to a specified file
    #[arg(short, long, value_names = &["OPTION", "CONTENT"], num_args(1..))]
    pub write: Option<Vec<String>>,
    
    // Display specific file size
    #[arg(short, long)]
    pub size: bool,

    /// Remove a specified file
    #[arg(short, long)]
    pub remove: bool,
    /// Check before removing a file
    #[arg(short, long)]
    pub check: bool,

    /// Creates a specified file
    #[arg(short, long)]
    pub new: bool,
}

fn main() {
    let cmd = Cli::parse();
    let mut flag = false;
    
    let Some(file_name) = cmd.file else {
        stdin::get_stdin_loop();
        exit(0);
    };

    if cmd.line {
        flag = true;
        raw::read_line(&file_name).unwrap_or_else(|e| {
            err!("Failed to open {}: {}", file_name, e);
            exit(1);
        });
    }

    if cmd.size {
        flag = true;
        size::get_size(&file_name);
    }
    
    if cmd.remove {
        flag = true;
        rm::remove_file(&file_name, cmd.check);
        return;
    }
    
    if let Some(write_cmd) = cmd.write {
        flag = true;
        write::write_file(&file_name, &write_cmd);
        return;
    }
    
    if flag {
        return;
    }
    
    raw::read_raw(&file_name).unwrap_or_else(|e| {
        err!("Failed to open {}: {}", file_name, e);
        exit(1);
    });
}




