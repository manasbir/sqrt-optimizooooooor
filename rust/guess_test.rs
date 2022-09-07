use std::fs::File;
use std::io::prelude::*;
mod sqrt;
mod msb_test;
mod helpers;

fn main() {
    let mut file = File::create("sqrt_binary_lengths.txt").unwrap();
    sqrt::newton_sqrt(15);

    let mut i = 4;
    let mut string = String::new();
    while i < 9223372036854775808 {
        let (x, count1) = sqrt::newton_sqrt(i);
        let (guess, y) = msb_test::msb_sqrt(i);
        let (guess2, z) = sqrt::msb_sqrt(i);

        if helpers::is_closer_to(guess, guess2, x) {
            string.push_str(&format!("{i}, {x}, {guess}, {guess2} \n").to_string());
        }

        //string.push_str(&format!("{i:b} \n").to_string());
        //string.push_str(&format!("{x:b} \n \n \n").to_string());
        i*=2;
    }
    file.write_all(string.as_bytes()).unwrap();

}