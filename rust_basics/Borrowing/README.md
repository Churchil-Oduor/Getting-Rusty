# Borrowing
A way of temporarily accessing data without taking ownership of it.
When borrowing, you're taking reference(pointer) to the data and not the data itself.

This prevents dangling pointers and data races.
Data can be borrowed mutably or immutably.

## Borrowing Rules
At any given time:
number of mutable refences is one but immutable references can be any number.

References must always be valid.
