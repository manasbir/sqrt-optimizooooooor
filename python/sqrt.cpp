int newtonSqrt(int n) {
    int x = n;
    while (x * x > n) {
        x = (x + n / x) / 2;
    }
    return x;
}