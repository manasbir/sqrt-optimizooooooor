use std::fs::File;
use std::io::prelude::*;
mod helpers;
mod sqrt;

fn main() {
    let mut file = File::create("testing_dif_msb.txt").unwrap();
    sqrt::newton_sqrt(15);

    let mut i = 4;
    let mut string = String::new();
    while i < 117420511696468706786662444551044609602 {
        let (x, count1) = sqrt::newton_sqrt(i);
        let (guess, y) = sqrt::msb_sqrt2(i);
        let (guess2, z) = sqrt::msb_sqrt(i);

        // need a way to differentiate between which is better
        // bad idea
        if helpers::is_closer_to(guess, guess2, x) {
            string.push_str(&format!("{i}, {x}, {guess}, {guess2}\n").to_string());
            string.push_str(&format!("{i:b}, {guess:b}\n").to_string());
            string.push_str(&format!("{x:b}, {guess2:b}\n\n").to_string());
        } else {
            string.push_str(&format!("{i}, {x}, {guess2}, {guess} \n").to_string());
            string.push_str(&format!("{i:b}, {guess2:b}\n").to_string());
            string.push_str(&format!("{x:b}, {guess:b}\n\n").to_string());
        }

        //string.push_str(&format!("{i:b} \n").to_string());
        //string.push_str(&format!("{x:b} \n \n \n").to_string());
        i*=3;
        i/=2;
        
    }
    file.write_all(string.as_bytes()).unwrap();

}