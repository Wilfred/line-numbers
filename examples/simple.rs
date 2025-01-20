use line_numbers::LinePositions;

fn main() {
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
}
