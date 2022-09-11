use ethers::types::U256;

pub fn oz_msb(n: U256) -> U256 {
    let mut result: U256 = 1.into();
    let mut x = n;
    
    if x >> 128 > 0.into() {
        x >>= 128;
        result <<= 64;
    } 
    if x >> 64 > 0.into() {
        x >>= 64;
        result <<= 32;
    }
    if x >> 32 > 0.into() {
        x >>= 32;
        result <<= 16;
    }
    if x >> 16 > 0.into() {
        x >>= 16;
        result <<= 8;
    }
    if x >> 8 > 0.into() {
        x >>= 8;
        result <<= 4;
    }
    if x >> 4 > 0.into() {
        x >>= 4;
        result <<= 2;
    }
    if x >> 2 > 0.into() {
        result <<= 1;
    }

    return result;
}

pub fn alt_msb(n: U256) -> U256 {
    let mut result: U256 = 3.into();
    let mut x = n;

    if x >> 128 > 0.into() {
        x >>= 128;
        result <<= 64;
    } 
    if x >> 64 > 0.into() {
        x >>= 64;
        result <<= 32;
    }
    if x >> 32 > 0.into() {
        x >>= 32;
        result <<= 16;
    }
    if x >> 16 > 0.into() {
        x >>= 16;
        result <<= 8;
    }
    if x >> 8 > 0.into() {
        x >>= 8;
        result <<= 4;
    }
    if x >> 4 > 0.into() {
        x >>= 4;
        result <<= 2;
    }
    if x >> 2 > 0.into() {
        result <<= 1;
    }

    result >>= 1;

    return result;

}

/* pub fn div2(n: U256) -> U256 {
    return n >> 1;
} */