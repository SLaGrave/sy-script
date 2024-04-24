# Syscript

A small, incredibly useless language.

Inspired by [OISCs](https://en.wikipedia.org/wiki/One-instruction_set_computer), especially the subleq (subtract and branch if not equal to zero) command. Built to be a hard to use langauge which has an included Python 3 parser.

## Project structure

This repo holds the two implementations of Syscript:
- `implementations/pysy` - The (abandoned) Python implementation of Syscript
- `implementations/sisy` - The new, Single Interpreted version of Syscript

There is also an `examples/` directory which contains code examples.

There is also the `site/` directory containing the GitHub Pages site for Syscript.

There is also the `langtest/` directory containing tooling to test my implementations.

## Syntax

There are only two commands in syscript: the `sy` operator and the `leaf` operator.

The `sy` operator requires four position arguments. The `sy` operator subtracts the second argument from the first, stores the difference in a variable (the third argument), and branches to a different location in the code (the fourth arguments) if the sum is less than or equal to 0. One, both, or none of the third and fourth arguments can be an underscore, meaning *ignore this argument*.

The `leaf` defines the starting point of a branch that can be reached from the sy command.

Because this language is terrible, comments are contained within chevrons -- `<<Some comment...>>`. Also lines have to end in a semicolon (`;`) (not comment lines though).

## All words / symbols

| Word/Symbol | Usage | Meaning/Use |
| --- | --- | --- |
| sy | sy op1 op2 op3 op4; | Subtract op2 from op1, store difference in op3, jump to op4 |
| leaf | leaf op1; | Define starting point of a branch location named op1 |
| _ | sy op1 op2 _ op4; | If op4 is present, op3 must have something there (vice versa with op4). The underscore means "nothing" |
| stdin | sy stdin op2 op3 op4; | Gets a number from the stdin |
| stdout | sy op1 op2 stout op4; | Prints ascii encoded symbol to stdout |
