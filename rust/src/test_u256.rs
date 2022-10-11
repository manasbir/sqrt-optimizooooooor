use ethers::types::U256;


struct MyU256(U256);
impl std::fmt::Binary for MyU256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


fn main() {
    dbg!(format!("{:b}", MyU256(U256::MAX)));
}