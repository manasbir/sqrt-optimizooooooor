# optimizooor
# why is half the repo makefile all of a sudden????
# ill update this later lol, so much to write
## ill publish an article if it gets interesting or if anyone cares

# new notes !!!!!!

there are no new notes >:(

- figure out rust bs

- figure out if babylonian or newton cost less gas

##
the eternal pursuit to get the most efficient square root function 

things will be written in python, then graphed, then written in solidity, then huff and yul (yul idk why, but its my repo lol)

no graphs

figure out gas efficiency just cuz, alot of things im doing just cuz lmao

need to rescratch kek

# notes
square root estimation is generally about making a good guess, 
due to the fact that both methods i use are 'alledgedly' quadratically convergent 
meaning that they double in accuracy everytime.

the methods require a guess to start if that wasnt already implied.

my math skills are generally lacking when it comes to reading the equations on wikipedia
so iyw plz add more methods and help optimize the ones already written :)))))

the two main methods (newton and babylonian method) for squareroots
generally work the same way, with the same convergence rate. (will test to confirm)

babylonian method has a tendancy to never end which is odd
this is due to the fact that the diff between x and y can be stuck at 1 which doesnt satisfy the loop condition

with the babylonian method, you have to balance 2 items rather than 1 like the newton method

they are the same kek, but only for when you are looking for sqrts

https://en.wikipedia.org/wiki/Methods_of_computing_square_roots

alot of the work is to do with the guesses, and we will steal that off
hackers delight/oz math.sol for initial testing

in solidity the decimals dont matter, so checks are easy
i dont think yul math saves gas lol

oz doesnt use checks but instead relies on the fact that the guesses made are good
and perhaps more efficient than recursion, i thinks thats due to the checks not being worth it computationally

a way to counteract this is to only check after a certain amount of guesses, but that is also hard to do


we could also check for when it reaches a dif of one, but that's quite a bit of computation

# todo
-figure out how oz makes its guesses

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


-~~also learn why my method/implementation of the babylonian method overflows lmao~~

-why i cross out bruh