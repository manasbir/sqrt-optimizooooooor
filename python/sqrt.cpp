#include <stdio.h>
#include <math.h> 
#include <iostream>
#include <fstream>
#include <string>


unsigned long long int newton_sqrt(unsigned long long int n) {
    unsigned long long int guess = n;
    unsigned long long int diff = 1;

    while (true) {
        unsigned long long int new_guess = ((guess * guess + n) / (2 * guess));

        if (new_guess == guess) {
            break;
        }
        guess = new_guess;

    }
    return std::min(guess, n/guess);
}

unsigned long long int other_sqrt(unsigned long long int n) {
    unsigned long long int last_guess = n;
    unsigned long long int guess;
    while (true) {
        guess = (last_guess + n / last_guess) / 2;
        if (guess == last_guess) {
            return guess;
        }
        last_guess = guess;
    }
}

unsigned long long int babylonian_sqrt(unsigned long long int n) {
    unsigned long long int guess = n;
    //unsigned long long int i = 1;
    unsigned long long int y = 1;
    while (guess - y > 0) {
        guess = (guess + y) / 2;
        y = n / guess;
    }
    return guess;
}

int main() {
    unsigned long long int n;
    //std::ofstream newtonsqrt ("newtonsqrt.txt");
    std::cout << "number: ";
    std::cin >> n;

    std::cout << "ground truth: " << sqrt(n) << std::endl;
    std::cout << "newton: " << newton_sqrt(n) << std::endl;
    std::cout << "other: " << other_sqrt(n) << std::endl;
    std::cout << "babylonian: " << babylonian_sqrt(n) << std::endl;
    return 0;
}