//use std::fs::File;
//use std::io::prelude::*;
mod guessing_methods;
mod sqrt_guess;
//mod helpers;

fn main() {
    //let mut file = File::create("guess_test.txt").unwrap();
    let mut i = 4;
    //let mut string = String::new();
    let mut alt_guess_fastest_count = 0;
    let mut oz_guess_fastest_count = 0;
    let mut tie = 0;

    while i < u128::MAX {
        let fastest_method;
        let mut oz = guessing_methods::oz_msb(i);
        let mut alt = guessing_methods::alt_msb(i);

        oz = sqrt_guess::newton_sqrt(i, oz);
        alt = sqrt_guess::newton_sqrt(i, alt);


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



        i+=192037423; // rand number
        //println!("{fastest_method}");
        if i % 9999999 == 0 {
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