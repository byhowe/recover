#[macro_export]
macro_rules! info {
    () => {println!()};
    ($($arg:tt)*) => {
        println!("{}", ansi_term::Color::Yellow.paint(format!($($arg)*)));
    }
}

#[macro_export]
macro_rules! error {
    () => {eprintln!()};
    ($($arg:tt)*) => {
        eprintln!("{}", ansi_term::Color::Red.paint(format!($($arg)*)));
    }
}

#[macro_export]
macro_rules! die {
    () => {{
        eprintln!();
        std::process::exit(1);
    }};
    ($($arg:tt)*) => {{
        eprintln!("{}", ansi_term::Color::Red.paint(format!($($arg)*)));
        std::process::exit(1);
    }}
}
