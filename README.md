# line-numbers <a href="https://crates.io/crates/line-numbers"><img src="https://img.shields.io/crates/v/line-numbers.svg?style=flat-square" alt="crates.io"></a> <a href="https://codecov.io/gh/Wilfred/line-numbers"><img src="https://img.shields.io/codecov/c/github/Wilfred/line-numbers?style=flat-square&token=jdOv9Fo8rG" alt="codecov.io"></a> <a href="https://docs.rs/line-numbers/latest/line_numbers/"><img alt="docs.rs" src="https://img.shields.io/docsrs/line-numbers?style=flat-square"></a>

line-numbers is a Rust crate for efficiently finding the line number
of a string offset.

## Usage

Create a `LinePositions`, then you can find line numbers for an
offset.

```rust
let s = "foo\nbar\nbaz\n";
let s_lines: Vec<_> = s.lines().collect();

let line_positions = LinePositions::from(s);

let offset = 5;
let (line_num, column) = line_positions.from_offset(offset);

println!(
    "Offset {} is on line {} (column {}), and the text of that line is {:?}.",
    offset,
    line_num.display(),
    column,
    s_lines[line_num.as_usize()]
);
```

## Similar Projects

* [line-span](https://crates.io/crates/line-span) solves a similar
  problem, but scans the whole string every time.
