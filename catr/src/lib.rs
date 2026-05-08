use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("Hello CatR!");
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("John Doe")
        .about("Rust cat")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number non-blank")
                .short("b")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: vec!["file".to_string()],
        number_lines: false,
        number_nonblank_lines: false,
    })
}
