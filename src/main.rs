extern crate clap;

use clap::{App, Arg};
use txtshape::{run, Conf};

fn main() {
    let matches = App::new("txtshape")
        .about("Shape text into ascii art")
        .arg(
            Arg::with_name("shape")
                .takes_value(true)
                .short("s")
                .long("shape")
                .required(true)
                .help("The shape you want to fit the text in"),
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
                .default_value("txtshape.out"),
        )
        .get_matches();
    let ascii = matches.value_of("shape").unwrap();
    let text = matches.value_of("text").unwrap();
    let output = matches.value_of("output").unwrap();

    match run(Conf::new(ascii, text, output)) {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    };
}
