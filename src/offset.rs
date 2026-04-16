use std::fmt;
use std::str::FromStr;

use anyhow::bail;

/// Direction of the offset relative to the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Offset from the beginning of the file (`+` prefix).
    FromBeginning,
    /// Offset from the end of the file (`-` prefix or no prefix).
    FromEnd,
}

/// A parsed and validated offset for `-n` (lines) or `-c` (bytes) arguments.
///
/// Represents the BSD tail(1) argument format: `[+|-][number][suffix]`
/// where suffix is one of `b` (512), `k` (1024), or `m` (1048576).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedOffset {
    /// Whether to count from the beginning or end of the file.
    pub direction: Direction,
    /// The computed count (after suffix multiplication).
    pub count: usize,
}

impl ParsedOffset {
    /// Creates a new `ParsedOffset` with the given direction and count.
    pub fn new(direction: Direction, count: usize) -> Self {
        Self { direction, count }
    }
}

impl fmt::Display for ParsedOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = match self.direction {
            Direction::FromBeginning => "+",
            Direction::FromEnd => "",
        };
        write!(f, "{}{}", sign, self.count)
    }
}

impl FromStr for ParsedOffset {
    type Err = anyhow::Error;

    /// Parses a BSD tail(1) offset string in the format `[+|-][number][suffix]`.
    ///
    /// # Suffixes
    ///
    /// | Suffix | Multiplier |
    /// |--------|------------|
    /// | `b`    | 512        |
    /// | `k`    | 1,024      |
    /// | `m`    | 1,048,576  |
    ///
    /// # Errors
    ///
    /// Returns an error with message `"illegal offset -- <input>"` for invalid input.
    fn from_str(s: &str) -> anyhow::Result<Self> {
        if s.is_empty() {
            bail!("illegal offset -- {}", s);
        }

        let mut chars = s.chars().peekable();

        // Determine direction from the first character.
        let direction = match chars.peek() {
            Some('+') => {
                chars.next();
                Direction::FromBeginning
            }
            Some('-') => {
                chars.next();
                Direction::FromEnd
            }
            Some(c) if c.is_ascii_digit() => Direction::FromEnd,
            _ => bail!("illegal offset -- {}", s),
        };

        // Collect the remaining characters.
        let rest: String = chars.collect();
        if rest.is_empty() {
            bail!("illegal offset -- {}", s);
        }

        // Split trailing suffix from digit portion.
        let (digits, suffix) = match rest.as_bytes().last() {
            Some(b'b') | Some(b'k') | Some(b'm') => {
                let (d, sfx) = rest.split_at(rest.len() - 1);
                (d, sfx.as_bytes().first().copied())
            }
            _ => (rest.as_str(), None),
        };

        // Digits must be non-empty and all ASCII digits.
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            bail!("illegal offset -- {}", s);
        }

        // Parse the numeric part.
        let base: usize = digits
            .parse()
            .map_err(|_| anyhow::anyhow!("illegal offset -- {}", s))?;

        // Apply suffix multiplier.
        let multiplier: usize = match suffix {
            Some(b'b') => 512,
            Some(b'k') => 1024,
            Some(b'm') => 1_048_576,
            None => 1,
            _ => unreachable!(),
        };

        let count = base
            .checked_mul(multiplier)
            .ok_or_else(|| anyhow::anyhow!("illegal offset -- {}", s))?;

        Ok(ParsedOffset { direction, count })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Valid input tests ----

    #[test]
    fn parse_plain_number() {
        let offset: ParsedOffset = "10".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 10));
    }

    #[test]
    fn parse_minus_prefix() {
        let offset: ParsedOffset = "-10".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 10));
    }

    #[test]
    fn parse_plus_prefix() {
        let offset: ParsedOffset = "+10".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromBeginning, 10));
    }

    #[test]
    fn parse_zero_default() {
        let offset: ParsedOffset = "0".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 0));
    }

    #[test]
    fn parse_plus_zero() {
        let offset: ParsedOffset = "+0".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromBeginning, 0));
    }

    #[test]
    fn parse_minus_zero() {
        let offset: ParsedOffset = "-0".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 0));
    }

    #[test]
    fn parse_single_digit() {
        let offset: ParsedOffset = "1".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 1));
    }

    #[test]
    fn parse_plus_single_digit() {
        let offset: ParsedOffset = "+1".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromBeginning, 1));
    }

    #[test]
    fn parse_suffix_b() {
        let offset: ParsedOffset = "5b".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 2560));
    }

    #[test]
    fn parse_suffix_k() {
        let offset: ParsedOffset = "5k".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 5120));
    }

    #[test]
    fn parse_suffix_m() {
        let offset: ParsedOffset = "5m".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 5_242_880));
    }

    #[test]
    fn parse_plus_with_suffix_b() {
        let offset: ParsedOffset = "+3b".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromBeginning, 1536));
    }

    #[test]
    fn parse_minus_with_suffix_k() {
        let offset: ParsedOffset = "-2k".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 2048));
    }

    #[test]
    fn parse_suffix_boundary_b() {
        let offset: ParsedOffset = "1b".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 512));
    }

    #[test]
    fn parse_suffix_boundary_k() {
        let offset: ParsedOffset = "1k".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 1024));
    }

    #[test]
    fn parse_suffix_boundary_m() {
        let offset: ParsedOffset = "1m".parse().unwrap();
        assert_eq!(offset, ParsedOffset::new(Direction::FromEnd, 1_048_576));
    }

    // ---- Invalid input tests ----

    fn assert_parse_error(input: &str) {
        let result = input.parse::<ParsedOffset>();
        assert!(result.is_err(), "expected error for input: {:?}", input);
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("illegal offset"),
            "error for {:?} should contain 'illegal offset', got: {}",
            input,
            err
        );
    }

    #[test]
    fn reject_empty_string() {
        assert_parse_error("");
    }

    #[test]
    fn reject_non_numeric() {
        assert_parse_error("test");
    }

    #[test]
    fn reject_double_plus() {
        assert_parse_error("++5");
    }

    #[test]
    fn reject_double_minus() {
        assert_parse_error("--5");
    }

    #[test]
    fn reject_mixed_signs() {
        assert_parse_error("+-5");
    }

    #[test]
    fn reject_invalid_multi_char_suffix() {
        assert_parse_error("12abc");
    }

    #[test]
    fn reject_invalid_suffix_char() {
        assert_parse_error("12x");
    }

    #[test]
    fn reject_plus_only() {
        assert_parse_error("+");
    }

    #[test]
    fn reject_minus_only() {
        assert_parse_error("-");
    }

    #[test]
    fn reject_suffix_without_digits() {
        assert_parse_error("+b");
    }

    #[test]
    fn reject_decimal() {
        assert_parse_error("3.14");
    }

    #[test]
    fn reject_space_in_middle() {
        assert_parse_error("1 0");
    }

    // ---- Display tests ----

    #[test]
    fn display_from_end() {
        let offset = ParsedOffset::new(Direction::FromEnd, 10);
        assert_eq!(offset.to_string(), "10");
    }

    #[test]
    fn display_from_beginning() {
        let offset = ParsedOffset::new(Direction::FromBeginning, 10);
        assert_eq!(offset.to_string(), "+10");
    }
}
