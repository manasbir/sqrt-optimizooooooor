// not actuall main.rs
// but msb_fulltest.rs


//use std::fs::File;
//use std::io::prelude::*;
mod guessing_methods;
mod sqrt_guess;
use ethers::types::U256;
//mod helpers;

fn main() {
    //let mut file = File::create("guess_test.txt").unwrap();
    let mut i: U256 = U256::MAX;
    //let mut string = String::new();
    let mut alt_guess_fastest_count: u128 = 0;
    let mut oz_guess_fastest_count: u128 = 0;
    let mut tie: u128 = 0;

    while i > 4.into() {
        let fastest_method;
        let oz_guess = guessing_methods::oz_msb(i);
        let alt_guess = guessing_methods::alt_msb(i);

        let oz = sqrt_guess::newton_sqrt(i, oz_guess);
        let alt = sqrt_guess::newton_sqrt(i, alt_guess);


        if oz < alt {
            oz_guess_fastest_count += 1;
            fastest_method = oz;
        } else if alt < oz {
            alt_guess_fastest_count += 1;
            fastest_method = alt;
        } else {
            tie += 1;
            fastest_method = oz;
        }

        if fastest_method >= 7 {
            println!("oz: {oz}, alt {alt}");
        }
        
        // create a string and constrantly push to it
        // and write that string to the file
        // each iteration




        i-=7.into(); // rand number
        //println!("{fastest_method}");
        if i % 9999999 == 0.into() {
            println!("{fastest_method}salt");
            println!("i: {i}, oz: {oz_guess_fastest_count}, alt: {alt_guess_fastest_count}, ties: {tie}");
        }
        // i: 670601741179819176, oz: 743756524, alt: 2140170377, ties: 608109863
        // making progress
        // need to see if this is actually helpful for limiting iterations of newton_sqrt
        // so for uint256, it would only be better if it limited iterations to 6
        
        // also need to test if recursion or set amounts is faster
        // this can only be done in huff as the gas savings are very minor
        // im guessing it costs for gas to do recursively
    }

}