Rust WarGame
============

This is the follow up to the long developed original [WarGame written in Java](https://github.com/ryanmr/wargame). This version of the *WarGame* uses [Rust](http://www.rust-lang.org/), an up and coming systems style programming language, akin to C and C++, but with a lot more on the table that focuses on modern program development.

The WarGame is a tiny kinda-sorta benchmark - and as such, it should not be taken too seriously.

Changelog
---------

There is a changelog detailing major changes between releases. See [changelog.md](changelog.md).

Legend
------

You definitely should read the [legend](https://github.com/WarGameBenchmarks/wargame/blob/master/legend.md) to learn about the output.

How To
------

With the executable directly:

```
./wargame [number of threads]
```

If the number of threads are not specified, the default is a single thread. No other configurations are available.

You should use [Cargo](https://crates.io/) to compile the WarGame in Rust if you do not have a binary executable already.

```
cargo build --release
```

```
cargo run --release [number of threads]
```

This will provide the best possible results.

Sample Output
------

```
ryan@server2:~/Code/wargame-rust$ cargo run --release 4
     Running `target/release/wargame-rust 4`
WarGame Rust
settings: threads = 4; multiplier = 1.00

4. done                                                                 
---

Samples:      8368
Mean:     43.89474
Median:   43.92427
S.D.:      0.12458
C.O.V.:    0.00284
---
Min-Max:         <  43.39707 -  44.02767 > Δ   0.63059
1-σ:             <  43.77016 -  44.01932 > Δ   0.24917
μ-Median:        <  43.89474 -  43.92427 > Δ   0.02953
99.9%% CI:       <  43.89026 -  43.89922 > Δ   0.00896
---
Threads: 4
Multiplier: 1.00
Speed: 43.87283 g/ms
Games: 2635292
Duration: 60.1s
---
Rank: (4/5) A
Rank Criteria: 1 | 2 | 4 | 3
---
Score: 44
```
