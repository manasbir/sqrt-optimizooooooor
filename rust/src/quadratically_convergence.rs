// quadratical convergence means
// ????

use std::fs::File;
use std::io::prelude::*;
use ethers::types::U256;
mod quad_conv_proof;
mod guessing_methods;


fn main() {
    let mut i: U256 = U256::MAX>>128;
    let mut string = String::new();
    //let mut string2 = String::new();
    let mut file = File::create("quad_conv_wo_guess.csv").unwrap();
    string.push_str(&format!("i, count, iterations, iterations_in_binary \n"));

    while i < U256::MAX {
        let (x, iterations1) = quad_conv_proof::newton_sqrt(i, i);
        // ignoring the my super swag method for now
        //let (y, iterations2) = quad_conv_proof::newton_sqrt(i, guessing_methods::oz_msb(i));

        string.push_str(&format!("{}, {}, \n", i, iterations1.len()).to_string());

        let iteratiosn = iterations1.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
        let iterations_in_binary = iterations1.into_iter().map(|x| {format!("{x:b}")}).collect::<Vec<String>>().join(", ");
        // copilot is godsend

        // todo:
        // compare x to iterations and compare that to dif iterations
        // do the same in binary
        // do the same but with correct # of digits rather than numerically
        // do the same in binary

        string.push_str(&format!("{iteratiosn}\n").to_string());
        string.push_str(&format!("{iterations_in_binary}\n\n").to_string());

        i/=2;
        i*=3;

        //fix this lol
        //file.write_all(string.as_bytes()).unwrap();
    }
}
