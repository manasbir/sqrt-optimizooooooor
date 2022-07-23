def sqrt(x):
    last_guess= x/2.0
    while True:
        guess= (last_guess + x/last_guess)/2
        if abs(guess - last_guess) < .000001: # example threshold
            return guess

        last_guess= guess

x = input()
print('the sqrt of', x, ' is:')