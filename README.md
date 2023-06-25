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
test bench_simple_with_00000bytes_x10_input ... bench:          16 ns/iter (+/- 0)
test bench_simple_with_00001bytes_x10_input ... bench:          25 ns/iter (+/- 0)
test bench_simple_with_00002bytes_x10_input ... bench:          24 ns/iter (+/- 1)
test bench_simple_with_00003bytes_x10_input ... bench:          25 ns/iter (+/- 0)
test bench_simple_with_00004bytes_x10_input ... bench:          23 ns/iter (+/- 0)
test bench_simple_with_00005bytes_x10_input ... bench:          24 ns/iter (+/- 0)
test bench_simple_with_00006bytes_x10_input ... bench:          25 ns/iter (+/- 0)
test bench_simple_with_00007bytes_x10_input ... bench:          27 ns/iter (+/- 0)
test bench_simple_with_00008bytes_x10_input ... bench:          22 ns/iter (+/- 0)
test bench_simple_with_00009bytes_x10_input ... bench:          25 ns/iter (+/- 1)
test bench_simple_with_00010bytes_x10_input ... bench:          26 ns/iter (+/- 0)
test bench_simple_with_00011bytes_x10_input ... bench:          28 ns/iter (+/- 0)
test bench_simple_with_00012bytes_x10_input ... bench:          25 ns/iter (+/- 1)
test bench_simple_with_00013bytes_x10_input ... bench:          26 ns/iter (+/- 0)
test bench_simple_with_00014bytes_x10_input ... bench:          26 ns/iter (+/- 0)
test bench_simple_with_00015bytes_x10_input ... bench:          26 ns/iter (+/- 1)
test bench_simple_with_00016bytes_x10_input ... bench:          25 ns/iter (+/- 0)
test bench_simple_with_00024bytes_x10_input ... bench:          26 ns/iter (+/- 0)
test bench_simple_with_00032bytes_x10_input ... bench:          32 ns/iter (+/- 0)
test bench_simple_with_00048bytes_x10_input ... bench:          39 ns/iter (+/- 1)
test bench_simple_with_00064bytes_x10_input ... bench:          56 ns/iter (+/- 0)
test bench_simple_with_00096bytes_x10_input ... bench:          68 ns/iter (+/- 0)
test bench_simple_with_00128bytes_x10_input ... bench:          76 ns/iter (+/- 3)
test bench_simple_with_00192bytes_x10_input ... bench:          98 ns/iter (+/- 2)
test bench_simple_with_00256bytes_x10_input ... bench:         120 ns/iter (+/- 3)
test bench_simple_with_01024bytes_x10_input ... bench:         433 ns/iter (+/- 8)
test bench_simple_with_04096bytes_x10_input ... bench:       1,734 ns/iter (+/- 54)
test bench_simple_with_16384bytes_x10_input ... bench:       6,955 ns/iter (+/- 96)
test bench_simple_with_65536bytes_x10_input ... bench:      27,840 ns/iter (+/- 601)


test result: ok. 0 passed; 0 failed; 0 ignored; 29 measured; 0 filtered out; finished in 21.00s

```

or in a more readable format

| Input size (Byte) | Time (ns) | Throughput(GB/s) |
|-------------------|-----------|------------------|
| 0                 | 1.6       | N/A              |
| 1                 | 2.5       | 0.4              |
| 2                 | 2.4       | 0.83             |
| 3                 | 2.5       | 1.2              |
| 4                 | 2.3       | 1.73             |
| 5                 | 2.4       | 2.08             |
| 6                 | 2.5       | 2.4              |
| 7                 | 2.7       | 2.59             |
| 8                 | 2.2       | 3.64             |
| 9                 | 2.5       | 3.6              |
| 10                | 2.6       | 3.85             |
| 11                | 2.8       | 3.93             |
| 12                | 2.5       | 4.8              |
| 13                | 2.6       | 5.0              |
| 14                | 2.6       | 5.38             |
| 15                | 2.6       | 5.77             |
| 16                | 2.5       | 6.4              |
| 24                | 2.6       | 9.23             |
| 32                | 3.2       | 10               |
| 48                | 3.9       | 12.3             |
| 64                | 5.6       | 11.4             |
| 96                | 6.8       | 14.1             |
| 128               | 7.6       | 16.8             |
| 192               | 9.8       | 19.6             |
| 256               | 12        | 21.3             |
| 1024              | 43.3      | 23.6             |
| 4096              | 173.4     | 23.6             |
| 16384             | 695.5     | 23.6             |
| 65536             | 2784.0    | 23.6             |

License
-------
MIT

[smhasher]: https://github.com/rurban/smhasher

[the original C implementation]: https://github.com/avaneev/komihash

