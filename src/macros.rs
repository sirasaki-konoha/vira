#[macro_export]
macro_rules! err {
    ($($msg: expr), *) => {{
        use crossterm::style::Stylize;
        let fmt = format!($($msg), *);
        eprintln!(
            "{} {}",
            ">>>".red().bold(),
            fmt
        );
    }};
}

#[macro_export]
macro_rules! warn {
    ($($msg: expr), *) => {{
        use crossterm::style::Stylize;  
        let fmt = format!($($msg), *);
        eprintln!("{} {}", ">>>".yellow().bold(), fmt);
    }};
}
