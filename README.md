# Row vs column major bench for Nushell

Demonstrates row and column major tables with an API abstraction that makes them feel basically the same.

Benchmark results (Linux, AMD Threadripper 2950X)

```
test col_major_from_value        ... bench:   4,105,603.65 ns/iter (+/- 264,171.82)
test col_major_get_row_as_record ... bench:         198.21 ns/iter (+/- 8.98)
test col_major_insert_column     ... bench:     877,263.64 ns/iter (+/- 105,120.73)
test col_major_into_value        ... bench:   2,807,941.65 ns/iter (+/- 290,107.44)
test col_major_sum_column        ... bench:     155,628.20 ns/iter (+/- 3,822.05)
test row_major_from_value        ... bench:   4,427,417.70 ns/iter (+/- 226,991.99)
test row_major_get_row_as_record ... bench:         164.61 ns/iter (+/- 57.20)
test row_major_insert_column     ... bench:   1,376,070.43 ns/iter (+/- 500,622.32)
test row_major_into_value        ... bench:   2,834,972.55 ns/iter (+/- 91,208.00)
test row_major_sum_column        ... bench:     272,910.07 ns/iter (+/- 3,795.36)
```
