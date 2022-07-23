import math 
import os

with open("words.txt","r") as words:
    string_of_words = words.read()

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
        guess = (guess + newGuess)/2
        newGuess = num / guess
        count+=1
    return guess, count

def sqrt(num):
    return math.sqrt(num), babylonianMethod(num), approxSqrt(num)