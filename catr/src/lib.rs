use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    //println!("Hello CatR!");
    for filename in config.files {
        //println!("{}", filename);
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(reader) => {
                if config.number_lines {
                    read_and_count(reader);
                    return Ok(());
                }
                if config.number_nonblank_lines {
                    read_skip_blanks(reader);
                    return Ok(());
                }
                only_read(reader);
            }
        };
    }

    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("John Doe")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("--number")
                .help("Prinbts the number of lines")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("--number-nonblank")
                .help("Number non-blank lines")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number-nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn only_read(reader: Box<dyn BufRead>) {
    for line in reader.lines() {
        let line = line.expect("something wrong :(");
        println!("{}", line);
    }
}

fn read_and_count(reader: Box<dyn BufRead>) {
    let mut i = 0;
    for line in reader.lines() {
        let line = line.expect("something wrong :(");
        i += 1;
        println!("{:>6}\t{}", i, line);
    }
}

fn read_skip_blanks(reader: Box<dyn BufRead>) {
    let mut i = 0;
    for line in reader.lines() {
        let line = line.expect("something wrong :(");
        //let line = line?;
        if line.len() > 0 {
            i += 1;
            println!("{:>6}\t{}", i, line);
        } else {
            println!("");
        }
    }
}
