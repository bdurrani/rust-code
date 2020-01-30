use clap::{App, Arg};
mod line;

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
    //    let test = "ABCD";
    //    let len = test.len();
    //    let width = constants::LETTER_WIDTH;
    //    let mut row1: Vec<u8> = Vec::with_capacity(width * len);
    //    row1.copy_from_slice(
    //        hash.get(&(test.chars().next().unwrap()).to_ascii_lowercase())
    //            .unwrap()[0],
    //    );
    //    row1.extend_from_slice(
    //        hash.get(&(test.chars().next().unwrap()).to_ascii_lowercase())
    //            .unwrap()[0],
    //    );
    //    row1.extend_from_slice(
    //        hash.get(&(test.chars().next().unwrap()).to_ascii_lowercase())
    //            .unwrap()[0],
    //    );
    //    for c in message.chars() {
    //        println!("{:?}", hash.get(&c.to_ascii_lowercase()).unwrap());
    //    }
    //    println!("{:?}", String::from_utf8(row1));
}
