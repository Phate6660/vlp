# .shuf

There are 2 versions, a C++ version and a Rust version.

## Rust

To build, use `./install mode` where `mode` is either `debug` or `release`.<br>
Binary will be at `target/mode/.shuf`.

## C++

To build, use either:
- `g++ -c -std=c++2a src/main.cpp -o .shuf` (only requires a working C++ compiler, `g++` is used as an example)
- `xmake` (requires [xmake](https://github.com/xmake-io/xmake)).

Binary will be at `build/OS/ARCH/mode/.shuf` if using `xmake`.

## Both

Uses: 
- `.shuf` to list a random entry in the current dir
- `.shuf mpv` to play the random item in mpv

Reason for making this: Post-Anime Depression.<br>
I needed an anime to re-watch to make myself feel better, but couldn't decide on one.<br>
Therefore... make the computer decide for me.
