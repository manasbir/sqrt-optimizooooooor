mod quad_conv_proof;
use std::fs::File;
use std::io::prelude::*;

fn main () {
    let mut file = File::create("determinism.txt").unwrap();
    let n = 98273483741;
    let (root, y) = quad_conv_proof::newton_sqrt_u128(n, 98273483741);
    println!("i: {}, y: {}", n, y.len());

    let mut i = 0;
    let mut errors = Vec::new();
    let mut error_difs: Vec<u128> = Vec::new();

    while i < y.len() {
        if y[i] > root {
            errors.push(y[i]-root);
        } else {
            errors.push(root-y[i]);
        }
        if i > 0 {
            if y[i] > y[i-1] {
                error_difs.push(y[i]-y[i-1]);
            } else {
                error_difs.push(y[i-1]-y[i]);
            }
        } else {
            error_difs.push(0);
        }
        
        file.write_all(format!("guess: {} error_1: {} error_2: {} \n", y[i], errors[i], error_difs[i]).as_bytes()).unwrap();
        file.write_all(format!("guess: {:b} error_1: {:b} error_2: {:b} \n", y[i], errors[i], error_difs[i]).as_bytes()).unwrap();
        file.write_all(format!("mul_error_1: {} mul_error_2: {} \n\n", errors[i]/root, error_difs[i]/root).as_bytes()).unwrap();
        println!("guess: {} error_1: {} error_2: {}", y[i], errors[i], error_difs[i]);
        i+=1;
    }
}