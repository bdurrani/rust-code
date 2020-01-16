use std::mem;

fn main() {
    //    println!("size of u32: {}", mem::size_of::<u32>());
    let size = mem::size_of::<u32>();
    println!("{0: <12} | {1: <12}", "size (bytes)", "verb");
    for item in vec![
        "GET", "HEAD", "POST", "PUT", "DELETE", "OPTIONS", "TRACE", "PATCH",
    ] {
        let mut trunc = String::from(item);
        println!("{0: <12} | {1: <12}", size, trunc);
        // pad strings with spaces
        trunc = format!("{1: <0$}", size, trunc);
        //        println!("{}", trunc);
        trunc.truncate(size);
        let mut array: [u8; 4] = [0x20; 4];
        let raw_bytes = trunc.as_bytes();
        array.copy_from_slice(&raw_bytes);
        // array.copy_from_slice(raw_bytes);
        //        println!("{:x?}", array);
    }
    // returns raw bytes as a slice
    // https://doc.rust-lang.org/std/primitive.str.html#method.as_bytes
    // let raw_bytes_slice = command.as_bytes();
    // let raw_bytes_vector = raw_bytes_slice.to_ve
    // println!("raw_bytes {:?}", raw_bytes);
}
