use clap::{App, Arg};
use std::io::{self, Error, StdoutLock, Write};
use std::process::exit;

fn main() {
    let matches = App::new("cat")
        .version("0.0.1")
        .about("Basic cat")
        .arg(Arg::with_name("show-all").short("f").long("show-all"))
        .arg(Arg::with_name("FILE").index(1).multiple(true))
        .get_matches();

    //    let file = matches.value_of("FILE").unwrap_or_else(|| "");

    let file = match matches.value_of("FILE") {
        Some(val) => val,
        None => "",
    };

    if file.is_empty() {
        println!("missing file argument");
        exit(1);
    }

    let printerrAndExit = |err: io::Error| -> ! {
        eprintln!("{}", err);
        exit(1);
    };

    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    let mut stdout = stdout.lock();
    //    stdout.write(b"hello").unwrap_or_else(printerrAndExit);
    print(&mut stdout, b"hello");
}

fn print(stdout: &mut StdoutLock, message: &[u8]) {
    match stdout.write(message) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}
