# line-numbers <a href="https://crates.io/crates/line-numbers"><img src="https://img.shields.io/crates/v/line-numbers.svg?style=flat-square" alt="crates.io"></a> <a href="https://codecov.io/gh/Wilfred/line-numbers"><img src="https://img.shields.io/codecov/c/github/Wilfred/line-numbers?style=flat-square&token=jdOv9Fo8rG" alt="codecov.io"></a>

line-numbers is a Rust crate for efficiently finding the line number
of a string offset.

## Usage

Create a `NewlinePositions`, then you can find line numbers for an
offset.

```rust
let s = "foo\nbar\nbaz\n";
let s_lines: Vec<_> = s.lines().collect();

let line_positions = LinePositions::from(s);

let offset = 5;
let line_num = line_positions.from_offset(offset);
println!(
    "Offset {} is on line {}, which has the text {:?}.",
    offset,
    line_num.display(),
    s_lines[line_num.as_usize()]
);
```

## Similar Projects

* [line-span](https://crates.io/crates/line-span) solves a similar
  problem, but scans the whole string every time.
