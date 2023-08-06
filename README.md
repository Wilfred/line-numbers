# line-numbers

line-numbers is a Rust crate for efficiently finding the line number
of a string offset.

## Usage

Create a `NewlinePositions`, then you can find line numbers for an
offset.

```rust
let s = "foo\nbar\nbaz\n";
let s_lines: Vec<_> = s.lines().collect();

let newlines = NewlinePositions::from(s);

let offset = 5;
let line_num = newlines.from_offset(offset);
println!(
    "Offset {} is on line {}, which has text {:?}",
    offset,
    line_num.display(),
    s_lines[line_num.as_usize()]
);
```

## Similar Projects

* [line-span](https://crates.io/crates/line-span) solves a similar
  problem, but scans the whole string every time.
