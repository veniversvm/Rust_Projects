use clap::{App, Arg};
use std::{error::Error, ffi::c_long};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("yo")
        .about("Rust Head")
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
                .long("--lines")
                .help("Select how many lines")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("--bytes")
                .help("Select how many bytes")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        files: vec!["file".to_string()],
        lines: 0,
        bytes: Some(0),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) => {
            if n > 0 {
                Ok(n)
            } else {
                Err(From::from(val))
            }
        }
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
