pub fn is_closer_to (x: u128, y: u128, n: u128) -> bool {
    // doesn't include the case where x and y are equal
    // only one scenario where this happened so far
    // ill add a equals option tomorrow pogU
    // but odds of that are low
    let x_diff;
    let y_diff;
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