Rust WarGame
============

This is the follow up to the long developed original [WarGame written in Java](https://github.com/ryanmr/wargame). This version of the *WarGame* uses [Rust](http://www.rust-lang.org/), an up and coming systems style programming language, akin to C and C++, but with a lot more on the table that focuses on modern program development.

The WarGame is a tiny kinda-sorta benchmark - and as such, it should not be taken too seriously.

Changelog
---------

There is a changelog detailing major changes between releases. See [changelog.md](changelog.md).

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
ryan@argon ~/Desktop/wargame-rust/wargame-rust
$ cargo run --release 8
     Running `target\release\wargame.exe 8`

settings: tasks = 8

1. prime time has started
1. et = 59s; g = 6417097; s = 106.95935 g/ms;
1. prime time has has ended

2. stability testing has started
2. et = 80s; g = 8576345; s = 107.07182 g/ms; t = 20; v = 99.72%;
2. stability testing has ended

3. 8 tasks stopped

---

Samples: 13114 collected
Mean: 106.88683
Standard Deviation: 1.06521
Coefficient of Variation: 0.99657
Coefficient of Variation: 100.34%
Maximum Speed: 107.86164

---

Threads: 8
Speed: 107.07158
Total Games: 8576911
Elapsed Time: 80104456039 nanoseconds; 80 seconds

Score: 107
```
