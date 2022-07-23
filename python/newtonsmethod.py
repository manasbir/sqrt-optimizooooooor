import math 
import os

if os.path.exists("babyloniansqrt.txt"):
    os.remove("babyloniansqrt.txt")

if os.path.exists("newtonsqrt.txt"):
    os.remove("newtonsqrt.txt")

babylonian_file = open("babyloniansqrt.txt", "w")
newton_file = open("newtonsqrt.txt", "w")


def newtonSqrt (num):
    error = 1
    guess = num
    count = 0
    diff = 9999999
    while abs(diff) > error:
        newton_file.write(str(guess) + "\n")
        newGuess = guess - ((guess**2 - num) / (2 * guess))        

        diff = (newGuess - guess)
        guess = newGuess
        count+=1

    
    return guess, count

def babylonianMethod(num):
    error = 1
    guess = num
    count = 0
    newGuess = 1
    while abs(newGuess - guess) > error:
        babylonian_file.write(str(guess) + "\n")
        guess = (guess + newGuess)/2
        newGuess = num / guess
        count+=1
    return guess, count

def sqrt(x):
    babylonianMethod(x)
    newtonSqrt(x)

number = int(input("sqrt: "))
sqrt(number)