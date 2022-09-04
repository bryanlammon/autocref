#[macro_use]
extern crate slog;

//use autocref::fs::{load_file, save_file};
use clap::{crate_version, App, Arg};
use slog::{Drain, Level};
use std::{path::Path, process, sync::Mutex};

fn main() {
    // Get the command-line arguments and flags
    let matches = App::new("autocref")
        .version(crate_version!())
        .author("Bryan Lammon")
        .about("A Supra + Pandoc post-processor for footnote cross-references")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .value_name("INPUT FILE")
                .help("The .docx file to process")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT FILE")
                .help("The .docx file to output (blank overwrites input)"),
        )
        .arg(
            Arg::with_name("verbose")
                .short('v')
                .long("verbose")
                .value_name("NUMBER")
                .help("Verbosity level between 0 (critical) and 3 (info)")
                .hidden_short_help(true)
                .hidden_long_help(true)
                .default_value("3"),
        )
        .get_matches();

    // Setup the logger.
    // First determine the log level.
    let min_log_level = match matches.value_of("verbose").unwrap() {
        "0" => Level::Critical,
        "1" => Level::Error,
        "2" => Level::Warning,
        "3" => Level::Info,
        "4" => Level::Debug,
        "5" => Level::Trace,
        _ => Level::Info,
    };

    // Then setup the terminal logger.
    let term_decorator = slog_term::TermDecorator::new().build();
    let term_drain = slog_term::CompactFormat::new(term_decorator).build().fuse();
    let term_drain = term_drain.filter_level(min_log_level).fuse();

    let _guard: slog_scope::GlobalLoggerGuard = {
        let term_logger = slog::Logger::root(
            Mutex::new(term_drain).fuse(),
            o!("version" => crate_version!()),
        );
        slog_scope::set_global_logger(term_logger)
    };

    debug!(slog_scope::logger(), "Logger setup.");

    // Setup configuration variables
    let input = Path::new(matches.value_of("input").unwrap());
    let output = match matches.is_present("output") {
        true => Path::new(matches.value_of("output").unwrap()),
        false => input,
    };

    match slog_scope::scope(&slog_scope::logger().new(o!("fn" => "autocref()")), || {
        autocref::autocref(input, output)
    }) {
        Ok(_) => (),
        Err(e) => {
            drop(_guard);
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
