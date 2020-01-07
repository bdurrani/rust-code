use clap::{App, Arg};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, StdoutLock, Write, ErrorKind};
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

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    if file.is_empty() {
        print_or_die(&mut stdout, b"No such file or directory");
    }

    if let Err(err) = cat(file, &mut stdout){
        print_or_die(&mut stdout, b"Something happened");
    }
}

fn cat(path: &str, stdout: &mut StdoutLock) -> io::Result<()> {
    let file = PathBuf::from(path);
    let metadata = fs::metadata(&file)?;
    if metadata.is_dir(){
        return Err(io::Error::new(ErrorKind::Other,"This is a directory"));
    }
    let file = File::open(file)?;
    let mut reader = BufReader::new(file);

    loop {
        let mut line = String::new();
        let num_read = reader.read_line(&mut line)?;
        if num_read == 0 {
            break;
        }
        stdout.write(line.as_bytes())?;
        //        let num_read = file.read(&mut buf)?;
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
