use clap::{App, Arg};
mod constants;

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
    let hash = constants::build_map();

    let test = "A";
    for c in test.chars() {
        println!("{:?}", hash.get(&c.to_ascii_lowercase()));
    }
}
