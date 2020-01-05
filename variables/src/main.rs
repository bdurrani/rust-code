fn main() {
    //    let mut x = 5;
    //    println!("The value of x is {}", x);
    //    x = 6;
    //    println!("The value of x is {}", x);
    //    let guess: u32 = "42".parse().expect("Not a number");
    //
    //    let x: (i32, f64, u8) = (500, 6.4, 1);
    //    let one = x.1;
    another_function();

    let mut s = String::from("hello");
    function1(&mut s);
}

fn another_function() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    //    r1.push_str("glah");

    println!("{} {}", r1, r2);
}

fn function1(s: &mut String) {
    let r1 = &s;
    let r2 = &s;
    //    r1.push_str("glah");

    println!("{} {}", r1, r2);
}
