# Syscript

A small, incredibly useless language.

Inspired by [OISCs](https://en.wikipedia.org/wiki/One-instruction_set_computer), especially the subleq (subtract and branch if not equal to zero) command. Built to be a hard to use langauge which has an included Python 3 parser.

Check out the [github pages site](https://slagrave.github.io/syscript/) for more info.

---

## Project structure

This repo holds the two implementations of Syscript:
- `implementations/sypython` - The (abandoned) Python implementation of Syscript
- `implementations/sisy` - The new, Single Interpreted version of Syscript

**Note: The sisy implementation is only present on the `implementation/rust` branch.**

There is also an `examples/` directory which contains code examples.

There is also the `site/` directory containing the GitHub Pages site for Syscript.

There is also the `langtest/` directory containing tooling to test my implementations.

## Running `langtest`s

Run the language test suite by calling the `langtest/langtest` script and pass the Syscript interpreter command you want to test.

For example, on MacOS, I use the following command:

```sh
./langtest/langtest -c './sisy'
```