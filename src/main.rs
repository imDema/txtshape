extern crate clap;

use clap::{App, Arg};
use asciicode::{Conf,run};

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
