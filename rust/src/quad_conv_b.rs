use ethers::types::U256;
mod quad_conv_proof;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("no_guess_b.txt").unwrap();
    let n = U256::MAX;
    let (root , guesses) = quad_conv_proof::newton_sqrt(n, 2.into());
    println!("root: {}, num of guesses: {}", root, guesses.len());
    file.write_all(format!("correct num: {}, num of guesses: {} \n", root , guesses.len()).as_bytes()).unwrap();
    file.write_all(format!("correct root: {:b}, num: {:b} \n", root.as_ref()[0] , n.as_ref()[0]).as_bytes()).unwrap();
    let mut i = 0;
    let mut errors = Vec::new();
    //let mut error_difs = Vec::new();
    while i < guesses.len() {
        if root > guesses[i] {
            errors.push(root-guesses[i]);
        } else {
            errors.push(guesses[i]-root);
        }
        println!("guess: {} , error: {}", guesses[i], errors[i]);
        let guess = guesses[i].as_ref();
        file.write_all(format!("guess: {:b} , error: {}} \n", guess, errors[i]).as_bytes()).unwrap();
        i+=1;
    }
}
