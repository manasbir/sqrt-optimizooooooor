use std::fs::File;
use std::io::prelude::*;
mod helpers;
mod sqrt;

fn main() {
    let mut file = File::create("testing_dif_msb.txt").unwrap();

    let mut i = 4;

    let mut string = String::new();

    let mut my_guess_closest_count = 0;
    let mut oz_guess_closest_count = 0;
    
    while i < 117420511696468706786662444551044609602 {
        let (x, count1) = sqrt::newton_sqrt(i);
        let (guess, y) = sqrt::msb_sqrt2(i);
        let (oz_guess, z) = sqrt::msb_sqrt(i);

        // need a way to differentiate between which is better
        // bad idea
        if helpers::is_closer_to(guess, oz_guess, x) {
            string.push_str(&format!("{i}, {x}, {guess}, {oz_guess}\n").to_string());
            string.push_str(&format!("{i:b}, {guess:b}\n").to_string());
            string.push_str(&format!("{x:b}, {oz_guess:b}\n\n").to_string());
            my_guess_closest_count += 1;
        } else {
            string.push_str(&format!("{i}, {x}, {oz_guess}, {guess} \n").to_string());
            string.push_str(&format!("{i:b}, {oz_guess:b}\n").to_string());
            string.push_str(&format!("{x:b}, {guess:b}\n\n").to_string());
            oz_guess_closest_count += 1;
        }


        i*=3;
        i/=2;
        
    }
    let mut final_string = String::new();
    final_string.push_str(&format!("my_guess_closest_count: {my_guess_closest_count} \n").to_string());
    final_string.push_str(&format!("oz_guess_closest_count: {oz_guess_closest_count} \n\n\n").to_string());
    final_string.push_str(&format!("{string}").to_string());
    file.write_all(final_string.as_bytes()).unwrap();

}