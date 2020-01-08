use clap::{App, Arg};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, ErrorKind, StdoutLock, Write};
use std::path::PathBuf;
use std::process::exit;

// This is cool!
#[cfg(target_os = "windows")]
const USAGE: &'static str = "
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &'static str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";
/// Holds the values of the program input
struct Program {
    number: bool,
    paths: Vec<String>,
}

fn main() {
    let matches = App::new("cat")
        .version("0.0.1")
        .about("Basic cat")
        .arg(Arg::with_name("show-all").short("f").long("show-all"))
        .arg(Arg::with_name("number").short("n").long("number"))
        .arg(Arg::with_name("FILE").index(1).multiple(true))
        .get_matches();

    //    let mut input_args = Program { number: false };

    let file = match matches.value_of("FILE") {
        Some(val) => val,
        None => "",
    };

    let multiple_files = match matches.values_of("FILE") {
        Some(val) => val.collect(),
        None => vec![""; 0],
    };

    println!("multiple files {:?}", multiple_files);

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    if file.is_empty() {
        print_or_die(&mut stdout, b"No such file or directory");
    }

    if let Err(err) = cat(file, &mut stdout) {
        if let Some(inner_error) = err.get_ref() {
            eprintln!("{}", inner_error);
        } else {
            eprintln!("{}", err);
        }
    }
}

fn cat(path: &str, stdout: &mut StdoutLock) -> io::Result<()> {
    let file = PathBuf::from(path);
    let metadata = fs::metadata(&file)?;
    if metadata.is_dir() {
        let err_message = format!("cat: {}: Is a directory", file.display());
        return Err(io::Error::new(ErrorKind::Other, err_message));
    }
    let file = File::open(file)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();

    loop {
        let num_read = reader.read_line(&mut line)?;
        if num_read == 0 {
            break;
        }
        stdout.write(line.as_bytes())?;
    }
    stdout.flush()?;
    Ok(())
}

fn print_or_die(stdout: &mut StdoutLock, message: &[u8]) {
    match stdout.write(message) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}
