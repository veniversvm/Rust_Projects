use clap::{App, Arg};
use std::{default, fs};
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
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Select how many lines")
                .required(false)
                .takes_value(true), //.conflicts_with("bytes"),
                                    //.default_value(10 as usize),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("Select how many bytes")
                .required(false)
                .takes_value(true)
                .conflicts_with("lines"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input text")
                .required(true)
                .min_values(1)
                .multiple(true)
                .default_value("-"),
        )
        .get_matches();

    let lines = match matches.value_of("lines") {
        Some(v) => parse_positive_int(v).map_err(|e| format!("illegal line count -- {}", e))?,
        None => 10, //Err(e),
    };

    let bytes_value = match matches.value_of("bytes") {
        Some(v) => parse_positive_int(v).map_err(|e| format!("illegal byte count -- {}", e))?,
        None => 0,
    };

    let bytes = if bytes_value > 0 {
        Some(bytes_value)
    } else {
        None
    };

    let file = matches.values_of_lossy("files").unwrap();
    println!("{:?}", file);

    get_folder_content(file[0].clone());

    Ok(Config {
        files: vec!["-".to_string()],
        lines,
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    // println!("Parse {}", val);
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

fn get_folder_content(file: String) {
    //file.insert_str(0, "./");
    println!("searchin in folder: {}", file);
    let paths = fs::read_dir(file).unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
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

#[test]
fn test_content_files() {}
