use std::fs::File;
use std::io::prelude::*;
mod sqrt;

fn main() -> std::io::Result<()> {
    let mut file = File::create("sqrt_binary.txt")?;
    sqrt::newton_sqrt(15);

    let mut i = 4;
    let mut string = String::new();
    while i < 170141183460469231731687303715884105728 {
        let (x, count1) = sqrt::newton_sqrt(i);
        string.push_str(&format!("{i:b} \n").to_string());
        string.push_str(&format!("{x:b} \n \n \n").to_string());
        i*=2;
    }
    file.write_all(string.as_bytes())?;

    Ok(())
}