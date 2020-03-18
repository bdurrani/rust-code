extern crate clap;
use clap::App;

/**
    Some reference implementations
    http://point-at-infinity.org/ssss/
*/

fn main() {
    App::new("myapp")
        .version("1.0")
        .about("Does great things!")
        .author("Kevin K.")
        .get_matches();
    println!("Hello, world!");
}
