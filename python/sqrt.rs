//use std::cmp;
//use std::io;

fn main() {
    let mut i = 3;
    while i < u128::MAX {

        println!("{i}");

        println!("ground truth {}", f64::sqrt(i as f64));

        let (x, count1) = newton_sqrt(i);
        println!("newton square root {x}, {count1}");

        let (z, count3) = babylonian_sqrt(i);
        println!("babylonian sqrt {z}, {count3}");

        let (num, num2) = msb_sqrt(i);
        println!("{num}, {num2}");

        i*=2;
    }

    /* let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u128 = input.trim().parse().unwrap();

    println!("ground truth {}", f64::sqrt(number as f64));

    let (x, count1) = newton_sqrt(number);
    println!("newton square root {x}, {count1}");

    let (z, count3) = babylonian_sqrt(number);
    println!("babylonian sqrt {z}, {count3}");

    let (num, num2) = msb_sqrt(number);
    println!("{num}, {num2}"); */
}

pub fn newton_sqrt(n: u128) -> (u128, u128) {
    let mut x = n;
    let mut count = 0;
    loop {
        count+=1;
        let y = (x + n / x) >> 1; // this is x-(x*x-n)/(2*x) simplified
        if x > y {
            if x-y <= 1 {
                x = std::cmp::min(x, n/x);
                return (x, count);
            }
        } else {
            if y-x <= 1 {
                x = std::cmp::min(x, n/x);
                return (x, count);
            }
            
        }
        x = y;
    }
}

pub fn babylonian_sqrt(n: u128) -> (u128, u128) {
    let mut x = n;
    let mut y = 1;
    let mut count = 0;
    loop {
        count+=1;
        x = (x+y)/2; //averages the numbers but causes overflow
        // can fix with x = (x&y) + (x^y) / 2
        y = n/x;
        //wont be necessary if we dont use guessing recursively
        if x > y {
            if x-y <= 1 {
                x = std::cmp::min(x, n/x);
                return (x, count);
            }
        } else {
            if y-x <= 1 {
                x = std::cmp::min(x, n/x);
                return (x, count);
            }
            
        }
    }
/*     x = std::cmp::min(x, n/x);
    return (x, count); */
}

pub fn msb_sqrt(n: u128) -> (u128, u128) {
    let mut result = 1;
    let mut x = n;
    let mut count = 0;

    /* if(x >> 128 > 0) {
        x >>= 128;
        result <<= 64;
    } */

    // x is used to measure something
    // x is thrown away afterwards
    // asking if x is greater than 2**64
    // than shift x by 64 
    // result is shifted by half

    // now assuming that if number is greater than 2^x
    // then the sqrt would be 2^x/2
    // will need to do a bit of testing and research
    // hehe

    // appears to be true to an error of ±1 binary digits

    if x >> 64 > 0 {
        x >>= 64;
        result <<= 32;
    }
    if x >> 32 > 0 {
        x >>= 32;
        result <<= 16;
    }
    if x >> 16 > 0 {
        x >>= 16;
        result <<= 8;
    }
    if x >> 8 > 0 {
        x >>= 8;
        result <<= 4;
    }
    if x >> 4 > 0 {
        x >>= 4;
        result <<= 2;
    }
    if x >> 2 > 0 {
        result <<= 1;
    }

    return (result, x);
}

//now i should just copy the oz guesing system
// and also implement the one i thought of tht also takes advantage of msb
// also read the hackers delight chapter to uinderstand it better