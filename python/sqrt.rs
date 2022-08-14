//use std::cmp;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: u128 = input.trim().parse().unwrap();

    println!("ground truth {}", f64::sqrt(number as f64));

    let (x, count1) = newton_sqrt(number);
    println!("newton square root {x}, {count1}");

    let (z, count3) = babylonian_sqrt(number);
    println!("babylonian sqrt {z}, {count3}");
}

fn newton_sqrt(n: u128) -> (u128, u128) {
    let mut x = n;
    let mut count = 0;
    loop {
        count+=1;
        let y = (x + n / x) / 2; // this is x-(x*x-n)/(2*x) simplified
        if x == y {
            x = std::cmp::min(y, n/y);
            return (x, count);
        }
        x = y;
    }
}

fn babylonian_sqrt(n: u128) -> (u128, u128) {
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

//now i should just copy the oz guesing system
// and also implement the one i thought of tht also takes advantage of msb
// also read the hackers delight chapter to uinderstand it better