
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

| Input size (Byte) | Time (ns) | Throughput(GB/s) |
|-------------------|-----------|------------------|
| 0                 | 1.7       | N/A              |
| 1                 | 2.6       | 0.38             |
| 2                 | 2.7       | 0.74             |
| 3                 | 2.8       | 1.07             |
| 4                 | 2.5       | 1.6              |
| 5                 | 2.6       | 1.92             |
| 6                 | 2.8       | 2.14             |
| 7                 | 2.9       | 2.41             |
| 8                 | 2.1       | 3.81             |
| 9                 | 2.7       | 3.33             |
| 10                | 2.9       | 3.45             |
| 11                | 3.0       | 3.67             |
| 12                | 2.7       | 4.44             |
| 13                | 2.9       | 4.48             |
| 14                | 2.9       | 4.83             |
| 15                | 2.9       | 5.17             |
| 16                | 2.6       | 6.15             |
| 24                | 2.7       | 8.89             |
| 32                | 3.3       | 9.70             |
| 48                | 4.2       | 11.43            |
| 64                | 5.8       | 11.03            |
| 96                | 7.0       | 13.71            |
| 128               | 7.9       | 16.20            |
| 192               | 10.1      | 19.01            |
| 256               | 12.4      | 20.64            |
| 1024              | 44.5      | 23.01            |
| 4096              | 176.0     | 23.27            |
| 16384             | 703.6     | 23.29            |
| 65536             | 2813.0    | 23.29            |




License
-------
MIT

[smhasher]: https://github.com/rurban/smhasher
[the original C implementation]: https://github.com/avaneev/komihash

