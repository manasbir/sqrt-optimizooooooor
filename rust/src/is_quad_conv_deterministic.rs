mod quad_conv_proof;
use std::fs::File;
use std::io::prelude::*;

fn main () {
    let mut file = File::create("determinism_v4.txt").unwrap();
    let n = 97976567546533578534;
    let (root, y) = quad_conv_proof::newton_sqrt_u128(n, n);
    println!("i: {}, y: {}", n, y.len());
    file.write_all (format!("i: {}, y: {} \n", n, y.len()).as_bytes()).unwrap();

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
        
        let mut me: f64 = (errors[i] as f64 / root as f64 ) as f64;
        let mut me2: f64 = (error_difs[i] as f64 / root as f64) as f64;

        file.write_all(format!("guess: {} error_1: {} error_2: {} \n", y[i], errors[i], error_difs[i]).as_bytes()).unwrap();
        file.write_all(format!("mul_error_1: {} mul_error_2: {} \n\n", me, me2).as_bytes()).unwrap();
        println!("guess: {} error_1: {} error_2: {}", y[i], errors[i], error_difs[i]);
        i+=1;
    }
}