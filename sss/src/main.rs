extern crate clap;
use clap::App;
// use std::convert::TryInto;

// https://crypto.stackexchange.com/questions/12873/aes-mixcolumn-stage/12880#12880

/**
    Some reference implementations
    http://point-at-infinity.org/ssss/
*/
const MAX_DEGREE: i32 = 1024;

fn field_int(degree: i32) {}

fn mul(a1: u8, b1: u8) -> u8 {
    // https://crypto.stackexchange.com/questions/12873/aes-mixcolumn-stage/12880#12880
    let mut a: u8 = a1;
    let mut b: u8 = b1;
    let mut p: u8 = 0;
    let mut carry: u8;

    // println!("starting a: {:b}", a);
    // https://en.wikipedia.org/wiki/Multiplication_algorithm#Binary_or_Peasant_multiplication
    while b != 0 {
        if b & 1 == 1 {
            p ^= a;
        }
        carry = a & 0x80u8;
        a <<= 1;
        // println!("a: {:b}", a);
        if carry != 0 {
            a ^= 0x001b;
        }
        b >>= 1;
        // println!("carry: {:b} a: {:b} b: {:b} p: {:b}", carry, a, b, p);
    }
    // println!("result:{:b} 0x{:X} b:{:}", p, p, b);
    return p;
}

fn print_exp() {
    let mut x = 0x01;
    for power in 0..255usize {
        let res = mul(x, 0x03u8);
        print!("0x{:X}", res);
        if (power + 1) % 15 == 0 {
            println!();
        }
        x = res;
    }
}

fn main() {
    App::new("myapp")
        .version("1.0")
        .about("Does great things!")
        .author("Kevin K.")
        .get_matches();
    let input = "This is a secret";
    let opt_security = MAX_DEGREE;
    let log = [0_u8; 256];
    print_exp();
}

#[test]
fn math_works() {
    let res = mul(0xb6, 0x53);
    assert_eq!(res, 0x36);
    let res = mul(0x1a, 0x53);
    assert_eq!(res, 0x4f);
}
