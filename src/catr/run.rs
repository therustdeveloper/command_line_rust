use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use crate::{Config, MyResult};

pub fn master(show: bool, config: &Config) -> MyResult<()> {
    if show {
        for filename in &config.files {
            match open(&filename) {
                Err(err) => eprintln!("Failed to open {}: {}", filename, err),
                Ok(file) => {
                    let mut last_num = 0;
                    for (line_num, line) in file.lines().enumerate() {
                        let line = line?;
                        if config.number_lines {
                            println!("{:>6}\t{}", line_num + 1, line);
                        } else if config.number_nonblank_lines {
                            if !line.is_empty() {
                                last_num += 1;
                                println!("{:>6}\t{}", last_num, line);
                            }
                        } else {
                            println!("{}", line);
                        }
                    }
                },
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}