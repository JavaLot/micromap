# The Fastest Linear Map in Rust

[![cargo](https://github.com/yegor256/micromap/actions/workflows/cargo.yml/badge.svg)](https://github.com/yegor256/micromap/actions/workflows/cargo.yml)
[![crates.io](https://img.shields.io/crates/v/micromap.svg)](https://crates.io/crates/micromap)
[![codecov](https://codecov.io/gh/yegor256/micromap/branch/master/graph/badge.svg)](https://codecov.io/gh/yegor256/micromap)
[![Hits-of-Code](https://hitsofcode.com/github/yegor256/micromap)](https://hitsofcode.com/view/github/yegor256/micromap)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/yegor256/micromap/blob/master/LICENSE.txt)
[![docs.rs](https://img.shields.io/docsrs/micromap)](https://docs.rs/micromap/latest/micromap/)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fyegor256%2Fmicromap.svg?type=shield&issueType=license)](https://app.fossa.com/projects/git%2Bgithub.com%2Fyegor256%2Fmicromap?ref=badge_shield&issueType=license)

A much faster alternative of
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
for very small maps.
It is also faster than
[FxHashMap](https://github.com/rust-lang/rustc-hash),
[hashbrown](https://github.com/rust-lang/hashbrown),
[ArrayMap](https://github.com/robjtede/tinymap),
[IndexMap](https://crates.io/crates/indexmap),
and _all_ others.
The smaller the map, the higher the performance.
It was observed that when a map contains more than 20 keys,
it may be better to use the standard
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
since the performance of `micromap::Map` _may_ start to degrade.
See the [benchmarking results](#benchmark) below.

**WELCOME**:
Not all functions that you might expect to have in a map are implemented.
I will appreciate if you contribute by implementing these
[missing functions](https://github.com/yegor256/micromap/issues).

First, add this to `Cargo.toml`:

```toml
[dependencies]
micromap = "0.0.16"
```

Then, use it like a standard hash map... well, almost:

```rust
use micromap::Map;
let mut m : Map<u64, &str, 10> = Map::new(); // allocation on stack
m.insert(1, "foo");
m.insert(2, "bar");
assert_eq!(2, m.len());
```

Pay attention, here the map is created with an extra generic argument `10`.
This is the total size of the map, which is allocated on stack when `::new()`
is called. Unlike `HashMap`, the `Map` doesn't use heap at all. If more than
ten keys will be added to the map, it will panic.

Read [the API documentation](https://docs.rs/micromap/latest/micromap/).
The struct
[`micromap::Map`](https://docs.rs/micromap/latest/micromap/struct.Map.html)
is designed to be as closely similar to
[`std::collections::HashMap`][std] as possible.

## Benchmark

There is a summary of a simple benchmark, where we compared `micromap::Map` with
a few other Rust maps, changing the total capacity of the map (horizontal axis).
We applied the same interactions
([`benchmark.rs`][rs])
to them and measured how fast they performed. In the following table,
the numbers over 1.0 indicate performance gain,
while the numbers below 1.0 demonstrate performance loss.

<!-- benchmark -->
| | 2 | 4 | 8 | 16 | 32 | 64 | 128 |
| --- | --: | --: | --: | --: | --: | --: | --: |
| `flurry::HashMap` | 287.38 | 81.25 | 41.84 | 21.80 | 12.02 | 5.38 | 2.46 |
| `hashbrown::HashMap` | 20.67 | 10.70 | 6.84 | 3.27 | 1.48 | 0.73 | 0.26 |
| `heapless::LinearMap` | 1.09 | 1.41 | 1.20 | 1.19 | 1.06 | 1.12 | 0.82 |
| `indexmap::IndexMap` | 13.29 | 12.21 | 7.60 | 5.80 | 2.13 | 1.00 | 0.43 |
| `linear_map::LinearMap` | 1.64 | 1.48 | 1.06 | 1.15 | 0.99 | 1.24 | 1.03 |
| `linked_hash_map::LinkedHashMap` | 26.39 | 19.89 | 12.17 | 6.90 | 3.38 | 1.59 | 0.69 |
| `litemap::LiteMap` | 1.65 | 2.21 | 4.37 | 3.28 | 2.22 | 1.04 | 0.52 |
| `micromap::Map` 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| `nohash_hasher::BuildNoHashHasher` | 20.54 | 11.13 | 7.66 | 2.99 | 1.52 | 0.69 | 0.32 |
| `rustc_hash::FxHashMap` | 20.42 | 10.80 | 6.85 | 3.00 | 1.27 | 0.65 | 0.28 |
| `std::collections::BTreeMap` | 25.60 | 10.39 | 5.42 | 4.45 | 2.41 | 1.19 | 0.62 |
| `std::collections::HashMap` | 20.29 | 14.11 | 8.64 | 4.87 | 2.58 | 1.15 | 0.50 |
| `tinymap::array_map::ArrayMap` | 1.68 | 4.38 | 4.54 | 4.88 | 4.29 | 4.65 | 3.89 |

The experiment [was performed][action] on 29-03-2025.
There were 1000000 repetition cycles.
The entire benchmark took 248s.
Uname: 'Linux'.

<!-- benchmark -->

As you see, the highest performance gain was achieved for the maps that
were smaller than ten keys.
For the maps of just a few keys, the gain was enormous.

## How to Contribute

First, install [Rust](https://www.rust-lang.org/tools/install) and then:

```bash
cargo test -vv
```

If everything goes well, fork repository, make changes, send us a
[pull request](https://www.yegor256.com/2014/04/15/github-guidelines.html).
We will review your changes and apply them to the `master` branch shortly,
provided they don't violate our quality standards. To avoid frustration,
before sending us your pull request please run `cargo test` again. Also,
run `cargo fmt` and `cargo clippy`.

Also, before you start making changes, run benchmarks:

```bash
rustup run nightly cargo bench
```

Then, after the changes you make, run it again. Compare the results.
If your changes
degrade performance, think twice before submitting a pull request.

[std]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[rs]: https://github.com/yegor256/micromap/blob/master/tests/benchmark.rs
[action]: https://github.com/yegor256/micromap/actions/workflows/benchmark.yml
