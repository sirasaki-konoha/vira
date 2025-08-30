#![allow(unused_assignments)]
use std::process::exit;
use clap::Parser;
mod raw;
mod macros;
mod write;
mod stdin;
mod size;

/// Visual Fire Read System vira
#[derive(Parser)]
struct Cli {
    pub file: Option<String>,

    #[arg(short, long)]
    pub line: bool,

    /// Write the following string to a file
    #[arg(short, long, value_names = &["OPTION", "CONTENT"], num_args(1..))]
    pub write: Option<Vec<String>>,
    
    #[arg(short, long)]
    pub size: bool,

    #[arg(short, long)]
    pub remove: bool,

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
    } else if cmd.size {
        flag = true;
        size::get_size(&file_name);
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




