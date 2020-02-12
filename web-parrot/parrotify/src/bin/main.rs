use clap::{App, Arg};
use parrotify::line;

fn main() {
    let matches = App::new("parrotify")
        .version("0.0.1")
        .about("parrot")
        .arg(Arg::with_name("string1").long("string1").default_value("*"))
        .arg(Arg::with_name("string2").long("string2").default_value("-"))
        .arg(Arg::with_name("MESSAGE").index(1))
        .get_matches();

    let message = match matches.value_of("MESSAGE") {
        Some(msg) => msg,
        None => "",
    };

    if message.is_empty() {
        println!("No message found. Exiting");
        return;
    }

    println!("Message: {}", message);

    let str1 = matches.value_of("string1").unwrap();
    let str2 = matches.value_of("string2").unwrap();
    let mut line = line::Line::new();
    for item in message.chars() {
        line.add_letter(&item);
    }

    line.replace_a(&str2.chars().nth(0).unwrap());
    println!("{}", line);
}
