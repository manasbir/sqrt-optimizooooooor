//use std::cmp;

fn main(){
    println!("{}", sqrt(2.0));

    let (x, count1) = newton_sqrt(123123);
    println!("{x}, {count1}");

    let (y, count2) = other_sqrt(123123);
    println!("{y}, {count2}");

    let (z, count3) = babylonian_sqrt(123123);
    println!("{z}, {count3}");
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

fn other_sqrt(n: u128) -> (u128, u128) {
    let mut x = n;
    let mut count = 0;
    loop {
        count+=1;
        let y = (x + n / x) / 2;
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