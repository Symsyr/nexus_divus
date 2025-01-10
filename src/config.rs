use clap::{App, Arg, ArgMatches};

pub fn get_matches() -> ArgMatches {
    App::new("nexus_divus")
        .version("0.1.0")
        .author("William Bowen")
        .about("An ETL Tool")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Print debug information verbosely")
                .takes_value(false),
        )
        .get_matches()
}