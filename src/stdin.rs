use std::{io::{self, BufRead, Write}, process::exit};

use crate::err;



pub fn get_stdin_loop() {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut buff: Vec<u8> = Vec::new();
    

    loop {
        stdin.read_until(b'\n', &mut buff).unwrap_or_else(|e| {
            err!("Failed to read line: {}", e);
            exit(1);
        });
        
        stdout.write_all(&buff).unwrap_or_else(|e| {
            err!("Failed to write stdout: {}", e);
            exit(1);
        });
        
        buff.clear();
    }

}
