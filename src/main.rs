use std::io::{self, BufRead, BufReader, Write, stdin, stdout};

fn main() -> io::Result<()> {
    let stdin = stdin().lock();
    let mut stdin = BufReader::new(stdin);
    let mut stdout = stdout().lock();

    remove_whitespace(&mut stdin, &mut stdout)?;

    Ok(())
}

fn remove_whitespace(read: &mut impl BufRead, write: &mut impl Write) -> io::Result<()> {
    let mut indentation = None;

    let mut iter = read.lines().peekable();
    while let Some(line) = iter.next() {
        let line = line?;

        let line_indentation = detect_indentation(&line);
        let indentation = *indentation.get_or_insert(line_indentation);

        let delta = line_indentation.saturating_sub(indentation);

        for c in std::iter::repeat_n(' ', delta).chain(line.chars().skip(line_indentation)) {
            write!(write, "{c}")?;
        }

        if iter.peek().is_some() {
            writeln!(write)?;
        }
    }

    write.flush()
}

fn detect_indentation(line: &str) -> usize {
    line.chars().take_while(|c| c.is_ascii_whitespace()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_detect_indentation() {
        assert_eq!(0, detect_indentation("foo"));
        assert_eq!(1, detect_indentation(" foo"));
        assert_eq!(2, detect_indentation("  foo"));
    }

    macro_rules! test_remove_whitespace {
        ($ident:ident, $input:expr, $output:expr $(,)?) => {
            #[test]
            fn $ident() {
                let mut read = BufReader::new(io::Cursor::new($input));
                let mut write = Vec::<u8>::new();

                remove_whitespace(&mut read, &mut write).unwrap();

                let result = String::from_utf8(write).unwrap();
                print!("{result}");
                assert_eq!(result, $output);
            }
        };
    }

    test_remove_whitespace!(remove_whitespace_no_indentation_basic, "foo", "foo");

    test_remove_whitespace!(
        remove_whitespace_no_indentation_two_lines,
        "foo\nbar",
        "foo\nbar"
    );

    test_remove_whitespace!(remove_whitespace_indentation_one_line_basic, "  foo", "foo");

    test_remove_whitespace!(
        remove_whitespace_indentation_two_lines_basic,
        "  foo\n  bar",
        "foo\nbar"
    );

    #[rustfmt::skip]
    test_remove_whitespace!(
        remove_whitespace_indentation_two_lines_different_indentation,
        Vec::from([
            "  def foo():",
            "    print(a)",
        ]).join("\n"),
        Vec::from([
            "def foo():",
            "  print(a)",
        ]).join("\n"),
    );
}
