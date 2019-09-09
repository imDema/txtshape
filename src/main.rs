extern crate clap;

use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

struct Conf {
    ascii: String,
    code: String,
    output: String,
}
impl Conf {
    fn new(a: &str, c: &str, o: &str) -> Conf {
        Conf {
            ascii: String::from(a),
            code: String::from(c),
            output: String::from(o),
        }
    }
}

fn main() {
    let matches = App::new("Asciicode")
        .about("Create shit ascii code")
        .arg(
            Arg::with_name("ascii")
                .takes_value(true)
                .short("a")
                .long("ascii")
                .required(true)
                .help("Ascii art (use whitespace for areas with no code)"),
        )
        .arg(
            Arg::with_name("code")
                .takes_value(true)
                .short("c")
                .long("code")
                .required(true)
                .help("Code source (remove compiler directives and paste them manually)"),
        )
        .arg(
            Arg::with_name("output")
                .required(false)
                .default_value("asciicode.c"),
        )
        .get_matches();
    let ascii = matches
        .value_of("ascii")
        .unwrap();
    let code = matches
        .value_of("code")
        .unwrap();
    let output = matches
        .value_of("output")
        .unwrap();

    match run(Conf::new(ascii, code, output)) {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    };
}

fn run(cfg: Conf) -> Result<(), Box<dyn Error>> {
    let mut asc = File::open(cfg.ascii)?;
    let mut cod = File::open(cfg.code)?;

    let mut out = File::create(cfg.output)?;

    let mut code = String::new();
    cod.read_to_string(&mut code)?;
    let mut codewords = code.split_whitespace().peekable();

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
