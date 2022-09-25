use ethers::types::U256;

fn newton_sqrt(U256: n, U256: guess) -> (U256, U256, Vec) {
    let mut x = guess;
    let mut count = 0;
    let mut iterations = Vec::new();
    loop {
        count+=1;
        let y = (x + n / x) >> 1; // this is x-(x*x-n)/(2*x) simplified
        if x > y {
            if x-y <= 1.into() {
                x = std::cmp::min(x, n/x);
                return (x, count, iterations);
            }
        } else {
            if y-x <= 1.into() {
                x = std::cmp::min(x, n/x);
                return (x, count, iterations);
            }
        }
        iterations.push(y);
        x = y;
    }
}