use line_numbers::NewlinePositions;

fn main() {
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
}
