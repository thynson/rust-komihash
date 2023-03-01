
komihash (rust)
==================

Komihash is a hash function passes all [smhasher] tests with extremely 
high performance. A PRNG named `komirand` is also provided that passes
PractRand test. See [the original C implementation] for more details.

Performance
-----------
This crate archives almost the same performance with the original C implementation. 
The benchmark is done on an Apple Macbook Pro (Apple M1 Pro).

```
test bench_simple_with_00000bytes_x10_input ... bench:          17 ns/iter (+/- 0)
test bench_simple_with_00001bytes_x10_input ... bench:          26 ns/iter (+/- 0)
test bench_simple_with_00002bytes_x10_input ... bench:          27 ns/iter (+/- 1)
test bench_simple_with_00003bytes_x10_input ... bench:          28 ns/iter (+/- 1)
test bench_simple_with_00004bytes_x10_input ... bench:          25 ns/iter (+/- 0)
test bench_simple_with_00005bytes_x10_input ... bench:          26 ns/iter (+/- 0)
test bench_simple_with_00006bytes_x10_input ... bench:          28 ns/iter (+/- 1)
test bench_simple_with_00007bytes_x10_input ... bench:          29 ns/iter (+/- 2)
test bench_simple_with_00008bytes_x10_input ... bench:          21 ns/iter (+/- 1)
test bench_simple_with_00009bytes_x10_input ... bench:          27 ns/iter (+/- 1)
test bench_simple_with_00010bytes_x10_input ... bench:          29 ns/iter (+/- 0)
test bench_simple_with_00011bytes_x10_input ... bench:          30 ns/iter (+/- 1)
test bench_simple_with_00012bytes_x10_input ... bench:          27 ns/iter (+/- 1)
test bench_simple_with_00013bytes_x10_input ... bench:          29 ns/iter (+/- 2)
test bench_simple_with_00014bytes_x10_input ... bench:          29 ns/iter (+/- 0)
test bench_simple_with_00015bytes_x10_input ... bench:          29 ns/iter (+/- 1)
test bench_simple_with_00016bytes_x10_input ... bench:          26 ns/iter (+/- 1)
test bench_simple_with_00024bytes_x10_input ... bench:          27 ns/iter (+/- 1)
test bench_simple_with_00032bytes_x10_input ... bench:          33 ns/iter (+/- 1)
test bench_simple_with_00048bytes_x10_input ... bench:          42 ns/iter (+/- 3)
test bench_simple_with_00064bytes_x10_input ... bench:          58 ns/iter (+/- 3)
test bench_simple_with_00096bytes_x10_input ... bench:          70 ns/iter (+/- 4)
test bench_simple_with_00128bytes_x10_input ... bench:          79 ns/iter (+/- 2)
test bench_simple_with_00192bytes_x10_input ... bench:         101 ns/iter (+/- 5)
test bench_simple_with_00256bytes_x10_input ... bench:         124 ns/iter (+/- 7)
test bench_simple_with_01024bytes_x10_input ... bench:         445 ns/iter (+/- 18)
test bench_simple_with_04096bytes_x10_input ... bench:       1,760 ns/iter (+/- 125)
test bench_simple_with_16384bytes_x10_input ... bench:       7,036 ns/iter (+/- 445)
test bench_simple_with_65536bytes_x10_input ... bench:      28,130 ns/iter (+/- 944)

test result: ok. 0 passed; 0 failed; 0 ignored; 29 measured; 0 filtered out; finished in 21.00s

```

or in a more readable format

| Input size (Byte)   | Time (ns) | Throughput(GB/s)   |
|---------------------|-----------|--------------------|
| 0                   | 17        | N/A                |
| 1                   | 26        | 0.04               |
| 2                   | 27        | 0.07               |
| 3                   | 28        | 0.11               |
| 4                   | 25        | 0.16               |
| 5                   | 26        | 0.19               |
| 6                   | 28        | 0.21               |
| 7                   | 29        | 0.24               |
| 8                   | 21        | 0.38               |
| 9                   | 27        | 0.33               |
| 10                  | 29        | 0.34               |
| 11                  | 30        | 0.37               |
| 12                  | 27        | 0.44               |
| 13                  | 29        | 0.45               |
| 14                  | 29        | 0.48               |
| 15                  | 29        | 0.51               |
| 16                  | 26        | 0.61               |
| 24                  | 27        | 0.89               |
| 32                  | 33        | 0.97               |
| 48                  | 42        | 1.14               |
| 64                  | 58        | 1.10               |
| 96                  | 70        | 1.37               |
| 128                 | 79        | 1.62               |
| 192                 | 101       | 1.90               |
| 256                 | 124       | 2.06               |
| 1024                | 445       | 2.29               |
| 4096                | 1760      | 2.33               |
| 16384               | 7036      | 2.34               |
| 65536               | 28130     | 2.33               |


License
-------
MIT

[smhasher]: https://github.com/rurban/smhasher
[the original C implementation]: https://github.com/avaneev/komihash

