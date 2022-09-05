use std::fs::File;
use std::io::prelude::*;
mod sqrt;

fn main() {
    let mut file = File::create("sqrt_binary_lengths.txt").unwrap();
    sqrt::newton_sqrt(15);

    let mut i = 4;
    let mut string = String::new();
    while i < 9223372036854775808 {
        let (x, count1) = sqrt::newton_sqrt(i);
        let (result, y) = sqrt(i);
        
        string.push_str(&format!("{i:b} \n").to_string());
        string.push_str(&format!("{x:b} \n \n").to_string());
        i*=2;
    }
    file.write_all(string.as_bytes()).unwrap();

}

fn sqrt (n: u128) -> (u128, u128) {
    let result = 11;
    let mut x = n;
    let mut count = 0;

    if x >> 64 > 0 {
        x >>= 64;
        result <<= 32;
    }
    if x >> 32 > 0 {
        x >>= 32;
        result <<= 16;
    }
    if x >> 16 > 0 {
        x >>= 16;
        result <<= 8;
    }
    if x >> 8 > 0 {
        x >>= 8;
        result <<= 4;
    }
    if x >> 4 > 0 {
        x >>= 4;
        result <<= 2;
    }
    if x >> 2 > 0 {
        result <<= 1;
    }

    return (result, x);
}