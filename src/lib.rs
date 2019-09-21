use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

pub struct Conf {
    ascii: String,
    code: String,
    output: String,
}
impl Conf {
    pub fn new(a: &str, c: &str, o: &str) -> Conf {
        Conf {
            ascii: String::from(a),
            code: String::from(c),
            output: String::from(o),
        }
    }
}

pub fn run(cfg: Conf) -> Result<(), Box<dyn Error>> {
    let mut asc = File::open(cfg.ascii)?;
    let mut cod = File::open(cfg.code)?;

    let mut out = File::create(cfg.output)?;

    let mut code = String::new();
    cod.read_to_string(&mut code)?;
    let mut codewords = code
        .split_whitespace()
        .peekable();

    let mut ascii = String::new();
    asc.read_to_string(&mut ascii)?;

    let mut output = String::new();
    let mut canwrite = 0;
    for c in ascii.chars() {
        match c.is_whitespace() {
            true => {
                for _ in 0..canwrite {
                    output.push(' ');
                }
                canwrite = 0;
                output.push(match c {
                    '\n' => '\n',
                    _ => ' ',
                });
            }
            false => {
                canwrite += 1;
                match codewords.peek() {
                    Some(s) => {
                        let l = s.len();
                        if l < canwrite {
                            output.push_str(s);
                            output.push(' ');
                            canwrite -= l + 1;
                            codewords.next();
                        }
                    }
                    None => break,
                }
            }
        }
    }
    match codewords.next() {
        None => out.write_all(output.as_bytes())?,
        Some(_) => eprintln!("Error, ascii art has insufficient characters!\nTry using a larger one (or duplicate it)\n{} words couldn'be be written", codewords.count()),
    };

    Ok(())
}
