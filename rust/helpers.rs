pub fn is_closer_to (x: u128, y: u128, n: u128) -> bool {
    let mut x_diff;
    let mut y_diff;
    if x > n {
        x_diff = x-n;
    } else {
        x_diff = n-x;
    }
    if y > n {
        y_diff = y-n;
    } else {
        y_diff = n-y;
    }
    if x_diff < y_diff {
        return true;
    } else {
        return false;
    }
}