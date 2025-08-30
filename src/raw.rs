use std::{error::Error, fs, io::{self, BufReader, Read, Write}, path::Path};





pub fn read_raw<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();
    let mut stdout = io::stdout().lock();
    let mut buff: Vec<u8> = Vec::new();
    
    let file = fs::File::open(path)?;
    let mut reader = io::BufReader::new(file);
    let _ = reader.read_to_end(&mut buff)?;
    
    stdout.write_all(&buff)?; 
    
    Ok(())
}


pub fn read_line<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();
    let mut stdout = io::stdout().lock();
    let mut buff: Vec<u8> = Vec::new();
    
    let inner = fs::File::open(path)?;
    let mut reader = BufReader::new(inner);
    reader.read_to_end(&mut buff)?;


    let mut buff_line: Vec<Vec<u8>> = Vec::new();
    let mut temp: Vec<u8> = Vec::new();
    let len = buff.len();
    for (curr, ch) in buff.iter().enumerate() {

        if *ch == b'\n' {
            temp.push(b'\n');
            buff_line.push(temp.clone());
            temp.clear();
        } else {
            temp.push(*ch);
        }
        
        if (curr + 1) == len {
            temp.push(b'\n');
            buff_line.push(temp.clone());
            temp.clear();
        }
    }
    
    let all_len_space=  buff_line.len().to_string().len();

    for (line, line_content) in buff_line.iter().enumerate() {
        print!("{:<width$} | ", line + 1, width = all_len_space);
        stdout.write_all(&line_content)?;
    }
    
    Ok(())
}
