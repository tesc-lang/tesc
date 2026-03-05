# TESC
A programming language for writing cli tests.

## Installation
`git clone` -> `cargo install --path tesc-cli`

## Getting started
All tests start with the keyword `test`.
Then there is an argument list that currently requires one argument.
This is the command to run your program.
Finally comes the code you want to run.
The statement would normally be a block, but it could be any statement!
Please note that all statements end in `;`, even blocks.

### Examples
```c
test add("python3 calculator.py") {
    self send "1 + 1"; // Inputs 1 + 1 + '\n' into the program
    self expect "2";   // Expects the program to print 2 + '\n'
};
```
