use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::iter::{Iterator, Peekable};

/// Configuration parameters
/// + `shape`: shape source
/// + `text`: word source
/// + `output`: output file path
pub struct Conf {
    shape: String,
    text: String,
    output: String,
}
impl Conf {
    pub fn new(shape: &str, text: &str, output: &str) -> Conf {
        Conf {
            shape: String::from(shape),
            text: String::from(text),
            output: String::from(output),
        }
    }
}

pub fn run(cfg: Conf) -> Result<(), Box<dyn Error>> {
    // Read input files
    let mut text = String::new();
    File::open(cfg.text)?.read_to_string(&mut text)?;

    let mut shape = String::new();
    File::open(cfg.shape)?.read_to_string(&mut shape)?;

    // Split text into words
    let mut words = text.split_whitespace().peekable();

    let mut output = String::new();
    let mut canwrite = 0;

    for c in shape.chars() {
        if words.peek().is_none() {
            // Finished writing all words
            break;
        } else if c.is_whitespace() {
            if canwrite > 0 {
                // Write as many characters as the len of the last sequence of non-whitespace characters
                output.push_str(&justify_in(&mut words, canwrite));
                canwrite = 0;
            }
            output.push(match c {
                '\n' => '\n',
                _ => ' ',
            });
        } else {
            // Count len of the sequence of non-whitespace characters
            canwrite += 1;
        }
    }

    // TODO: Decide whether the program should repeat / resize / truncate text
    if words.next().is_some() {
        eprintln!("Error, ascii art has insufficient characters!\nTry using a larger one (or duplicate it)\n{} words couldn'be be written", words.count() + 1);
    };

    File::create(cfg.output)?.write_all(output.as_bytes())?;

    Ok(())
}

/// Places as many words from `words` as possible within a `n` characters long String
/// The words will be separated by at least one `' '`
/// and if at least 2 words fit they will be aligned to both the `0` and `n-1` position
/// with each word being spaced equally +- 1
fn justify_in(words: &mut Peekable<std::str::SplitWhitespace>, n: usize) -> String {
    let mut line = Vec::new();

    let mut remaining = n;
    let mut spaces = 0;

    // Put as many words as possible in line and count the necessary spaces
    while words.peek().is_some() && words.peek().unwrap().len() < remaining - spaces {
        let w = words.next().unwrap();
        remaining -= w.len();
        spaces += 1;
        line.push(w)
    }

    // Make remaining and spaces immutable
    let remaining = remaining;
    let spaces = if spaces > 0 { spaces - 1 } else { 0 }; // We don't put a space after the last word

    if line.len() > 1 {
        // With 2 or more words we can justify
        let div = remaining / spaces;
        let rem = remaining % spaces;

        line.into_iter()
            .enumerate()
            .fold(String::with_capacity(n), |mut s, (i, w)| {
                if i > 0 {
                    s.push_str(&space_string(div + if i <= rem { 1 } else { 0 }));
                }
                s.push_str(w);
                s
            })
    } else {
        // With 0 or 1 words we pad with spaces
        let mut out = line.get(0).unwrap_or(&"").to_string();
        out.push_str(&space_string(n - out.len()));
        out
    }
}

/// Push `x` spaces to the end of `s`
fn space_string(x: usize) -> String {
    (0..x).fold(String::with_capacity(x), |mut s, _| {
        s.push(' ');
        s
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_justified() {
        let mut iter = "testing out the justified text fitter"
            .split_whitespace()
            .peekable();
        assert_eq!(
            justify_in(&mut iter, 40),
            String::from("testing  out  the  justified text fitter")
        );

        let mut iter = "testing out the justified text fitter withareallylongwordwhoa"
            .split_whitespace()
            .peekable();
        assert_eq!(
            justify_in(&mut iter, 46),
            String::from("testing   out   the   justified   text  fitter")
        );

        let mut iter = "thisisalongword".split_whitespace().peekable();
        assert_eq!(justify_in(&mut iter, 8), String::from("        "));

        let mut iter = "thisisalongword".split_whitespace().peekable();
        assert_eq!(
            justify_in(&mut iter, 18),
            String::from("thisisalongword   ")
        );
    }
}
