# The Beal Conjecture


For the following equation:

```
A^X + B^Y = C^Z

Where:
A, B, C, X, Y, and Z are positive integers
X, Y, and Z are >=3
```

Beal asserts that A, B, and C will always share a common factor.

Either:
1. Find a counterexample
2. Prove that it's true

Good Luck!

## Notes:

 1. Make a list of all numbers that are powers below uint.max
 2. Find all solutions to the equation given those numbers.
 3. Look for counterexamples or patterns.
 4. In the case we can't reasonably calculate all of the cases,
    figure out the order of magnitude of the problem, and search
    for heuristics to eliminate cases. Also determine if concurrency
    functionally helps us solve the problem.


## TODO
* Add more tests
* Consider limiting PowerNumbers to only non-power bases. (eg. skip 9^x, since it's counted by 3^x)
  * This will eliminate many extra calculations
* If you only use prime PowerNumer bases, any remaining Solutions are worth $1_000_000 :)
  * These aren't the only possible winners though, since non-prime triads can exist without shared factors, even if they individually have multiple factors
* figure out how to make the beal equation show up nicely with LaTeX markdown in the readme