#![allow(unused_assignments)]
use std::{env, process::{self, exit}};
use clap::Parser;
use crossterm::style::Stylize;
mod raw;
mod macros;
mod write;
mod stdin;
mod size;
mod rm;
mod new;

const VIRA_VERSION: &'static str = "1.0";

/// Visual Fire Read System vira
#[derive(Parser)]
struct Cli {
    pub file: Option<String>,

    /// Display vira's version
    #[arg(short, long)]
    pub version: bool,

    /// Display file content line by line
    #[arg(short, long)]
    pub line: bool,

    /// Writes the following content to a specified file
    #[arg(short, long, value_names = &["OPTION", "CONTENT"], num_args(1..))]
    pub write: Option<Vec<String>>,
    
    /// Display specific file size
    #[arg(short, long)]
    pub size: bool,

    /// Remove a specified file
    #[arg(short, long)]
    pub remove: bool,

    /// Check before removing a file or directory or creates new file or directory
    #[arg(short, long)]
    pub check: bool,

    /// Creates a specified file
    #[arg(short, long)]
    pub new: bool,
    
    /// Creates a specified directory
    #[arg(short, long)]
    pub directory: bool,
}

fn main() {
    let bind = env::args().collect::<Vec<String>>();
    let exe_name = bind.first().unwrap();

    let cmd = Cli::parse();
    let mut flag = false;

    if cmd.version {
        version(&exe_name);
    }
    
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
    
    if cmd.new {
        flag = true;
        new::create_file(&file_name, cmd.check);
        return;
    }
    
    if cmd.directory {
        flag = true;
        new::create_directory(&file_name, cmd.check);
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


fn version(exe_name: &str) {
    println!("{} {} v{}", ">>>".cyan().bold(), exe_name, VIRA_VERSION);
    println!(
        "{} {} is licensed under BSD 3-Clause License.", 
        ">>>".cyan().bold(), 
        exe_name
    );
    exit(0);
}



