//! Efficiently find line numbers and line spans within a string.
//!
//! ```rust
//! use line_numbers::LinePositions;
//!
//! let s = "foo\nbar\nbaz\n";
//! let s_lines: Vec<_> = s.lines().collect();
//!
//! let line_positions = LinePositions::from(s);
//!
//! let offset = 5;
//! let line_num = line_positions.from_offset(offset);
//! println!(
//!     "Offset {} is on line {}, which has the text {:?}.",
//!     offset,
//!     line_num.display(),
//!     s_lines[line_num.as_usize()]
//! );
//! ```

// The `from_offset*` methods on NewlinePositions are sensible names,
// and the docs clippy cites:
// https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv
// don't actually have an opinion on `from_foo` names.
#![allow(clippy::wrong_self_convention)]

use std::cmp::Ordering;
use std::fmt;

/// A distinct number type for line numbers, to prevent confusion with
/// other numerical data.
///
/// Zero-indexed internally.
///
/// We use a 32-bit integer, so a file cannot have more than 4 billion
/// lines. This keeps the size of the struct small. It's common to
/// have a lot of `LineNumber`s when analysing large files, so the
/// struct size is more important than handling crazy big files.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineNumber(pub u32);

impl LineNumber {
    pub fn display(self) -> String {
        format!("{}", self.0 + 1)
    }

    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Debug for LineNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LineNumber: {} (zero-indexed: {})",
            self.display(),
            self.0
        )
    }
}

impl From<u32> for LineNumber {
    fn from(number: u32) -> Self {
        Self(number)
    }
}

/// A range within a single line of a string.
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct SingleLineSpan {
    pub line: LineNumber,
    /// Start column.
    pub start_col: u32,
    /// End column.
    pub end_col: u32,
}

/// A struct for efficiently converting absolute string positions to
/// line-relative positions.
#[derive(Debug)]
pub struct LinePositions {
    /// A vector of the start and end positions (in bytes) of all the
    /// lines in a string. Positions include the newline character
    /// itself.
    positions: Vec<(usize, usize)>,
}

impl From<&str> for LinePositions {
    fn from(s: &str) -> Self {
        let mut line_start = 0;
        let mut positions = vec![];
        for line in s.split('\n') {
            let line_end = line_start + line.len() + "\n".len();
            // TODO: this assumes lines terminate with \n, not \r\n.
            positions.push((line_start, line_end - 1));
            line_start = line_end;
        }

        LinePositions { positions }
    }
}

impl LinePositions {
    /// Return the line number containing this `offset`, measured in
    /// bytes.
    ///
    /// # Panics
    ///
    /// Panics if `offset` is out of bounds.
    pub fn from_offset(&self, offset: usize) -> LineNumber {
        if let Some((_, s_end)) = self.positions.last() {
            assert!(
                offset <= *s_end,
                "Offset {} is out of bounds for a string of length {}",
                offset,
                s_end
            );
        }

        let idx = self.positions.binary_search_by(|(line_start, line_end)| {
            if *line_end < offset {
                return Ordering::Less;
            }
            if *line_start > offset {
                return Ordering::Greater;
            }

            Ordering::Equal
        });

        LineNumber::from(idx.expect("line should be present") as u32)
    }

    /// Convert this region into line spans. If the region includes a
    /// newline, the vec will contain multiple items.
    ///
    /// # Panics
    ///
    /// Panics if `region_start` or `region_end` are out of bounds, or
    /// if `region_start` is greater than `region_end`.
    pub fn from_region(&self, region_start: usize, region_end: usize) -> Vec<SingleLineSpan> {
        assert!(region_start <= region_end);

        let first_idx = self.from_offset(region_start);
        let last_idx = self.from_offset(region_end);

        let mut res = vec![];
        for idx in first_idx.0..=last_idx.0 {
            let (line_start, line_end) = self.positions[idx as usize];
            res.push(SingleLineSpan {
                line: idx.into(),
                start_col: region_start.saturating_sub(line_start) as u32,
                end_col: if region_end < line_end {
                    region_end - line_start
                } else {
                    line_end - line_start
                } as u32,
            });
        }

        res
    }

    /// Given a region in the current LinePositions, convert it to be
    /// relative to a `start` offset in a larger, enclosing string.
    ///
    /// # Panics
    ///
    /// Panics if `region_start` or `region_end` are out of bounds, or
    /// if `region_start` is greater than `region_end`.
    pub fn from_region_relative_to(
        &self,
        start: SingleLineSpan,
        region_start: usize,
        region_end: usize,
    ) -> Vec<SingleLineSpan> {
        assert!(region_start <= region_end);

        let mut res = vec![];
        for pos in self.from_region(region_start, region_end) {
            if pos.line.0 == 0 {
                res.push(SingleLineSpan {
                    line: (pos.line.0 + start.line.0).into(),
                    // On the first line of the inner string, the
                    // inner column offset may not match the column
                    // offset of the enclosing string.
                    start_col: pos.start_col + start.start_col,
                    end_col: pos.end_col + start.start_col,
                });
            } else {
                res.push(SingleLineSpan {
                    line: (pos.line.0 + start.line.0).into(),
                    // On later lines in the inner string, since we've
                    // seen a newline, we know the column offsets are
                    // the same as the enclosing string.
                    start_col: pos.start_col,
                    end_col: pos.end_col,
                });
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_one_indexed() {
        let ln = LineNumber(0);
        assert_eq!(ln.display(), "1");
    }

    #[test]
    fn from_region_first_line() {
        let newline_positions: LinePositions = "foo".into();
        let line_spans = newline_positions.from_region(1, 3);
        assert_eq!(
            line_spans,
            vec![SingleLineSpan {
                line: 0.into(),
                start_col: 1,
                end_col: 3
            }]
        );
    }

    #[test]
    fn from_region_first_char() {
        let newline_positions: LinePositions = "foo".into();
        let line_spans = newline_positions.from_region(0, 0);
        assert_eq!(
            line_spans,
            vec![SingleLineSpan {
                line: 0.into(),
                start_col: 0,
                end_col: 0
            }]
        );
    }

    #[test]
    fn from_region_split_over_multiple_lines() {
        let newline_positions: LinePositions = "foo\nbar\nbaz\naaaaaaaaaaa".into();
        let line_spans = newline_positions.from_region(5, 10);

        assert_eq!(
            line_spans,
            vec![
                SingleLineSpan {
                    line: 1.into(),
                    start_col: 1,
                    end_col: 3
                },
                SingleLineSpan {
                    line: 2.into(),
                    start_col: 0,
                    end_col: 2
                }
            ]
        );
    }

    #[test]
    fn from_region_relative_to() {
        let newline_positions: LinePositions = "foo\nbar".into();

        let pos = SingleLineSpan {
            line: 100.into(),
            start_col: 1,
            end_col: 1,
        };

        let line_spans = newline_positions.from_region_relative_to(pos, 1, 7);
        assert_eq!(
            line_spans,
            vec![
                SingleLineSpan {
                    line: 100.into(),
                    start_col: 2,
                    end_col: 4
                },
                SingleLineSpan {
                    line: 101.into(),
                    start_col: 0,
                    end_col: 3
                }
            ]
        );
    }

    #[test]
    #[should_panic(expected = "out of bounds for a string")]
    fn test_from_offset_out_of_bounds() {
        let newline_positions: LinePositions = "foo".into();
        let _ = newline_positions.from_offset(4);
    }
}
