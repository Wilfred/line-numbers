use line_numbers::LinePositions;

fn main() {
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
}
