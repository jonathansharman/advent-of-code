My solutions to [Advent of Code](https://adventofcode.com/), written in Rust.

Puzzle tests are organized into crates year, then into modules by day. Use
`cargo test` to check solutions. I would recommend running in `--release` mode
since some solutions are not super fast. I would also recommend `--nocapture` so
that the test macro can print solutions and timing info. For example, to check
the part 1 solution for day 1, 2021:

```
cd  2021
cargo test d01::test1 --release -- --nocapture
```

Leave out `test1`/`d01::test1` to run both parts/the entire year.

Note that puzzle inputs are not checked into the repo. You'll need to manually
place input files under `2020/input/01.txt`, etc. Note also that not all of my
solutions are general - some only work for my personal input file.
