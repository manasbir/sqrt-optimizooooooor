pub fn newton_sqrt (n: u128, guess: u128) -> u128 {
    let mut x = guess;
    let mut count = 0;
    loop {
        count+=1;
        let y = (x + n / x) >> 1; // this is x-(x*x-n)/(2*x) simplified
        if x > y {
            if x-y <= 1 {
                //x = std::cmp::min(x, n/x);
                return count;
            }
        } else {
            if y-x <= 1 {
                //x = std::cmp::min(x, n/x);
                return count;
            }
            
        }
        x = y;
    }
}