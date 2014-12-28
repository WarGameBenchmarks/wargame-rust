Rust WarGame
============

This is the follow up to the long developed original [WarGame written in Java](https://github.com/ryanmr/wargame). This version of the *WarGame* uses [Rust](http://www.rust-lang.org/), an up and coming systems style programming language, akin to C and C++, but with a lot more on the table that focuses on modern program development.

The WarGame is a tiny kinda-sorta benchmark - and as such, it should not be taken too seriously.

How To
------


```
./wargame [number of threads]
```
If the number of threads are not specified, the default is a single thread. No other configurations are available at this time.

If you download this repository and have *cargo* installed, to compile and run this:

```
cargo run
```

Sample Output
------

```
ryan@server:~/wargame-rust$ ./wargame 8
settings: tasks = 8

1. prime time has begun
1. et = 59s; g = 538191; s = 8.977047 g/ms;     
1. prime time has has ended

2. stability testing has begun
2. et = 210s; g = 1882052; s = 8.959508 g/ms; t = 15 @ 10s;     
2. stability testing has ended

3. 8 tasks stopped
```