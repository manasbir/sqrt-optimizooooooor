use std::fs::File;
use std::io::prelude::*;
mod sqrt;

fn main() {
    let mut file = File::create("msb_test.txt").unwrap();
    sqrt::newton_sqrt(15);

    let mut i = 4;
    let mut string = String::new();
    while i < 9223372036854775808 {
        let (x, count1) = sqrt::newton_sqrt(i);
        let (result, y) = sqrt::msb_sqrt2(i);
        let (result2, z) = sqrt::msb_sqrt(i);
        
        string.push_str(&format!("{i:b} \n").to_string());
        string.push_str(&format!("{x:b} \n").to_string());
        string.push_str(&format!("{result2:b} \n").to_string());
        string.push_str(&format!("{result:b} \n \n").to_string());
        i*=2;
    }
    file.write_all(string.as_bytes()).unwrap();

}

