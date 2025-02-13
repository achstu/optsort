# About

The optsort crate provides `optsort!` macro which expands to if-else decision tree which sort given sequence. It is guaranteed that depth of this tree (maximum numer of comparision between elements) will not exceed $\left \lceil{\log_2(n!)}\right \rceil$ where $n$ is lenght of the sequence.

### Example

For $n = 5$ it is sufficient to use $7$ comparisons.