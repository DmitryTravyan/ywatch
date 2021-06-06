use clap::{ArgMatches, App, Arg};

pub(crate) fn collect_args() -> ArgMatches<'static> {
    App::new("ywatch")
        .version(crate::VERSION)
        .args(&[
            Arg::with_name("configuration")
                .env("YWATCH_CONFIG")
                .help("path to ywatch configuration file")
                .short("c")
        ])
        .get_matches()
}
