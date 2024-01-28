use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    for filename in config.files {
        match open(&filename){
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                if config.number_lines {
                    for (index, line) in file.lines().enumerate() {
                        let line = line?;
                        println!("{:>6}\t{}", index + 1, line);
                    }
                } else if config.number_nonblank_lines {
                    let mut count = 0;
                    for line in file.lines() {
                        let line = line?;
                        if line.trim().is_empty() {
                            println!();
                        } else {
                            count = count + 1;
                            println!("{:>6}\t{}",  count, line);
                        }
                    }
                } else {
                    for line in file.lines() {
                        let line = line?;
                        println!("{}", line);
                    }
                }

            },
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("1.0.0")
        .author("hisanari")
        .about("rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input files")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("print number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("nonblank_number")
                .help("print number non blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}