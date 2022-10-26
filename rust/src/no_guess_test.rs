use ethers::types::U256;
mod quad_conv_proof;
use std::fs::File;
use std::io::prelude::*;

fn main () {
    let mut file = File::create("no_guess.txt").unwrap();

    let (i,y) = quad_conv_proof::newton_sqrt(U256::MAX, 2.into());
    println!("i: {}, y: {}", i, y.len());
    file.write_all(format!("i: {}, y: {} \n", i, y.len()).as_bytes()).unwrap();
    let mut x = 0;
    let mut errors = Vec::new();
    //let mut error_difs = Vec::new();
    while x < y.len() {
        if i > y[x] {
            errors.push(i-y[x]);
        } else {
            errors.push(y[x]-i);
        }
        println!("{} , error: {}", y[x], errors[x]);
        file.write_all(format!("{} , error: {} \n", y[x], errors[x]).as_bytes()).unwrap();
        x+=1;
    }
}