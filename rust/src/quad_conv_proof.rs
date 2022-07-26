use ethers::types::U256;

pub fn newton_sqrt_u128(n: u128, guess: u128) -> (u128, Vec<u128>) {
    let mut x = guess;
    let mut iterations = vec![guess];
    loop {
        let y = (x + n / x) >> 1; // this is x-(x*x-n)/(2*x) simplified
        if x > y {
            if x-y <= 1 {
                x = std::cmp::min(x, n / x);
                return (x, iterations);
            }
        } else {
            if y-x <= 1 {
                x = std::cmp::min(x, n / x);
                return (x, iterations);
            }
        }
        iterations.push(y);
        x = y;
    }
}

pub fn newton_sqrt(n: U256, guess: U256) -> (U256, Vec<U256>) {
    let mut x = guess;
    let mut iterations = vec![n];
    loop {
        let y = (x + n / x) >> 1; // this is x-(x*x-n)/(2*x) simplified
        if x > y {
            if x-y <= 1.into() {
                x = std::cmp::min(x, n / x);
                return (x, iterations);
            }
        } else {
            if y-x <= 1.into() {
                x = std::cmp::min(x, n / x);
                return (x, iterations);
            }
        }
        iterations.push(y);
        x = y;
    }
}