// quadratical convergence means
// ????

use std::fs::File;
use std::io::prelude::*;
use ethers::types::U256;
mod quad_conv_proof;
mod guessing_methods;


fn main() {
    let mut i: U256 = 9134535344534513451345.into();
    let mut i_string = String::new();
    let mut string2 = String::new();
    let mut file = File::create("i.txt").unwrap();

    while i < 117420511696468706786662444551044609602.into() {
        let (x, count1, iterations1) = quad_conv_proof::newton_sqrt(i, i);
        // ignoring the my super swag method for now
        let (y, count2, iterations2) = quad_conv_proof::newton_sqrt(i, guessing_methods::oz_msb(i));

        i_string.push_str(&format!("{i},").to_string());
        string2.push_str(&format!("{},").to_string());
        string.push_str(&format!("{x:b}, {guess:b}\n\n").to_string());

        i*=3;
        i/=2;
        file.write_all(final_string.as_bytes()).unwrap();
    }
}
