use std::error::Error;
use std::fs::File;
use std::str::SplitWhitespace;
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

    // Iterate over shape characters writing as many words as possible in non-whitespace sequences
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

/// Places as many words from `words` as possible within a `n` characters long String filling with spaces
/// 
/// If at least 2 words fit they will
/// + be aligned to both the `0` and `n-1` position
/// + have at least one `' '` between each word
/// + have the same (+- 1) number of `' '` between each word
/// 
/// # Arguments
/// 
///  + `text_words` word source
///  + `length` length of the string to create
fn justify_in(text_words: &mut Peekable<SplitWhitespace>, length: usize) -> String {
    let (words, remaining) = fit_words(text_words, length);

    if words.len() > 1 {
        // With 2 or more words we can justify
        let spaces = words.len() - 1;

        let div = remaining / spaces;
        let rem = remaining % spaces;

        words.into_iter()
            .enumerate()
            .fold(String::with_capacity(length), |mut s, (i, w)| {
                if i > 0 {
                    s.push_str(&space_string(div + if i <= rem { 1 } else { 0 }));
                }
                s.push_str(w);
                s
            })
    } else {
        // With 0 or 1 words we pad with spaces
        let mut out = words.get(0).unwrap_or(&"").to_string();
        out.push_str(&space_string(length - out.len()));
        out
    }
}

/// Returns the first words from `text_words` that can fit in `length` characters (counting for at least one space between each)
/// and how many characters could not be filled (including the required space between words)
fn fit_words<'a>(text_words: &'a mut Peekable<SplitWhitespace>, length: usize) -> (Vec<&'a str>, usize) {
    let mut remaining = length;
    let mut spaces = 0;
    let mut words = Vec::new();
    while text_words.peek().is_some() && remaining > spaces && text_words.peek().unwrap().len() <= remaining - spaces {
        let w = text_words.next().unwrap();
        remaining -= w.len();
        spaces += 1;
        words.push(w)
    }
    
    (words, remaining)
}

/// Returns a string of spaces of `len` length
fn space_string(len: usize) -> String {
    (0..len).fold(String::with_capacity(len), |mut s, _| {
        s.push(' ');
        s
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fitting() {
        let mut iter = "test fit words 01234567123456"
            .split_whitespace()
            .peekable();
        
        assert_eq!(
            fit_words(&mut iter, 6),
            (vec!["test"], 2)
        );

        let mut iter = "test fit words 01234567123456"
            .split_whitespace()
            .peekable();
        
        assert_eq!(
            fit_words(&mut iter, 8),
            (vec!["test", "fit"], 1)
        );

        let mut iter = "test fit words 01234567123456"
            .split_whitespace()
            .peekable();
        
        assert_eq!(
            fit_words(&mut iter, 40),
            (vec!["test", "fit", "words", "01234567123456"], 40 - 26)
        );
    }

    #[test]
    fn justification() {
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
