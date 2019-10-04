extern crate clap;

use clap::{App, Arg};
use asciicode::{Conf,run};

fn main() {
    let matches = App::new("txtshape")
        .about("Create ascii text art")
        .arg(
            Arg::with_name("ascii")
                .takes_value(true)
                .short("a")
                .long("ascii")
                .required(true)
                .help("Ascii art (be sure whitespace is used where nothing should be written)"),
        )
        .arg(
            Arg::with_name("text")
                .takes_value(true)
                .short("text")
                .long("text")
                .required(true)
                .help("Text content"),
        )
        .arg(
            Arg::with_name("output")
                .required(false)
                .default_value("asciicode.out"),
        )
        .get_matches();
    let ascii = matches
        .value_of("ascii")
        .unwrap();
    let text = matches
        .value_of("text")
        .unwrap();
    let output = matches
        .value_of("output")
        .unwrap();

    match run(Conf::new(ascii, text, output)) {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    };
}
