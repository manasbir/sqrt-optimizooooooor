pub fn oz_msb(n: u128) -> u128 {
    let mut result = 1;
    let mut x = n;

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

    return result;
}

pub fn alt_msb(n: u128) -> u128 {
    let mut result = 3;
    let mut x = n;

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

    result >>= 1;

    return result;

}

/* pub fn div2(n: u128) -> u128 {
    return n >> 1;
} */