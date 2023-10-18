use clap::{App, Arg};

pub fn master(show: bool) {
    if show {
        let matches = App::new("command_line_rust")
            .version("0.1.0")
            .author("William Munoz <william@rustdeveloper.io>")
            .about("Rust echo")
            .arg(
                Arg::with_name("text")
                    .value_name("TEXT")
                    .help("input text")
                    .required(true)
                    .min_values(1),
            )
            .arg(
                Arg::with_name("omit_newline")
                    .short("n")
                    .help("Do not print newline")
                    .takes_value(false),
            )
            .get_matches();

        let text = matches.values_of_lossy("text").unwrap();
        let omit_newline = matches.is_present("omit_newline");

        print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    }
}
