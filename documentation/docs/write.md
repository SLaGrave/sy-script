# Write Sy

## Overview

### Syntax

There are only two commands in syscript: the `sy` operator and the `leaf` operator.

The `sy` operator requires four position arguments. The `sy` operator subtracts the second argument from the first, stores the difference in a variable (the third argument), and branches to a different location in the code (the fourth arguments) if the sum is less than or equal to 0. One, both, or none of the third and fourth arguments can be an underscore, meaning *ignore this argument*.

The `leaf` defines the starting point of a branch that can be reached from the sy command.

Because this language is terrible, comments are contained within chevrons -- `<<Some comment...>>`. Also lines have to end in a semicolon (`;`) (not comment lines though).

### All words / symbols

| Word/Symbol | Usage | Meaning/Use |
| --- | --- | --- |
| sy | sy op1 op2 op3 op4; | Subtract op2 from op1, store difference in op3, jump to op4 |
| leaf | leaf op1; | Define starting point of a branch location named op1 |
| _ | sy op1 op2 _ op4; | Can be used in place of arg 3 or 4 -- means "nothing" |
| stdin | sy stdin op2 op3 op4; | Gets a number from the stdin |
| stdout | sy op1 op2 stout op4; | Prints ascii encoded symbol to stdout |

## Write

Open a file, write some commands, and then run it using `syscript.py`.

The following script prints "Hello, World!\n" three times. It does so using the following steps:

1. Set variable `counter` to 3
1. Print "Hello, World!\n"
    - This is done by not subtracting anything, and saving it to `stdout`
1. Subtract 1 from `counter`, branching to `End` if less than or equal to 0
1. If not, return to PrintHelloWorld (step 2)

```
leaf ControlVariables;
sy 3 0 counter _;

<<Test comment>>

leaf PrintHelloWorld;
sy 72 0 stdout _;
sy 101 0 stdout _;
sy 108 0 stdout _;
sy 108 0 stdout _;
sy 111 0 stdout _;
sy 44 0 stdout _;
sy 32 0 stdout _;
sy 119 0 stdout _;
sy 111 0 stdout _;
sy 114 0 stdout _;
sy 108 0 stdout _;
sy 100 0 stdout _;
sy 33 0 stdout _;
sy 10 0 stdout _;

leaf CounterChange;
sy counter 1 counter End;
sy 0 0 _ PrintHelloWorld;

leaf End;

```