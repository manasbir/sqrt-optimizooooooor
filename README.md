# optimizooor
 me wanting to learn yul (cringe name lmao)

# notes
square root estimation is generally about making a good guess, 
my math skills are generally lacking when it comes to reading the equations on wikipedia



the two main methods (newton and babylonian method) for squareroots
generally work the same way, with the same convergence rate. (will test to confirm)

https://en.wikipedia.org/wiki/Methods_of_computing_square_roots

alot of the work is to do with the guesses, and we will steal that off
hackers delight/oz math.sol

in solidity the decimals dont matter, so checks are easy

oz doesnt use checks but instead relies on the fact that the guesses made are good
and actually more efficient than recursion

# todo
-make the oz guessing system in python (will do js or c++ if too hard)

-can use guess w/o decimal into consideration, add something that will delete the float point part in the guess

-find a way to graph txt files with a bunch of numbers (google spreadsheets should be good)

-compare guesses vs recursion

-once we find with has better convergence, compare guessing with no guessing

-should have for graphs if we dont care about decimals

    1. newton no guess
    2. newton with guess
    3. babylonian no guess
    4. babylonian with guess


# solidity todo:
-make a solidity version of the best in yul, solidity, and huff

-?????

-profit


-also learn why my method/implementation of the babylonian method overflows lmao