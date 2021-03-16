# Styling your Sy script

LMAO you wanna style this garbage?

Okay...

- File should have the sy type -- `test.sy`
- If one of the first two arguments to a `sy` command is 0, it should be the second one
    - Good: `sy 12 0 _ _;`
    - Bad: `sy 0 12 _ _;`
- Variables should use lowerCamelCase naming schemes
- Branch locations/leafs should use UpperCamelCase naming schemes
- Printing a string should be its own leaf
- One newline between the end of a leaf and the `leaf` command of the next one.