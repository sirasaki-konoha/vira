use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use crate::err;
use crate::warn;


fn add<P: AsRef<Path>, T: AsRef<str>>(path: P, content: T) {
    let path = path.as_ref();
    let content = content.as_ref();
    let mut open_opts = OpenOptions::new();

    let mut f = open_opts.append(true).open(&path).unwrap_or_else(|e| {
        err!("Failed to open {}: {}", path.display(), e);
        exit(1);
    });
    
    let content = format!("{}", content);

    writeln!(f, "{}", content).unwrap_or_else(|e| {
        err!("Failed to write content to {}: {}", path.display(), e);
        exit(1);
    });
}

fn rewrite<P: AsRef<Path>, T: AsRef<str>>(path: P, content: T) {
    let path = path.as_ref();
    let content = content.as_ref();
    let mut file = File::create(&path).unwrap_or_else(|e| {
        err!("Failed to create {}: {}", path.display(), e);
        exit(1);
    });

    
    let content = format!("{}", content);
    writeln!(file, "{}", content).unwrap_or_else(|e| {
        err!("Failed to write content to {}: {}", path.display(), e);
        exit(1);
    });
}

fn help() {
    err!("Usage: --write [OPTIONS..] [CONTENT]");
    err!("Optinos:");
    err!("\t r (rewrite):");
    err!("\t\tRewrites the contents of the file (existing contents will be lost) ");
    err!("");
    err!("\t\tIf the file does not exist, create a new file");
    err!("\t o (overwrite):");
    err!("\t\tContinue writing the contents of the file as is.")
}

pub fn write_file<P: AsRef<Path>>(path: P,args: &[String]) {
    let mut over_write = false;
    let mut re_write = false;

    if args.is_empty() {
        help();
    }
    
    if args.len() == 1 {
        if args[0] == "o" || args[0] == "r" {
            warn!("Argument 2 is not specified. The default option (append) is used.");
            add(path, &args[0]);
            return;
        }
        
        if args[0] == "=help" {
            warn!("=help will rename to 'help'");
            add(&path, "help");
            return;
        } else if args[0] == "help" {
            help();
            warn!("To disable this help message. Plese chenge content 'help' to '=help'");
            return;
        }
        
        add(&path, &args[0]);
    }

    if args[0] == "overwrite" || args[0] == "o" {
        over_write = !over_write;
    } else if args[0] == "rewrite" || args[0] == "r" {
        re_write = !re_write;
    }
    
    let mut content = String::new();
    if !over_write && !re_write {
        for p in args[1..].iter() {
            content.push_str(&format!(" {}", p));
        }
    } else {
        content.push_str(&args[1]);
        if args.len() > 2 {
            for p in args[2..].iter() {
                content.push_str(&format!(" {}", p));
            }
        }
    }
    
    if re_write {
        rewrite(path, &content);
    } else if over_write {
        add(path, &content);
    } else {
        warn!("Argument 1 is not Available. The default option (append) is used.");
        add(path, &content);
    }

} 




