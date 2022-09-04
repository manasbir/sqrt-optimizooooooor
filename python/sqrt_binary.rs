use std::fs::File;
use std::io::prelude::*;
mod sqrt;

fn main() -> std::io::Result<()> {
    let mut file = File::create("sqrt_binary_lengths.txt")?;
    sqrt::newton_sqrt(15);

    let mut i = 4;
    let mut string = String::new();
    while i < 9223372036854775808 {
        let (x, count1) = sqrt::newton_sqrt(i);
        let mut binary = format!("{i:b}").len();
        let mut binary2 = format!("{x:b}").len();
        if binary2 * 2 != binary {
            string.push_str(&"{i}, {x}, {binary}, {binary2} \n");
        }
        //string.push_str(&format!("{i:b} \n").to_string());
        //string.push_str(&format!("{x:b} \n \n \n").to_string());
        i*=2;
    }
    file.write_all(string.as_bytes())?;

    Ok(())
}