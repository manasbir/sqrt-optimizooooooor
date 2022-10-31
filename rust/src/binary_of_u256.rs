use ethers::types::U256;
mod quad_conv_proof;

fn main() {
    let x = U256::from_dec_str("213123123123").unwrap();
    println!("{}", format!("{:b}", x.as_ref()[0]));
}