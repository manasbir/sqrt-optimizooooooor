# Returns the square root of n.
# Note that the function
def squareRoot(n):
 
    # We are using n itself as
    # initial approximation This
    # can definitely be improved
        x = n
        y = 1
         
        # e decides the accuracy level
        e = 1
        while(x - y > e):   
            x = (x + y)/2
            y = n / x
            print(x)
     
        return x
 
# Driver program to test
# above function
number = input()
sqrt = squareRoot(int(number))
 
print("Square root of", number, "is", round(sqrt))