# 0.3.1 (unreleased)

# 0.3.0 (released 19th November 2023)

Breaking change: Renamed `from_offsets` to `from_region`, and
`from_offsets_relative_to` to `from_region_relative_to`.

# 0.2.2 (released 26th August 2023)

Documented panic behaviour and improved panic messages.

# 0.2.1 (released 6th August 2023)

Fixed explanations in the README.

# 0.2.0 (released 6th August 2023)

Replaced `LineNumber::one_indexed()` with `LineNumber::display()`, as
one-indexed lines are only really for human consumption.

Added function `NewlinePositions::from_offset()`.

Renamed `NewlinePositions` to `LinePositions`.

# 0.1.0 (released 5th August 2023)

Initial release.
