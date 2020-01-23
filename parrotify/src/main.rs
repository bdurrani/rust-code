use clap::{App, Arg};
mod constants;

fn main() {
    let matches = App::new("parrotify")
        .version("0.0.1")
        .about("parrot")
        .arg(Arg::with_name("string1").short("s1").long("string1"))
        .arg(Arg::with_name("string2").short("s2").long("string2"))
        .arg(Arg::with_name("message").index(1))
        .get_matches();

    println!("Hello, world! {:?}", constants::ENCODED_A);
}
