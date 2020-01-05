use std::io;
use rand::Rng;
use std::cmp::Ordering;

// https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/

fn main() {
    println!("Hello, world!");
    let secret_numer = rand::thread_rng().gen_range(1,101);
    println!("The secret numer is {}", secret_numer);
    loop {
        println!("Guess the number");
        let mut  guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed : {}", guess);

        match guess.cmp(&secret_numer){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
        }

    }
}
