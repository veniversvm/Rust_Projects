use clap::{App, Arg};

fn main() {
    //    println!("Hello, world!");

    let matches = App::new("echor")
        .version("0.1.0")
        .author("Fran!")
        .about("Rcust echo command")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print new line")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{ending}", text.join(" "));
}
