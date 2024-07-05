# Ownership
Set of rules that govern memory management
The rules are enforced at compile time

Incase any of the rules are violated the program fails to compile.

## Ownership Rules
Every value has an owner
Their can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.

An owner of a value is the variable or data structure that holds it and is resposible
for allocation and freeing the memory used to store that data.

time : -> 1: 52:36
