# Advent Of Code 2024

https://adventofcode.com/2024 solutions in Rust.

Retrieve your daily input using your session cookie via:

```sh
cargo run --release -- --day <day> download --session <session>
```

The data is put in data/ and used directly at compile time.

To compute the  execution time, use:

```sh
cargo run --release -- --day <day> execute --part <part>
```

To measure execution time for a particular day, use:

```sh
cargo run --release -- --day <day> benchmark --number <number> --current
```

## [Day 01: Historian Hysteria](https://adventofcode.com/2024/day/1)

basic line parsing + minimization

[Code](./src/solutions/day01.rs)

## [Day 02: Red-Nosed Reports](https://adventofcode.com/2024/day/2)

basic line parsing

[Code](./src/solutions/day02.rs)

## [Day 03: Mull It Over](https://adventofcode.com/2024/day/3)

parsing with subtlety

[Code](./src/solutions/day03.rs)

## [Day 04: Ceres Search](https://adventofcode.com/2024/day/4)

matrix and indices

[Code](./src/solutions/day04.rs)

## [Day 05: Print Queue](https://adventofcode.com/2024/day/5)

ordering.
part 2 assumes that a solution exists… and can be stopped earlier.

[Code](./src/solutions/day05.rs)

## [Day 06: Guard Gallivant](https://adventofcode.com/2024/day/6)

path…
part 2 causality is implied… which wasn't clear at all (ie. no obstacle on previous positions)

[Code](./src/solutions/day06.rs)

## [Day 07: Bridge Repair](https://adventofcode.com/2024/day/7)

basic parsing and combinations.

[Code](./src/solutions/day07.rs)

## [Day 08: Resonant Collinearity](https://adventofcode.com/2024/day/8)

antinode resonance definition were totally unclear as usual.

[Code](./src/solutions/day08.rs)




## [Day 15: Warehouse Woes](https://adventofcode.com/2024/day/15)

moving block and detect feasability.

[Code](./src/solutions/day15.rs)
