use std::mem::size_of;

fn main() {
    println!(
        "{0: <12} | {1: <12} | {2: <12} | {3: <12}",
        "size (bytes)", "verb", "big-endian", "little-endian"
    );
    for size in vec![size_of::<u16>(), size_of::<u32>()] {
        for item in vec![
            "GET", "HEAD", "POST", "PUT", "DELETE", "OPTIONS", "TRACE", "PATCH",
        ] {
            let mut trunc = String::from(item);
            // pad strings with spaces
            trunc = format!("{1: <0$}", 4, trunc);
            trunc.truncate(size);
            if size == 4 {
                let mut array: [u8; 4] = [0x20; 4];
                let raw_bytes = trunc.as_bytes();
                //            println!("before slice {:?}", raw_bytes);
                //            array.copy_from_slice(&[raw_bytes[0], raw_bytes[1], raw_bytes[2], raw_bytes[3]]);
                array.copy_from_slice(&raw_bytes[..4]);
                println!(
                    "{0: <12} | {1: <12} | {2: <12} | {3: <12}",
                    size,
                    item,
                    u32::from_be_bytes(array),
                    u32::from_le_bytes(array)
                );
            } else {
                let mut array: [u8; 2] = [0x20; 2];
                let raw_bytes = trunc.as_bytes();
                array.copy_from_slice(&raw_bytes[..2]);
                println!(
                    "{0: <12} | {1: <12} | {2: <12} | {3: <12}",
                    size,
                    item,
                    u16::from_be_bytes(array),
                    u16::from_le_bytes(array)
                );
            }
        }
    }
}
