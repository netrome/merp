extern crate clap;
extern crate merp;

use clap::{Arg, App};

use merp::Merp;

fn main() {
    let matches = App::new("MERP: Mårtens Excellent Regexmatching Product")
        .version("1.0")
        .author("Mårten Nilsson <m@martennilsson.se>")
        .about("Merp is the tool that finds what needs to be found in no time.")
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .value_name("FILE")
             .help("Pattern for matching the file name")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("query")
             .short("q")
             .long("query")
             .value_name("QUERY")
             .help("Serch pattern that the merp will look for")
             .takes_value(true)
             .required(true))
        .get_matches();
    println!("Derpaderpa");

    let merp = Merp::new()
        .files(matches.value_of("file").unwrap_or(r"\./.*").to_owned())
        .query(matches.value_of("query").unwrap_or(r".*").to_owned())
        .build();
    merp.match_files();
}
