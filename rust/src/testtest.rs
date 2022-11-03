use ethers::types::U256;

fn main () {
    let n: U256 = 1725203.into();
    let mut x = n.leading_zeros();
    println!("x: {}", x);
    x = (256 - x - 1) / 2+1;
    println!("x: {}", x);
    x = 1<<x;
    println!("x: {}", x);
}